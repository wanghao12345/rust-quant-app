use tauri::State;
use tracing::info;
use crate::{
    models::trade::KlineRequest, 
    state::AppState
};

// 获取K线数据
#[tauri::command]
pub async fn fetch_klines(
    state: State<'_, AppState>,
    request: KlineRequest,
) -> Result<(), String> {
    info!("获取K线数据{:?}", request);
    state.binance_client.get_klines(&request).await.map_err(|e| e.to_string())?;
    Ok(())
}

// 获取交易对列表
#[tauri::command]
pub async fn fetch_symbols() -> Vec<String> {
    vec![]
}