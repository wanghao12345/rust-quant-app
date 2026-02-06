use std::sync::Arc;

use crate::binance::client::BinanceClient;


pub mod command;
pub mod state;
pub mod binance;
pub mod models;
pub mod error;
pub mod database;



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let binance_client = Arc::new(BinanceClient::new());

    // 初始化状态
    let state = state::AppState { 
        binance_client: binance_client.clone(),
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
