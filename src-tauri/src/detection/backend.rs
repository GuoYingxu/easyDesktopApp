use serde::{Deserialize, Serialize};

/// 单个缺陷描述
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Defect {
    /// 缺陷类型，如 "划痕"、"凹陷"、"色差"
    pub label: String,
    /// 置信度 0.0 ~ 1.0
    pub confidence: f32,
    /// 归一化边界框 [x, y, width, height]
    pub bbox: [f32; 4],
}

/// 一次推理的完整结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionResult {
    /// 检测到的缺陷列表；为空表示外观合格
    pub defects: Vec<Defect>,
    /// 推理耗时（毫秒）
    pub inference_ms: u64,
}

impl DetectionResult {
    /// 无缺陷的快捷构造
    pub fn pass(inference_ms: u64) -> Self {
        Self { defects: vec![], inference_ms }
    }
}

/// 检测后端抽象接口
///
/// 实现此 trait 即可接入不同的推理引擎（ONNX Runtime、
/// OpenCV DNN、远程 HTTP 服务等），无需改动上层代码。
pub trait DetectionBackend: Send + Sync {
    /// 后端唯一名称，用于日志和前端展示
    fn name(&self) -> &str;

    /// 对一帧图像执行缺陷检测
    ///
    /// * `image_data` — JPEG 或 PNG 的原始字节
    fn detect(&self, image_data: &[u8]) -> Result<DetectionResult, String>;

    /// 后端是否已就绪（例如模型是否已加载完毕）
    fn is_ready(&self) -> bool;
}
