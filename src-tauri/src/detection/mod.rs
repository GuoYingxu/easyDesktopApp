pub mod backend;
pub mod commands;
mod mock;

use std::sync::Arc;
use backend::DetectionBackend;
use mock::MockBackend;

/// Tauri 托管状态：持有当前活跃的检测后端。
///
/// 通过替换 `backend` 字段即可在运行前切换推理引擎，
/// 例如接入 ONNX Runtime 时只需：
/// ```rust
/// DetectionState { backend: Arc::new(OnnxBackend::new(model_path)) }
/// ```
pub struct DetectionState {
    pub backend: Arc<dyn DetectionBackend>,
}

impl DetectionState {
    /// 创建带默认后端的状态
    ///
    /// 当前默认使用 MockBackend；接入真实模型后在此处替换。
    pub fn with_default_backend() -> Self {
        Self {
            backend: Arc::new(MockBackend),
        }
    }

    /// 使用自定义后端创建状态（生产环境使用）
    pub fn with_backend(backend: impl DetectionBackend + 'static) -> Self {
        Self {
            backend: Arc::new(backend),
        }
    }
}
