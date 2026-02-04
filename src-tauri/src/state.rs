use std::sync::Arc;

use crate::binance::client::BinanceClient;


#[derive(Clone)]
pub struct AppState {
    pub binance_client: Arc<BinanceClient>,
}
