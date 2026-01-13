use log::LevelFilter;
use tauri::{ Runtime, plugin::Builder, plugin::TauriPlugin};
use tauri_plugin_log::{Builder as LogBuilder, Target, RotationStrategy,TargetKind};
use super::config::LogConfig;

fn build_log_plugin_from_config(cfg: &LogConfig)-> TauriPlugin<tauri::Wry> {
    // 解析LevelFilter
    let level = cfg.level.parse::<LevelFilter>().unwrap_or(LevelFilter::Info); 
    // 解析max_size
    let default_max_size = 10u128 * 1024 * 1024;
    let max_size = cfg.max_file_size.unwrap_or(default_max_size);

    let mut builder =  LogBuilder::new()
        .rotation_strategy(RotationStrategy::KeepSome(cfg.keep_files))
        .max_file_size(max_size)
        .level(level);
    // 根据配置决定是否输出到stdout 
    if cfg.to_stdout {
        builder = builder.target(Target::new(TargetKind::Stdout));
    }
    // 写入app_log_dir
    builder = builder.target(Target::new(TargetKind::LogDir {file_name:None}));
    builder.build()
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
 
    Builder::new("logs")
        .plugin( build_log_plugin_from_config)
        .invoke_handler(|_webview, _args| Ok(()))
        .build()
}
