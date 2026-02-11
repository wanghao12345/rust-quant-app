use std::sync::Arc;

use crate::{binance::client::BinanceClient, database::store::MarketDataStore};


#[derive(Clone)]
pub struct AppState {
    pub binance_client: Arc<BinanceClient>,
    pub data_store: Arc<MarketDataStore>,
}
