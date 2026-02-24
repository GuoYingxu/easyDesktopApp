use log::LevelFilter;
use tauri::{Runtime, plugin::TauriPlugin};
use tauri_plugin_log::{Builder as LogBuilder, Target, RotationStrategy, TargetKind};
use super::config::LogConfig;

/// 从配置构建日志插件
fn build_log_plugin_from_config<R: Runtime>(cfg: &LogConfig) -> TauriPlugin<R> {
    // 解析LevelFilter
    let level = cfg.level.parse::<LevelFilter>().unwrap_or(LevelFilter::Info);

    // 解析max_size
    let default_max_size = 10u128 * 1024 * 1024; // 10 MB
    let max_size = cfg.max_file_size.unwrap_or(default_max_size);

    // 构建目标列表
    let mut targets = vec![
        // 总是写入日志文件
        Target::new(TargetKind::LogDir { file_name: None })
    ];

    // 根据配置决定是否输出到stdout
    if cfg.to_stdout {
        targets.push(Target::new(TargetKind::Stdout));
    }
    
    // 构建日志器
    let mut builder = LogBuilder::new()
        .level(level)
        .max_file_size(max_size)
        .targets(targets)
        .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal);
        // .formatter(formatter);

    // 设置日志轮转策略：保留指定数量的归档文件
    if cfg.keep_files > 0 {
        builder = builder.rotation_strategy(RotationStrategy::KeepSome(cfg.keep_files));
    }

    builder.build()
}

/// 初始化日志系统
///
/// 加载配置文件（若不存在则使用默认配置），然后构建日志插件。
/// 日志将根据配置输出到文件和/或标准输出，并自动进行轮转。
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    // 加载或使用默认配置
    let cfg = LogConfig::load_or_default();

    // 校验配置
    if let Err(e) = cfg.validate() {
        eprintln!("Invalid log config: {}, using default", e);
        return build_log_plugin_from_config(&LogConfig::default());
    }

    // 构建日志插件
    build_log_plugin_from_config(&cfg)
}
