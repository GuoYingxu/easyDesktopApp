use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;

/// 设备角色：决定收到数据后触发的事件类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum DeviceRole {
    /// 扫码枪 — 收到条码数据后触发外观检测流程
    Scanner,
    /// PLC — 收到信号后触发生产状态切换
    Plc,
    /// 光源控制器 — 接收系统发出的亮度/开关指令
    Light,
    /// 机械臂 — 接收系统发出的运动控制指令
    Robot,
    /// 相机 — 接收触发拍照等控制指令
    Camera,
}

/// 单个串口设备的连接参数与角色配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerialDeviceConfig {
    /// 唯一标识，由前端生成，如 "scanner_0"、"light_a"
    pub device_id: String,
    /// 显示名称，如 "扫码枪"、"光源A"
    pub name: String,
    /// 设备角色，决定数据路由逻辑
    pub role: DeviceRole,
    /// 串口端口名，如 "COM3"（Windows）或 "/dev/ttyUSB0"（Linux）
    pub port: String,
    /// 波特率，常用值：9600 / 19200 / 38400 / 57600 / 115200
    pub baud_rate: u32,
    /// 数据位：5 | 6 | 7 | 8，默认 8
    pub data_bits: u8,
    /// 停止位：1 | 2，默认 1
    pub stop_bits: u8,
    /// 奇偶校验："None" | "Odd" | "Even"，默认 "None"
    pub parity: String,
    /// 是否启用（false 时不开启监听线程）
    pub enabled: bool,
}

impl SerialDeviceConfig {
    /// 将 parity 字符串转换为 serialport crate 的枚举
    pub fn parity_value(&self) -> serialport::Parity {
        match self.parity.as_str() {
            "Odd" => serialport::Parity::Odd,
            "Even" => serialport::Parity::Even,
            _ => serialport::Parity::None,
        }
    }

    /// 将 data_bits 转换为 serialport crate 的枚举
    pub fn data_bits_value(&self) -> serialport::DataBits {
        match self.data_bits {
            5 => serialport::DataBits::Five,
            6 => serialport::DataBits::Six,
            7 => serialport::DataBits::Seven,
            _ => serialport::DataBits::Eight,
        }
    }

    /// 将 stop_bits 转换为 serialport crate 的枚举
    pub fn stop_bits_value(&self) -> serialport::StopBits {
        match self.stop_bits {
            2 => serialport::StopBits::Two,
            _ => serialport::StopBits::One,
        }
    }
}

/// 所有串口设备的配置集合
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SerialConfig {
    pub devices: Vec<SerialDeviceConfig>,
}

impl SerialConfig {
    /// 配置文件路径：`{config_dir}/easydesktopapp/serial_config.json`
    pub fn config_path() -> PathBuf {
        let mut base = dirs::config_dir()
            .unwrap_or_else(|| std::env::current_dir().unwrap());
        base.push(env!("CARGO_PKG_NAME"));
        base.push("serial_config.json");
        base
    }

    /// 从磁盘加载配置；文件不存在或解析失败时返回空配置
    pub fn load_or_default() -> Self {
        let path = Self::config_path();
        match fs::read_to_string(&path) {
            Ok(s) => serde_json::from_str::<SerialConfig>(&s).unwrap_or_else(|e| {
                eprintln!("serial_config.json 解析失败，使用空配置: {}", e);
                SerialConfig::default()
            }),
            Err(_) => SerialConfig::default(),
        }
    }

    /// 持久化配置到磁盘（自动创建父目录）
    pub fn save(&self) -> io::Result<()> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let s = serde_json::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        fs::write(path, s)
    }

    /// 校验配置合法性
    pub fn validate(&self) -> Result<(), String> {
        for dev in &self.devices {
            if dev.device_id.is_empty() {
                return Err("device_id 不能为空".into());
            }
            if dev.port.is_empty() {
                return Err(format!("设备 {} 的 port 不能为空", dev.name));
            }
            if dev.baud_rate == 0 {
                return Err(format!("设备 {} 的 baud_rate 不能为 0", dev.name));
            }
            if ![5u8, 6, 7, 8].contains(&dev.data_bits) {
                return Err(format!("设备 {} 的 data_bits 必须为 5/6/7/8", dev.name));
            }
            if ![1u8, 2].contains(&dev.stop_bits) {
                return Err(format!("设备 {} 的 stop_bits 必须为 1 或 2", dev.name));
            }
        }
        Ok(())
    }

    /// 按 device_id 查找设备
    pub fn find_device(&self, device_id: &str) -> Option<&SerialDeviceConfig> {
        self.devices.iter().find(|d| d.device_id == device_id)
    }
}
