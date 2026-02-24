use super::backend::{DetectionBackend, DetectionResult, Defect};

/// Mock 后端 —— 不依赖任何模型文件，专供开发调试使用。
///
/// 行为：
/// - 图像字节长度为奇数时模拟检测到划痕（方便前端验证缺陷渲染逻辑）
/// - 其余情况返回合格（无缺陷）
pub struct MockBackend;

impl DetectionBackend for MockBackend {
    fn name(&self) -> &str {
        "mock"
    }

    fn detect(&self, image_data: &[u8]) -> Result<DetectionResult, String> {
        // 用字节长度奇偶来模拟有/无缺陷，避免引入随机依赖
        if image_data.len() % 2 == 1 {
            Ok(DetectionResult {
                defects: vec![Defect {
                    label: "划痕".to_string(),
                    confidence: 0.87,
                    bbox: [0.10, 0.20, 0.30, 0.15],
                }],
                inference_ms: 12,
            })
        } else {
            Ok(DetectionResult::pass(8))
        }
    }

    fn is_ready(&self) -> bool {
        true
    }
}
