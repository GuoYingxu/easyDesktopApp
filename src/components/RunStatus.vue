<template>
  <div class="run-status">

    <div >
      <div> 运行状态： </div>
      <div>{{ running ? '系统运行中' : '系统未启动' }}</div>
    </div>
    <el-tag :type="running ? 'success' : 'info'" effect="dark">
      <template v-if="running">
        系统运行中
      </template>
      <template v-else>
       系统未启动
      </template>
    </el-tag>
    
    <el-button 
      v-if="running"
      type="danger" 
      size="small"
      @click="handleShutdown"
    >
      关机
    </el-button>
    
    <el-button 
      v-else
      type="primary" 
      size="small"
      @click="handleStartup"
    >
      开机
    </el-button>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { ElTag, ElButton } from 'element-plus'
import { ElMessageBox } from 'element-plus'

const running = ref(false)

async function handleStartup() {
  running.value = true
  console.log('System started')
}

async function handleShutdown() {
  try {
    await ElMessageBox.confirm(
      '确认要关闭系统吗？',
      '关机确认',
      {
        confirmButtonText: '确认',
        cancelButtonText: '取消',
        type: 'warning',
      }
    )
    running.value = false
    console.log('System shutdown confirmed')
  } catch {
    console.log('Shutdown cancelled')
  }
}
</script>

<style scoped>
.run-status { 
  display: flex; 
  align-items: center; 
  gap: 12px;
  padding: 8px 12px;
  background: rgba(255, 255, 255, 0.04);
  border-radius: 6px;
}
</style>
