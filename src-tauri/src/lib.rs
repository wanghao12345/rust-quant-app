use std::sync::Arc;

use crate::{binance::client::BinanceClient, database::store::MarketDataStore};


pub mod command;
pub mod state;
pub mod binance;
pub mod models;
pub mod error;
pub mod database;



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let binance_client = Arc::new(BinanceClient::new());

    // 初始化数据库
    let db_path = "binance.db";
    let data_store = match MarketDataStore::new(db_path).await {
        Ok(store) => Arc::new(store),
        Err(e) => {
            tracing::error!("数据库初始化失败: {:?}", e);
            panic!("数据库初始化失败: {:?}", e);
        }
    };

    // 初始化状态
    let state = state::AppState { 
        binance_client: binance_client.clone(),
        data_store: data_store.clone(),
    };

    tauri::Builder::default()
        .manage(state)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            command::fetch_klines,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
