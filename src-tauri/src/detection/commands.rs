use tauri::State;
use super::backend::DetectionResult;
use super::DetectionState;

/// 对一帧图像执行缺陷检测
///
/// 前端调用示例：
/// ```ts
/// import { invoke } from '@tauri-apps/api/core'
/// const result = await invoke<DetectionResult>('detect_image', { imageData: bytes })
/// ```
#[tauri::command]
pub async fn detect_image(
    image_data: Vec<u8>,
    state: State<'_, DetectionState>,
) -> Result<DetectionResult, String> {
    state.backend.detect(&image_data)
}

/// 查询当前后端名称（如 "mock" / "onnx"）
#[tauri::command]
pub async fn get_backend_name(
    state: State<'_, DetectionState>,
) -> Result<String, String> {
    Ok(state.backend.name().to_string())
}

/// 查询后端是否就绪（模型加载完毕）
#[tauri::command]
pub async fn is_backend_ready(
    state: State<'_, DetectionState>,
) -> Result<bool, String> {
    Ok(state.backend.is_ready())
}
