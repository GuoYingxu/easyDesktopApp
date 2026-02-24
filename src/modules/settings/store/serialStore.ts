import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import type {
  SerialConfig,
  SerialDeviceConfig,
  SerialDataPayload,
  SerialErrorPayload,
  SerialStatusPayload,
  DeviceStatus,
} from '../models/SerialConfig'

const MAX_RECENT_DATA = 200

export const useSerialStore = defineStore('serial', {
  state: () => ({
    config: { devices: [] } as SerialConfig,
    availablePorts: [] as string[],
    /** 运行时设备状态（非持久化，由 serial:status / serial:error 事件驱动） */
    deviceStatuses: {} as Record<string, DeviceStatus>,
    /** 最近收到的串口数据（最多 MAX_RECENT_DATA 条） */
    recentData: [] as SerialDataPayload[],
    isLoading: false,
    error: null as string | null,
    listenersActive: false,
  }),

  getters: {
    enabledDevices: (state) => state.config.devices.filter((d) => d.enabled),
    deviceById: (state) => (id: string) =>
      state.config.devices.find((d) => d.device_id === id),
    statusOf: (state) => (id: string): DeviceStatus =>
      state.deviceStatuses[id] ?? 'idle',
  },

  actions: {
    // ─── 配置管理 ──────────────────────────────────────────────────────────

    async loadConfig(): Promise<void> {
      this.isLoading = true
      this.error = null
      try {
        const cfg = await invoke<SerialConfig>('get_serial_config')
        this.config = cfg
        // 初始化所有设备状态为 idle
        for (const dev of cfg.devices) {
          if (!this.deviceStatuses[dev.device_id]) {
            this.deviceStatuses[dev.device_id] = 'idle'
          }
        }
      } catch (err) {
        this.error = err instanceof Error ? err.message : String(err)
      } finally {
        this.isLoading = false
      }
    },

    async saveConfig(config: SerialConfig): Promise<void> {
      this.isLoading = true
      this.error = null
      try {
        await invoke('update_serial_config', { config })
        this.config = config
        // 重置所有状态为 idle，等待 serial:status 更新
        for (const dev of config.devices) {
          this.deviceStatuses[dev.device_id] = 'idle'
        }
      } catch (err) {
        this.error = err instanceof Error ? err.message : String(err)
        throw err
      } finally {
        this.isLoading = false
      }
    },

    async resetConfig(): Promise<void> {
      this.isLoading = true
      this.error = null
      try {
        const empty = await invoke<SerialConfig>('reset_serial_config')
        this.config = empty
        this.deviceStatuses = {}
        this.recentData = []
      } catch (err) {
        this.error = err instanceof Error ? err.message : String(err)
      } finally {
        this.isLoading = false
      }
    },

    // ─── 端口管理 ──────────────────────────────────────────────────────────

    async fetchAvailablePorts(): Promise<void> {
      try {
        this.availablePorts = await invoke<string[]>('list_available_ports')
      } catch (err) {
        this.availablePorts = []
        console.warn('获取串口列表失败:', err)
      }
    },

    // ─── 监听控制 ──────────────────────────────────────────────────────────

    async startListeners(): Promise<void> {
      await invoke('start_serial_listeners')
      this.listenersActive = true
    },

    async stopListeners(): Promise<void> {
      await invoke('stop_serial_listeners')
      this.listenersActive = false
    },

    // ─── 发送指令 ──────────────────────────────────────────────────────────

    async sendCommand(deviceId: string, data: number[]): Promise<void> {
      await invoke('send_serial_command', { deviceId, data })
    },

    // ─── 本地设备增删改（对话框操作，不立即持久化）──────────────────────────

    addDevice(device: SerialDeviceConfig): void {
      this.config.devices.push(device)
      this.deviceStatuses[device.device_id] = 'idle'
    },

    removeDevice(deviceId: string): void {
      this.config.devices = this.config.devices.filter(
        (d) => d.device_id !== deviceId,
      )
      delete this.deviceStatuses[deviceId]
    },

    updateDevice(deviceId: string, patch: Partial<SerialDeviceConfig>): void {
      const idx = this.config.devices.findIndex((d) => d.device_id === deviceId)
      if (idx !== -1) {
        this.config.devices[idx] = { ...this.config.devices[idx], ...patch }
      }
    },

    // ─── 事件回调（由 useSerialEvents composable 调用）──────────────────────

    onDataReceived(payload: SerialDataPayload): void {
      if (this.deviceStatuses[payload.device_id] !== 'connected') {
        this.deviceStatuses[payload.device_id] = 'connected'
      }
      this.recentData.unshift(payload)
      if (this.recentData.length > MAX_RECENT_DATA) {
        this.recentData.length = MAX_RECENT_DATA
      }
    },

    onErrorReceived(payload: SerialErrorPayload): void {
      this.deviceStatuses[payload.device_id] = 'error'
      console.error(`串口 ${payload.port} 错误:`, payload.error)
    },

    onStatusChanged(payload: SerialStatusPayload): void {
      this.deviceStatuses[payload.device_id] = payload.connected
        ? 'connected'
        : 'idle'
    },
  },
})
