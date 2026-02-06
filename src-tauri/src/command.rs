use tauri::State;
use tracing::info;
use crate::{
    models::trade::{Kline, KlineRequest}, 
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

/// 获取缓存的K线数据
#[tauri::command]
pub async fn get_cached_klines(
    _state: State<'_, AppState>,
    _request: KlineRequest,
) -> Result<Vec<Kline>, String> {
    Ok(vec![])
    // state.klines_cache.get(&request).cloned().ok_or_else(|| "缓存中没有K线数据".to_string())
}

/// 更新缓存的K线数据
#[tauri::command]
pub async fn update_cache(
    _state: State<'_, AppState>,
    _symbol: String,
    _interval: String
) -> Result<usize, String> {
    Ok(0)
}
