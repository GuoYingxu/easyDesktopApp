use dirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;

/// 应用端日志配置（简洁可扩展）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    /// 最大文件大小（字节）。None 表示使用默认值（见 DEFAULT_MAX_FILE_SIZE）。
    pub max_file_size: Option<u128>,
    /// 保留归档文件数量（KeepSome semantics）。
    pub keep_files: usize,
    /// 日志级别字符串，例如 "Info", "Debug"。会在构造日志时解析为 LevelFilter。
    pub level: String,
    /// 是否同时输出到 stdout（便于开发/容器环境）。
    pub to_stdout: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            max_file_size: Some(10 * 1024 * 1024), // 10 MB 默认
            keep_files: 10,                        // 保留最近 10 个归档
            level: "Info".into(),
            to_stdout: true,
        }
    }
}

impl LogConfig {
    /// 返回配置文件路径：使用系统配置目录（dirs::config_dir）下以 crate 名称为目录的 log_config.json
    pub fn config_path() -> PathBuf {
        // 优先使用平台配置目录，否则回退到当前目录
        let mut base = dirs::config_dir().unwrap_or_else(|| std::env::current_dir().unwrap());
        // 使用 Cargo 包名作为子目录（通常等于 product name）
        println!("cargo pkg name: {}", env!("CARGO_PKG_NAME"));
        base.push(env!("CARGO_PKG_NAME"));
        // 创建目录（调用方可保证目录存在或在 save 中创建）
        base.push("log_config.json");
        println!("log config path: {}", base.display());
        base
    }

    /// 从磁盘加载配置，若不存在或解析错误则返回默认配置
    pub fn load_or_default() -> Self {
        let path = Self::config_path();
        match fs::read_to_string(&path) {
            Ok(s) => match serde_json::from_str::<LogConfig>(&s) {
                Ok(cfg) => cfg,
                Err(e) => {
                    eprintln!(
                        "failed to parse log config {}, using default: {}",
                        path.display(),
                        e
                    );
                    LogConfig::default()
                }
            },
            Err(_) => LogConfig::default(),
        }
    }

    /// 保存配置到磁盘（会创建父目录）
    pub fn save(&self) -> io::Result<()> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let s = serde_json::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("serialize error: {}", e)))?;
        fs::write(path, s)?;
        Ok(())
    }

    /// 简单校验配置（例如确保 keep_files 不等于 0）
    pub fn validate(&self) -> Result<(), String> {
        if self.keep_files == 0 {
            return Err("keep_files must be >= 1".into());
        }
        if let Some(v) = self.max_file_size {
            if v == 0 {
                return Err("max_file_size must be > 0".into());
            }
        }
        // level 的合法性由上层 parse 为 LevelFilter 时决定（这里不严格检查）
        Ok(())
    }
}
