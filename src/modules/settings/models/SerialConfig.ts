/** 设备角色（与 Rust DeviceRole enum 对应，serde camelCase） */
export type DeviceRole = 'scanner' | 'plc' | 'light' | 'robot' | 'camera'

export type Parity = 'None' | 'Odd' | 'Even'

export type DeviceStatus = 'idle' | 'connected' | 'error'

/** 单个串口设备配置（对应 Rust SerialDeviceConfig） */
export interface SerialDeviceConfig {
  device_id: string
  name: string
  role: DeviceRole
  port: string
  baud_rate: number
  data_bits: number
  stop_bits: number
  parity: Parity
  enabled: boolean
}

/** 串口配置整体（对应 Rust SerialConfig） */
export interface SerialConfig {
  devices: SerialDeviceConfig[]
}

/** `serial:data` 事件 payload（对应 Rust SerialDataPayload） */
export interface SerialDataPayload {
  device_id: string
  role: DeviceRole
  port: string
  data: number[]
  data_str: string | null
  timestamp_ms: number
}

/** `serial:error` 事件 payload */
export interface SerialErrorPayload {
  device_id: string
  port: string
  error: string
  timestamp_ms: number
}

/** `serial:status` 事件 payload */
export interface SerialStatusPayload {
  device_id: string
  port: string
  connected: boolean
  timestamp_ms: number
}

/** 设备角色的中文标签和颜色 */
export const ROLE_META: Record<DeviceRole, { label: string; type: 'primary' | 'success' | 'warning' | 'danger' | 'info' }> = {
  scanner: { label: '扫码枪', type: 'primary' },
  plc:     { label: 'PLC',   type: 'warning' },
  light:   { label: '光源',  type: 'success' },
  robot:   { label: '机械臂', type: 'danger' },
  camera:  { label: '相机',  type: 'info' },
}

/** 常用波特率选项 */
export const BAUD_RATES = [9600, 19200, 38400, 57600, 115200]

/** 创建具有默认值的新设备配置 */
export function createDefaultDevice(role: DeviceRole): SerialDeviceConfig {
  return {
    device_id: `${role}_${Date.now()}`,
    name: ROLE_META[role].label,
    role,
    port: '',
    baud_rate: 9600,
    data_bits: 8,
    stop_bits: 1,
    parity: 'None',
    enabled: true,
  }
}
