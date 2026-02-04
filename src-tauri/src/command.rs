use serde::{Deserialize, Serialize};
use tauri::State;

use crate::state::AppState;

#[derive(Debug, Deserialize, Serialize)]
pub struct KlineRequest {
    symbol: String,
    interval: String,
    start_time: u64,
    end_time: u64,
}

// 获取K线数据
#[tauri::command]
pub async fn fetch_klines(
    _state: State<'_, AppState>,
    _request: KlineRequest,
) -> Result<(), String> {
    Ok(())
}

// 获取交易对列表
#[tauri::command]
pub async fn fetch_symbols() -> Vec<String> {
    vec![]
}