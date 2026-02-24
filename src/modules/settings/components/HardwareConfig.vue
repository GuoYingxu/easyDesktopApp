<template>
  <div class="p-4 flex flex-col gap-6">

    <!-- PLC Ethernet 配置（独立于串口模块，保持原有结构） -->
    <div>
      <div class="text-sm font-medium mb-2">PLC（以太网）</div>
      <el-form :model="plcConfig" label-width="80px" class="max-w-sm">
        <el-form-item label="IP 地址">
          <el-input v-model="plcConfig.ip" placeholder="192.168.0.10" />
        </el-form-item>
        <el-form-item label="端口">
          <el-input-number v-model="plcConfig.port" :min="1" :max="65535" />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" size="small" @click="savePlc">保存</el-button>
        </el-form-item>
      </el-form>
    </div>

    <!-- 串口设备管理 -->
    <div>
      <div class="flex justify-between items-center mb-3">
        <div class="text-sm font-medium">串口设备</div>
        <div class="flex gap-2">
          <el-button size="small" @click="openDetectDialog" :loading="detecting">
            检测串口设备
          </el-button>
          <el-button size="small" type="primary" @click="openAddDialog">
            新增设备
          </el-button>
        </div>
      </div>

      <!-- 错误提示 -->
      <el-alert v-if="error" :title="error" type="error" show-icon class="mb-3" />

      <!-- 设备列表 -->
      <el-table :data="config.devices" stripe>
        <el-table-column type="index" label="序号" width="60" />

        <el-table-column label="名称" prop="name" min-width="100" />

        <el-table-column label="角色" width="90">
          <template #default="{ row }">
            <el-tag :type="ROLE_META[row.role as DeviceRole].type" size="small">
              {{ ROLE_META[row.role as DeviceRole].label }}
            </el-tag>
          </template>
        </el-table-column>

        <el-table-column label="端口" prop="port" width="100" />

        <el-table-column label="波特率" prop="baud_rate" width="90" />

        <el-table-column label="状态" width="80">
          <template #default="{ row }">
            <el-tag
              :type="STATUS_TYPE[statusOf(row.device_id)]"
              size="small"
            >
              {{ STATUS_LABEL[statusOf(row.device_id)] }}
            </el-tag>
          </template>
        </el-table-column>

        <el-table-column label="启用" width="65">
          <template #default="{ row }">
            <el-switch
              :model-value="row.enabled"
              @change="(val: boolean) => toggleDevice(row, val)"
            />
          </template>
        </el-table-column>

        <el-table-column label="操作" width="175">
          <template #default="{ row }">
            <el-button size="small" type="info" @click="openMonitorDialog(row)">测试</el-button>
            <el-button size="small" @click="openEditDialog(row)">编辑</el-button>
            <el-button size="small" type="danger" @click="removeDevice(row.device_id)">
              删除
            </el-button>
          </template>
        </el-table-column>
      </el-table>

      <!-- 保存/重置按钮 -->
      <div class="flex gap-2 mt-3">
        <el-button
          type="primary"
          :loading="isLoading"
          @click="handleSave"
        >
          保存配置
        </el-button>
        <el-button :loading="isLoading" @click="handleReset">重置</el-button>
      </div>
    </div>

    <!-- 串口监控对话框 -->
    <el-dialog
      :title="`串口监控 — ${monitorDevice?.name ?? ''} (${monitorDevice?.port ?? ''})`"
      v-model="monitorVisible"
      append-to-body
      width="620px"
      @closed="onMonitorClosed"
    >
      <!-- 状态栏 -->
      <div class="flex items-center gap-3 mb-3">
        <el-tag :type="STATUS_TYPE[monitorStatus]" size="small">
          {{ STATUS_LABEL[monitorStatus] }}
        </el-tag>
        <span class="text-xs text-gray-400">{{ monitorLogs.length }} 条记录</span>
        <el-button size="small" @click="monitorLogs = []">清空</el-button>
      </div>

      <!-- 日志区域 -->
      <div
        ref="logContainer"
        class="h-72 overflow-y-auto rounded p-3 font-mono text-xs bg-gray-900"
      >
        <div v-if="monitorLogs.length === 0" class="text-gray-500 text-center mt-10">
          等待数据输入…
        </div>
        <div
          v-for="(log, i) in monitorLogs"
          :key="i"
          class="leading-5"
        >
          <span class="text-gray-500 select-none">{{ log.time }}</span>
          <span class="mx-2" :class="LOG_COLOR[log.type]">{{ log.content }}</span>
        </div>
      </div>

      <template #footer>
        <el-button @click="monitorVisible = false">关闭</el-button>
      </template>
    </el-dialog>

    <!-- 检测串口设备对话框 -->
    <el-dialog
      title="检测串口设备"
      v-model="detectDialogVisible"
      append-to-body
      width="480px"
    >
      <div v-if="detecting" class="flex justify-center py-6">
        <el-icon class="is-loading text-2xl"><Loading /></el-icon>
      </div>

      <template v-else>
        <div v-if="availablePorts.length === 0" class="text-center text-gray-400 py-6">
          未检测到可用串口设备
        </div>

        <el-table v-else :data="availablePorts" :show-header="false">
          <el-table-column>
            <template #default="{ row: port }">
              <span class="font-mono">{{ port }}</span>
            </template>
          </el-table-column>

          <el-table-column width="100" align="right">
            <template #default="{ row: port }">
              <!-- 该端口已在配置中 -->
              <el-tag v-if="isPortConfigured(port)" type="success" size="small">
                已配置
              </el-tag>
              <!-- 未配置，可添加 -->
              <el-button
                v-else
                size="small"
                type="primary"
                @click="addFromDetected(port)"
              >
                添加
              </el-button>
            </template>
          </el-table-column>
        </el-table>
      </template>

      <template #footer>
        <el-button @click="detectDialogVisible = false">关闭</el-button>
      </template>
    </el-dialog>

    <!-- 新增/编辑对话框 -->
    <el-dialog
      :title="dialogTitle"
      v-model="dialogVisible"
      append-to-body
      width="480px"
    >
      <el-form :model="form" label-width="80px">
        <el-form-item label="设备角色">
          <el-select v-model="form.role" :disabled="isEdit" class="w-full">
            <el-option
              v-for="(meta, role) in ROLE_META"
              :key="role"
              :label="meta.label"
              :value="role"
            />
          </el-select>
        </el-form-item>

        <el-form-item label="名称">
          <el-input v-model="form.name" placeholder="设备显示名称" />
        </el-form-item>

        <el-form-item label="串口">
          <div class="flex gap-2 w-full">
            <el-select
              v-model="form.port"
              filterable
              allow-create
              placeholder="选择或输入端口"
              class="flex-1"
            >
              <el-option
                v-for="p in availablePorts"
                :key="p"
                :label="p"
                :value="p"
              />
            </el-select>
            <el-button size="small" @click="refreshPorts">刷新</el-button>
          </div>
        </el-form-item>

        <el-form-item label="波特率">
          <el-select v-model="form.baud_rate" class="w-full">
            <el-option v-for="b in BAUD_RATES" :key="b" :label="String(b)" :value="b" />
          </el-select>
        </el-form-item>

        <el-form-item label="数据位">
          <el-select v-model="form.data_bits" class="w-full">
            <el-option v-for="b in [5,6,7,8]" :key="b" :label="String(b)" :value="b" />
          </el-select>
        </el-form-item>

        <el-form-item label="停止位">
          <el-select v-model="form.stop_bits" class="w-full">
            <el-option :value="1" label="1" />
            <el-option :value="2" label="2" />
          </el-select>
        </el-form-item>

        <el-form-item label="校验位">
          <el-select v-model="form.parity" class="w-full">
            <el-option value="None" label="无" />
            <el-option value="Odd"  label="奇校验" />
            <el-option value="Even" label="偶校验" />
          </el-select>
        </el-form-item>

        <el-form-item label="启用">
          <el-switch v-model="form.enabled" />
        </el-form-item>
      </el-form>

      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" @click="confirmDialog">保存</el-button>
      </template>
    </el-dialog>

  </div>
