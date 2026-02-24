use std::collections::HashMap;
use std::io::Read;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::Serialize;
use serialport::SerialPort;
use tauri::{AppHandle, Emitter};

use super::config::{DeviceRole, SerialConfig, SerialDeviceConfig};

// ─── 事件 Payload ────────────────────────────────────────────────────────────

/// `serial:data` 事件：收到串口数据时推送给前端
#[derive(Debug, Clone, Serialize)]
pub struct SerialDataPayload {
    pub device_id: String,
    pub role: DeviceRole,
    pub port: String,
    /// 原始字节数组
    pub data: Vec<u8>,
    /// 尝试 UTF-8 解码；失败时为 None
    pub data_str: Option<String>,
    pub timestamp_ms: u64,
}

/// `serial:error` 事件：串口读取失败，监听线程将退出
#[derive(Debug, Clone, Serialize)]
pub struct SerialErrorPayload {
    pub device_id: String,
    pub port: String,
    pub error: String,
    pub timestamp_ms: u64,
}

/// `serial:status` 事件：设备连接/断开状态变化
#[derive(Debug, Clone, Serialize)]
pub struct SerialStatusPayload {
    pub device_id: String,
    pub port: String,
    pub connected: bool,
    pub timestamp_ms: u64,
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

// ─── 单个运行中设备的句柄 ─────────────────────────────────────────────────────

struct RunningDevice {
    /// 设置为 true 时，读循环在下次超时后退出
    cancel: Arc<AtomicBool>,
    /// 写端口（克隆自读端口，线程安全）
    write_port: Arc<Mutex<Box<dyn SerialPort>>>,
}

// ─── SerialManager ────────────────────────────────────────────────────────────

/// 串口监听线程的生命周期管理器
///
/// - `start_all`：按配置为每个 enabled 设备启动一个读线程
/// - `stop_all`：设置所有 cancel flag，线程在 100ms 超时后自然退出
/// - `restart_all`：先 stop_all 再 start_all（配置变更时调用）
/// - `send_to_device`：通过写端口向指定设备发送字节
pub struct SerialManager {
    running: Mutex<HashMap<String, RunningDevice>>,
}

impl SerialManager {
    pub fn new() -> Self {
        Self {
            running: Mutex::new(HashMap::new()),
        }
    }

    /// 为配置中所有 `enabled = true` 的设备启动监听线程
    pub fn start_all(&self, config: &SerialConfig, app: AppHandle) {
        let mut running = self.running.lock().unwrap();
        for dev in config.devices.iter().filter(|d| d.enabled) {
            if running.contains_key(&dev.device_id) {
                continue; // 已在运行，跳过
            }
            if let Some(handle) = Self::spawn_reader(dev.clone(), app.clone()) {
                running.insert(dev.device_id.clone(), handle);
            }
        }
    }

    /// 停止所有监听线程（设置 cancel flag；线程在读超时后退出）
    pub fn stop_all(&self) {
        let mut running = self.running.lock().unwrap();
        for (_, dev) in running.drain() {
            dev.cancel.store(true, Ordering::Relaxed);
        }
    }

    /// 重启所有监听线程（配置变更后调用）
    pub fn restart_all(&self, config: &SerialConfig, app: AppHandle) {
        self.stop_all();
        // 等待旧线程退出（最长 200ms）
        std::thread::sleep(Duration::from_millis(200));
        self.start_all(config, app);
    }

    /// 向指定设备的串口写入数据
    pub fn send_to_device(&self, device_id: &str, data: Vec<u8>) -> Result<(), String> {
        let running = self.running.lock().unwrap();
        let dev = running
            .get(device_id)
            .ok_or_else(|| format!("设备 {} 未连接或未启用", device_id))?;
        let mut port = dev.write_port.lock().unwrap();
        use std::io::Write;
        port.write_all(&data)
            .map_err(|e| format!("写入失败: {}", e))
    }

    // ─── 内部：为单个设备打开端口并启动读线程 ────────────────────────────────

