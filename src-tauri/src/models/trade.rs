use serde::Deserialize;

/// K线请求参数
#[derive(Debug, Deserialize)]
pub struct KlineRequest {
    /// 交易对，例如 "BTCUSDT"
    pub symbol: String,
    /// K线时间间隔，例如 "1m", "5m", "1h", "1d"
    pub interval: String,
    /// 开始时间，Unix时间戳（毫秒）
    pub start_time: Option<u64>,
    /// 结束时间，Unix时间戳（毫秒）
    pub end_time: Option<u64>,
    /// 返回数据数量，默认 500，最大 1000
    pub limit: Option<u32>,
}

/// K线数据
#[derive(Debug, Deserialize)]
pub struct Kline {
    // 开盘时间
    pub open_time: u64,
    // 开盘价
    pub open: String,
    // 最高价
    pub high: String,
    // 最低价
    pub low: String,
    // 收盘价
    pub close: String,
    // 成交量
    pub volume: String,
    // 收盘时间
    pub close_time: u64,
    // 成交额
    pub quote_asset_volume: String,
    // 成交笔数
    pub number_of_trades: u64,
    // 主动买入成交量
    pub taker_buy_base_asset_volume: String,
    // 主动买入成交额
    pub taker_buy_quote_asset_volume: String,
    // 忽略字段
    pub ignore: String,
}