</template>

<script setup lang="ts">
import { ref, computed, nextTick, onMounted } from 'vue'
import { storeToRefs } from 'pinia'
import { ElMessageBox } from 'element-plus'
import { Loading } from '@element-plus/icons-vue'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { useSerialStore } from '../store/serialStore'
import {
  type SerialDeviceConfig,
  type DeviceRole,
  type DeviceStatus,
  type SerialDataPayload,
  type SerialErrorPayload,
  type SerialStatusPayload,
  ROLE_META,
  BAUD_RATES,
  createDefaultDevice,
} from '../models/SerialConfig'

// ─── Store ─────────────────────────────────────────────────────────────────

const store = useSerialStore()
const { config, availablePorts, isLoading, error } = storeToRefs(store)
const { statusOf } = store

// ─── PLC Ethernet 配置（独立于串口模块，暂保持本地状态）──────────────────

const plcConfig = ref({ ip: '192.168.0.10', port: 502 })
function savePlc() {
  console.info('PLC 配置保存（待接入 Modbus 模块）:', JSON.parse(JSON.stringify(plcConfig.value)))
}

// ─── 状态样式映射 ───────────────────────────────────────────────────────────

const STATUS_LABEL: Record<DeviceStatus, string> = {
  idle: '空闲',
  connected: '已连接',
  error: '错误',
}

const STATUS_TYPE: Record<DeviceStatus, 'info' | 'success' | 'danger'> = {
  idle: 'info',
  connected: 'success',
  error: 'danger',
}

// ─── 加载 ──────────────────────────────────────────────────────────────────

onMounted(async () => {
  await store.loadConfig()
})

// ─── 串口监控 ─────────────────────────────────────────────────────────────────