    fn spawn_reader(cfg: SerialDeviceConfig, app: AppHandle) -> Option<RunningDevice> {
        // 打开串口，显式禁用硬件流控（USB CDC 虚拟串口不需要 RTS/CTS）
        let port = serialport::new(&cfg.port, cfg.baud_rate)
            .data_bits(cfg.data_bits_value())
            .stop_bits(cfg.stop_bits_value())
            .parity(cfg.parity_value())
            .flow_control(serialport::FlowControl::None)
            .timeout(Duration::from_millis(100))
            .open();

        let mut read_port = match port {
            Ok(mut p) => {
                // USB CDC 扫码枪通常需要 DTR=true 才会开始发送数据
                // DTR 信号通知设备"主机已就绪"，没有它扫码枪会静默
                if let Err(e) = p.write_data_terminal_ready(true) {
                    log::warn!("[串口 {}|{}] 设置 DTR 失败（非致命）: {}", cfg.name, cfg.port, e);
                } else {
                    log::info!("[串口 {}|{}] DTR 已置高", cfg.name, cfg.port);
                }
                p
            }
            Err(e) => {
                log::warn!(
                    "串口 {} ({}) 打开失败: {}",
                    cfg.name,
                    cfg.port,
                    e
                );
                // 向前端报告连接失败
                let _ = app.emit(
                    "serial:error",
                    SerialErrorPayload {
                        device_id: cfg.device_id.clone(),
                        port: cfg.port.clone(),
                        error: e.to_string(),
                        timestamp_ms: now_ms(),
                    },
                );
                return None;
            }
        };

        // 克隆写端口（读循环使用原始端口）
        let write_port = match read_port.try_clone() {
            Ok(p) => Arc::new(Mutex::new(p)),
            Err(e) => {
                log::warn!("串口 {} try_clone 失败: {}", cfg.port, e);
                return None;
            }
        };

        let cancel = Arc::new(AtomicBool::new(false));
        let cancel_clone = cancel.clone();

        // 通知前端：设备已连接
        let _ = app.emit(
            "serial:status",
            SerialStatusPayload {
                device_id: cfg.device_id.clone(),
                port: cfg.port.clone(),
                connected: true,
                timestamp_ms: now_ms(),
            },
        );

        log::info!("串口 {} ({}) 监听启动", cfg.name, cfg.port);

        std::thread::spawn(move || {
            let mut buf = vec![0u8; 256];
            let mut line_buf: Vec<u8> = Vec::new();
            // 每 50 次超时（约 5 秒）打印一次诊断日志
            let mut timeout_count = 0u32;

            loop {
                if cancel_clone.load(Ordering::Relaxed) {
                    break;
                }

                match read_port.read(&mut buf) {
                    Ok(0) => continue,
                    Ok(n) => {
                        for &byte in &buf[..n] {
                            if byte == b'\r' || byte == b'\n' {
                                // 收到行结束符，将缓冲区作为一条完整消息emit
                                if !line_buf.is_empty() {
                                    let complete = std::mem::take(&mut line_buf);
                                    let data_str = String::from_utf8(complete.clone())
                                        .ok()
                                        .map(|s| s.trim().to_string())
                                        .filter(|s| !s.is_empty());

                                    // info 级别：确保在默认日志配置下可见
                                    match &data_str {
                                        Some(s) => log::info!(
                                            "[串口 {}|{}] RX: {}",
                                            cfg.name, cfg.port, s
                                        ),
                                        None => log::info!(
                                            "[串口 {}|{}] RX (hex): {}",
                                            cfg.name, cfg.port,
                                            complete.iter().map(|b| format!("{b:02X}")).collect::<Vec<_>>().join(" ")
                                        ),
                                    }

                                    let _ = app.emit(
                                        "serial:data",
                                        SerialDataPayload {
                                            device_id: cfg.device_id.clone(),
                                            role: cfg.role.clone(),
                                            port: cfg.port.clone(),
                                            data: complete,
                                            data_str,
                                            timestamp_ms: now_ms(),
                                        },
                                    );
                                }
                            } else {
                                line_buf.push(byte);
                            }
                        }
                    }
                    Err(e)
                        if e.kind() == std::io::ErrorKind::TimedOut
                            || e.kind() == std::io::ErrorKind::WouldBlock =>
                    {
                        timeout_count += 1;
                        // 每 ~5 秒打印一次诊断：显示缓冲区待读字节数
                        // 有字节但没被读到 → read() 问题
                        // 一直 0   → 数据没到达串口层（DTR/模式/端口号问题）
                        if timeout_count % 50 == 0 {
                            let pending = read_port.bytes_to_read().unwrap_or(0);
                            log::info!(
                                "[串口 {}|{}] 等待中... 缓冲区待读 {} 字节",
                                cfg.name, cfg.port, pending
                            );
                        }
                        // 超时/非阻塞返回：正常，继续循环检查 cancel flag
                        // 若缓冲区已有数据但长时间无行结束符，主动flush（适配无结束符协议）
                        if line_buf.len() >= 64 {
                            let complete = std::mem::take(&mut line_buf);
                            let data_str = String::from_utf8(complete.clone()).ok();
                            log::info!(
                                "[串口 {}|{}] RX flush {}B",
                                cfg.name, cfg.port, complete.len()
                            );
                            let _ = app.emit(
                                "serial:data",
                                SerialDataPayload {
                                    device_id: cfg.device_id.clone(),
                                    role: cfg.role.clone(),
                                    port: cfg.port.clone(),
                                    data: complete,
                                    data_str,
                                    timestamp_ms: now_ms(),
                                },
                            );
                        }
                        continue;
                    }
                    Err(e) => {
                        log::error!("串口 {} 读取错误: {}", cfg.port, e);
                        let _ = app.emit(
                            "serial:error",
                            SerialErrorPayload {
                                device_id: cfg.device_id.clone(),
                                port: cfg.port.clone(),
                                error: e.to_string(),
                                timestamp_ms: now_ms(),
                            },
                        );
                        let _ = app.emit(
                            "serial:status",
                            SerialStatusPayload {
                                device_id: cfg.device_id.clone(),
                                port: cfg.port.clone(),
                                connected: false,
                                timestamp_ms: now_ms(),
                            },
                        );
                        break;
                    }
                }
            }

            log::info!("串口 {} ({}) 监听线程退出", cfg.name, cfg.port);
        });

        Some(RunningDevice { cancel, write_port })
    }
}
