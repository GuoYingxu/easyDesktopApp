use tauri::command;
use crate::logging::config::LogConfig;

/// 获取当前日志配置
#[command]
pub async fn get_log_config() -> Result<LogConfig, String> {
    log::debug!("get_log_config");
    Ok(LogConfig::load_or_default())
}

/// 更新日志配置
#[command]
pub async fn update_log_config(config: LogConfig) -> Result<(), String> {
    // 验证配置
    if let Err(e) = config.validate() {
       log::error!("Invalid log config: {}", e);
        return Err(format!("Invalid log config: {}", e));
    }

    // 保存配置到磁盘
    config.save()
        .map_err(|e| format!("Failed to save log config: {}", e))?;
    log::info!("Log config updated  {:?}", config);
    Ok(())
}

/// 重置日志配置为默认值
#[command]
pub async fn reset_log_config() -> Result<LogConfig, String> {
    let default_config = LogConfig::default();
    
    // 保存默认配置到磁盘
    default_config.save()
        .map_err(|e| format!("Failed to save default log config: {}", e))?;
    log::info!("Log config reset to default");
    Ok(default_config)
}