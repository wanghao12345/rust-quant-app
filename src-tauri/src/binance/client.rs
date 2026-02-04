use std::time::Duration;

use reqwest::Client;
use serde_json::Value;
use tauri::Url;
use tracing::info;

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
        // symbol: &str,
        // interval: &str,
        // start_time: Option<u64>,
        // end_time: Option<u64>,
        // limit: Option<u32>,
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

        info!("响应JSON: {:?}", json);

        Ok(vec![])
    }

    /// 获取交易所信息
    pub async fn get_exchange_info(&self) -> Result<Value, String> {
        Ok(Value::Null)
    }
}