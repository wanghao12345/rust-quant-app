
pub mod command;
pub mod state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // 初始化状态
    let state = state::AppState {};

    tauri::Builder::default()
        .manage(state)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            command::fetch_klines,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
