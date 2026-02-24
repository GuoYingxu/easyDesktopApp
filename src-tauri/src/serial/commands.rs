use tauri::{AppHandle, State};

use super::config::SerialConfig;
use super::SerialState;

/// 获取当前串口配置
#[tauri::command]
pub async fn get_serial_config() -> Result<SerialConfig, String> {
    log::debug!("get_serial_config");
    Ok(SerialConfig::load_or_default())
}

/// 保存串口配置并重启所有监听线程
///
/// 调用后后端会：校验 → 持久化 → 重启线程。
/// 前端只需调用一次，无需额外触发 `start_serial_listeners`。
#[tauri::command]
pub async fn update_serial_config(
    config: SerialConfig,
    state: State<'_, SerialState>,
    app: AppHandle,
) -> Result<(), String> {
    config.validate()?;
    config
        .save()
        .map_err(|e| format!("保存配置失败: {}", e))?;
    log::info!("串口配置已更新，共 {} 个设备", config.devices.len());
    state.manager.restart_all(&config, app);
    Ok(())
}

/// 重置串口配置为空（停止所有监听）
#[tauri::command]
pub async fn reset_serial_config(state: State<'_, SerialState>) -> Result<SerialConfig, String> {
    state.manager.stop_all();
    let empty = SerialConfig::default();
    empty
        .save()
        .map_err(|e| format!("保存配置失败: {}", e))?;
    log::info!("串口配置已重置");
    Ok(empty)
}

/// 手动启动所有 enabled 设备的监听线程
///
/// 通常由 setup 自动调用；提供此命令以便前端在需要时手动重连。
#[tauri::command]
pub async fn start_serial_listeners(
    state: State<'_, SerialState>,
    app: AppHandle,
) -> Result<(), String> {
    let config = SerialConfig::load_or_default();
    state.manager.start_all(&config, app);
    log::info!("串口监听已启动");
    Ok(())
}

/// 手动停止所有监听线程
#[tauri::command]
pub async fn stop_serial_listeners(state: State<'_, SerialState>) -> Result<(), String> {
    state.manager.stop_all();
    log::info!("串口监听已停止");
    Ok(())
}

/// 向指定设备的串口写入字节数据
///
/// 适用于向光源、机械臂等设备发送控制指令。
#[tauri::command]
pub async fn send_serial_command(
    device_id: String,
    data: Vec<u8>,
    state: State<'_, SerialState>,
) -> Result<(), String> {
    state.manager.send_to_device(&device_id, data)
}

/// 列出当前系统可用的串口端口名
///
/// 前端用于填充端口选择下拉框。
#[tauri::command]
pub async fn list_available_ports() -> Result<Vec<String>, String> {
    serialport::available_ports()
        .map(|ports| ports.into_iter().map(|p| p.port_name).collect())
        .map_err(|e| format!("获取端口列表失败: {}", e))
}