type LogType = 'data' | 'error' | 'status'

interface LogEntry {
  time: string
  type: LogType
  content: string
}

const LOG_COLOR: Record<LogType, string> = {
  data:   'text-green-400',
  error:  'text-red-400',
  status: 'text-yellow-400',
}

const monitorVisible = ref(false)
const monitorDevice  = ref<SerialDeviceConfig | null>(null)
const monitorStatus  = ref<DeviceStatus>('idle')
const monitorLogs    = ref<LogEntry[]>([])
const logContainer   = ref<HTMLElement | null>(null)

let unlisteners: UnlistenFn[] = []

async function openMonitorDialog(row: SerialDeviceConfig) {
  monitorDevice.value = row
  monitorLogs.value   = []
  monitorStatus.value = store.statusOf(row.device_id)
  monitorVisible.value = true

  const id = row.device_id

  unlisteners.push(
    await listen<SerialDataPayload>('serial:data', (e) => {
      if (e.payload.device_id !== id) return
      monitorStatus.value = 'connected'
      const content = e.payload.data_str ?? toHex(e.payload.data)
      pushLog('data', content)
    }),
    await listen<SerialErrorPayload>('serial:error', (e) => {
      if (e.payload.device_id !== id) return
      monitorStatus.value = 'error'
      pushLog('error', `错误: ${e.payload.error}`)
    }),
    await listen<SerialStatusPayload>('serial:status', (e) => {
      if (e.payload.device_id !== id) return
      monitorStatus.value = e.payload.connected ? 'connected' : 'idle'
      pushLog('status', e.payload.connected ? '已连接' : '已断开')
    }),
  )
}

function onMonitorClosed() {
  unlisteners.forEach((fn) => fn())
  unlisteners = []
  monitorDevice.value = null
}

function pushLog(type: LogType, content: string) {
  const now = new Date()
  const time = `${now.toTimeString().slice(0, 8)}.${String(now.getMilliseconds()).padStart(3, '0')}`
  monitorLogs.value.push({ time, type, content })
  nextTick(() => {
    if (logContainer.value) {
      logContainer.value.scrollTop = logContainer.value.scrollHeight
    }
  })
}

function toHex(data: number[]): string {
  return data.map((b) => b.toString(16).padStart(2, '0').toUpperCase()).join(' ')
}

// ─── 检测串口设备 ─────────────────────────────────────────────────────────────

const detectDialogVisible = ref(false)
const detecting = ref(false)

/** 当前配置中已使用的端口集合，用于标记"已配置" */
const configuredPorts = computed(
  () => new Set(config.value.devices.map((d) => d.port))
)

function isPortConfigured(port: string) {
  return configuredPorts.value.has(port)
}

async function openDetectDialog() {
  detectDialogVisible.value = true
  detecting.value = true
  await store.fetchAvailablePorts()
  detecting.value = false
}

/** 从检测结果直接带入端口，跳到新增设备表单 */
function addFromDetected(port: string) {
  detectDialogVisible.value = false
  isEdit.value = false
  dialogTitle.value = '新增设备'
  form.value = { ...createDefaultDevice('scanner'), port }
  dialogVisible.value = true
}

async function refreshPorts() {
  await store.fetchAvailablePorts()
}

// ─── 对话框 ─────────────────────────────────────────────────────────────────

const dialogVisible = ref(false)
const isEdit = ref(false)
const dialogTitle = ref('新增设备')

const form = ref<SerialDeviceConfig>(createDefaultDevice('scanner'))

function openAddDialog() {
  isEdit.value = false
  dialogTitle.value = '新增设备'
  form.value = createDefaultDevice('scanner')
  dialogVisible.value = true
}

function openEditDialog(row: SerialDeviceConfig) {
  isEdit.value = true
  dialogTitle.value = '编辑设备'
  form.value = { ...row }
  dialogVisible.value = true
}

function confirmDialog() {
  if (!form.value.port) {
    return
  }
  if (isEdit.value) {
    store.updateDevice(form.value.device_id, form.value)
  } else {
    store.addDevice({ ...form.value })
  }
  dialogVisible.value = false
}

// ─── 设备操作 ───────────────────────────────────────────────────────────────

function toggleDevice(row: SerialDeviceConfig, enabled: boolean) {
  store.updateDevice(row.device_id, { enabled })
}

async function removeDevice(deviceId: string) {
  await ElMessageBox.confirm('确定删除该设备配置？', '提示', {
    confirmButtonText: '删除',
    cancelButtonText: '取消',
    type: 'warning',
  })
  store.removeDevice(deviceId)
}

// ─── 保存 / 重置 ────────────────────────────────────────────────────────────

async function handleSave() {
  await store.saveConfig(store.config)
}

async function handleReset() {
  await ElMessageBox.confirm('重置将清空所有串口设备配置，确认继续？', '提示', {
    confirmButtonText: '重置',
    cancelButtonText: '取消',
    type: 'warning',
  })
  await store.resetConfig()
}
</script>
