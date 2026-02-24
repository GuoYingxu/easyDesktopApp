<template>
  <div class="logging-config p-4">
    <el-form :model="logConfig" label-width="150px">
      <el-form-item label="日志等级">
        <el-select v-model="logConfig.level">
          <el-option label="DEBUG" value="DEBUG" />
          <el-option label="INFO" value="Info" />
          <el-option label="WARN" value="WARN" />
          <el-option label="ERROR" value="ERROR" />
        </el-select>
      </el-form-item>
      <el-form-item label="日志文件大小(MB)">
        <el-input v-model.number="logFileSizeMB" />
      </el-form-item>
      <el-form-item label="保存日志数量">
        <el-input v-model.number="logConfig.keep_files" />
      </el-form-item>
      <el-form-item>
        <el-checkbox v-model="logConfig.to_stdout">输出到控制台</el-checkbox>
      </el-form-item>
      <el-form-item>
        <el-button
          type="primary"
          @click="saveLogConfig"
          :loading="isLoading"
          :disabled="isLoading"
        >
          保存日志配置
        </el-button>
        <el-button @click="resetLogConfig" :loading="isLoading" :disabled="isLoading">
          重置为默认
        </el-button>
        <span v-if="showRestartNotice" class="restart-notice">配置已保存，重启程序后生效</span>
        <el-button v-if="showRestartNotice" @click="restartApp" type="success" :loading="restarting" :disabled="restarting">
          {{ restarting ? '正在重启...' : '立即重启' }}
        </el-button>
        <span v-if="error" class="error-message">{{ error }}</span>
      </el-form-item>
    </el-form>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { useLogStore } from '../store/logStore';
import { storeToRefs } from 'pinia';
import { relaunch } from '@tauri-apps/plugin-process';

const logStore = useLogStore();
const {logConfig, error,isLoading} = storeToRefs(logStore)
const showRestartNotice = ref(false);
const restarting = ref(false);

onMounted(async () => {
  console.log('Loading log config...');
  await logStore.loadLogConfig();
  console.log('Log config loaded:', logStore.logConfig);
});


// Computed property to handle file size conversion between bytes and MB
const logFileSizeMB = computed({
  get() {
    if (logStore.logConfig.max_file_size === null) {
      return null;
    }
    return Math.round(logStore.logConfig.max_file_size / (1024 * 1024));
  },
  set(value: number | null) {
    if (value === null || value <= 0) {
      logStore.setLogMaxFileSize(null);
    } else {
      logStore.setLogMaxFileSize(value * 1024 * 1024);
    }
  }
});

async function saveLogConfig() {
  try {
    // Prepare the config object with the right structure
    const config = {
      ...logStore.logConfig,
      max_file_size: logStore.logConfig.max_file_size
    };

    await logStore.updateLogConfig(config);
    // Show restart notice after successful save
    showRestartNotice.value = true;
  } catch (error) {
    console.error('Failed to save log config:', error);
  }
}

async function resetLogConfig() {
  await logStore.resetLogConfig();
  // Show restart notice after reset
  showRestartNotice.value = true;
}

async function restartApp() {
  restarting.value = true;
  try {
    await relaunch();
  } catch (err) {
    console.error('Failed to restart app:', err);
    restarting.value = false;
  }
}
</script>

<style scoped>
.p-4 { padding: 16px }
.error-message {
  color: #f56c6c;
  margin-left: 10px;
}
.restart-notice {
  color: #e6a23c;
  margin-left: 10px;
  font-size: 14px;
}
</style>