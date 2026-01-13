// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use log;
mod logging;
mod menu;
mod tray;
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    //   let daily_plugin = make_daily_plugin::<tauri::Wry>(7, true); // keep last 7 days, use UTC
    tauri::Builder::default()
        .setup(|app| {
            // 初始化日志
            // let _app_handle = logging::normal_log::init_logger(app);
            // // 测试
            // log::info!("Application started");
            // log::debug!("Debug message");
            // log::warn!("Warning message");
            // Try to load JSON config next to executable, otherwise use default
            // tray::create_tray(app)?;
            // menu::create_menu(app)?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
