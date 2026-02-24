use tauri::Manager;

#[cfg(feature = "logging")]
mod logging;

#[cfg(feature = "tray")]
mod tray;

// mod menu;

#[cfg(feature = "detection")]
mod detection;

#[cfg(feature = "serial")]
mod serial;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default();

    #[cfg(feature = "logging")]
    let builder = builder.plugin(logging::logs::init());

    #[cfg(feature = "single-instance")]
    let builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
        log::info!("检测到第二个实例启动，激活现有窗口");
        if let Some(window) = app.get_webview_window("main") {
            let _ = window.show();
            let _ = window.set_focus();
            let _ = window.unminimize();
        }
    }));

    #[cfg(feature = "detection")]
    let builder = builder.manage(detection::DetectionState::with_default_backend());

    #[cfg(feature = "serial")]
    let builder = builder.manage(serial::SerialState::new());

    builder
        .setup(|app| {
            #[cfg(feature = "logging")]
            if let Ok(log_dir) = app.path().app_log_dir() {
                println!("=====================================");
                println!("日志文件目录: {}", log_dir.display());
                println!("=====================================");
            }

            log::info!("Application started successfully");

            // 启动串口监听（需要 AppHandle，必须在 setup 内）
            #[cfg(feature = "serial")]
            {
                let cfg = serial::config::SerialConfig::load_or_default();
                let state = app.state::<serial::SerialState>();
                state.manager.start_all(&cfg, app.handle().clone());
                log::info!("串口监听已初始化，共 {} 个设备", cfg.devices.len());
            }

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_process::init())
        .invoke_handler(tauri::generate_handler![
            // --- 日志管理命令（仅 logging feature）---
            #[cfg(feature = "logging")]
            crate::logging::commands::get_log_config,
            #[cfg(feature = "logging")]
            crate::logging::commands::update_log_config,
            #[cfg(feature = "logging")]
            crate::logging::commands::reset_log_config,
            // --- 检测命令（仅 detection feature）---
            #[cfg(feature = "detection")]
            crate::detection::commands::detect_image,
            #[cfg(feature = "detection")]
            crate::detection::commands::get_backend_name,
            #[cfg(feature = "detection")]
            crate::detection::commands::is_backend_ready,
            // --- 串口命令（仅 serial feature）---
            #[cfg(feature = "serial")]
            crate::serial::commands::get_serial_config,
            #[cfg(feature = "serial")]
            crate::serial::commands::update_serial_config,
            #[cfg(feature = "serial")]
            crate::serial::commands::reset_serial_config,
            #[cfg(feature = "serial")]
            crate::serial::commands::start_serial_listeners,
            #[cfg(feature = "serial")]
            crate::serial::commands::stop_serial_listeners,
            #[cfg(feature = "serial")]
            crate::serial::commands::send_serial_command,
            #[cfg(feature = "serial")]
            crate::serial::commands::list_available_ports,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
