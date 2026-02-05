use std::time::Duration;

use reqwest::Client;
use serde_json::Value;
use tauri::Url;
use tracing::{info, warn};

use crate::{error::ServiceError, models::trade::{Kline, KlineRequest}};


#[derive(Clone)]
pub struct BinanceClient {
    client: Client,
    base_url: String,
}

impl BinanceClient {
    pub fn new() -> Self {
        let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .expect("创建Http客户端失败！");
        Self {
            client,
            base_url: "https://api.binance.com".to_string(),
        }
    }

    /// 获取K线数据
    pub async fn get_klines(
        &self,
        request: &KlineRequest,
    ) -> Result<Vec<Kline>, ServiceError> {
        let KlineRequest { symbol, interval, start_time, end_time, limit } = request;
        let mut url = Url::parse(&format!("{}/api/v3/klines", self.base_url)).map_err(|e| ServiceError::UrlError(e.to_string()))?;
        url.query_pairs_mut()
            .append_pair("symbol", symbol)
            .append_pair("interval", interval);
        if let Some(start_time) = start_time {
            url.query_pairs_mut().append_pair("startTime", &start_time.to_string());
        }
        if let Some(end_time) = end_time {
            url.query_pairs_mut().append_pair("endTime", &end_time.to_string());
        }
        if let Some(limit) = limit {
            url.query_pairs_mut().append_pair("limit", &limit.to_string());
        }

        info!("请求URL: {}", url);

        let response = self.client.get(url).send().await.map_err(|e| ServiceError::BinanceError(e.to_string()))?;
        
        if !response.status().is_success() {
            return Err(ServiceError::BinanceError(format!("请求失败，状态码: {}", response.status())));
        }
        let json: Value = response.json().await.map_err(|e| ServiceError::BinanceError(e.to_string()))?;
        self.parse_klines(json)
    }

    pub fn parse_klines(&self, json: Value) -> Result<Vec<Kline>, ServiceError> {
        // let klines: Vec<Kline> = serde_json::from_value(json).map_err(|e| ServiceError::BinanceError(e.to_string()))?;
        let mut klines = Vec::new();
        if let Value::Array(arr) = json {
            for (i, item) in arr.iter().enumerate() {
                if let Value::Array(kline_arr) = item {
                    if kline_arr.len() < 11 {
                        warn!("第{}个K线数据不完整: {:?}", i, kline_arr);
                        continue;
                    }
                    let kline = Kline {
                        open_time: kline_arr[0].as_u64().ok_or_else(|| ServiceError::BinanceError("open_time 字段解析失败".to_string()))?,
                        open: kline_arr[1].as_str().ok_or_else(|| ServiceError::BinanceError("open 字段解析失败".to_string()))?.to_string(),
                        high: kline_arr[2].as_str().ok_or_else(|| ServiceError::BinanceError("high 字段解析失败".to_string()))?.to_string(),
                        low: kline_arr[3].as_str().ok_or_else(|| ServiceError::BinanceError("low 字段解析失败".to_string()))?.to_string(),
                        close: kline_arr[4].as_str().ok_or_else(|| ServiceError::BinanceError("close 字段解析失败".to_string()))?.to_string(),
                        volume: kline_arr[5].as_str().ok_or_else(|| ServiceError::BinanceError("volume 字段解析失败".to_string()))?.to_string(),
                        close_time: kline_arr[6].as_u64().ok_or_else(|| ServiceError::BinanceError("close_time 字段解析失败".to_string()))?,
                        quote_asset_volume: kline_arr[7].as_str().ok_or_else(|| ServiceError::BinanceError("quote_asset_volume 字段解析失败".to_string()))?.to_string(),
                        number_of_trades: kline_arr[8].as_u64().ok_or_else(|| ServiceError::BinanceError("number_of_trades 字段解析失败".to_string()))?,
                        taker_buy_base_asset_volume: kline_arr[9].as_str().ok_or_else(|| ServiceError::BinanceError("taker_buy_base_asset_volume 字段解析失败".to_string()))?.to_string(),
                        taker_buy_quote_asset_volume: kline_arr[10].as_str().ok_or_else(|| ServiceError::BinanceError("taker_buy_quote_asset_volume 字段解析失败".to_string()))?.to_string(),
                        ignore: kline_arr[11].as_str().ok_or_else(|| ServiceError::BinanceError("ignore 字段解析失败".to_string()))?.to_string(),
                    };
                    info!("第{}个K线数据: {:?}", i, kline);
                    klines.push(kline);
                } else {
                    warn!("第{}个K线数据不是数组: {:?}", i, item);
                }
            }
        } else {
            warn!("响应JSON不是数组: {:?}", json);
        }
        info!("解析到{}条K线数据", klines.len());
        Ok(klines)
    }
    /// 获取交易所信息
    pub async fn get_exchange_info(&self) -> Result<Value, String> {
        Ok(Value::Null)
    }
}