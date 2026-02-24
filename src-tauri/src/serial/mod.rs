pub mod commands;
pub mod config;
pub mod manager;

use manager::SerialManager;

/// Tauri 托管状态：持有串口管理器
///
/// 通过 `app.manage(SerialState::new())` 注入，
/// 在命令函数中通过 `State<'_, SerialState>` 访问。
pub struct SerialState {
    pub manager: SerialManager,
}

impl SerialState {
    pub fn new() -> Self {
        Self {
            manager: SerialManager::new(),
        }
    }
}
