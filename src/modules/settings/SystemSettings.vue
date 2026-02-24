<template>
  <div class="system-settings">
    <el-tabs v-model="systemTab" type="border-card">
      <el-tab-pane label="数据库配置" name="database">
        <DatabaseConfig />
      </el-tab-pane>

      <el-tab-pane label="日志配置" name="logging">
        <LoggingConfig />
      </el-tab-pane>

      <el-tab-pane label="存储配置" name="storage">
        <StorageConfig />
      </el-tab-pane>

      <el-tab-pane label="硬件管理" name="hardware">
        <HardwareConfig />
      </el-tab-pane>

      <el-tab-pane label="系统集成" name="integration">
        <SystemIntegration />
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import DatabaseConfig from './components/DatabaseConfig.vue';
import LoggingConfig from './components/LoggingConfig.vue';
import StorageConfig from './components/StorageConfig.vue';
import HardwareConfig from './components/HardwareConfig.vue';
import SystemIntegration from './components/SystemIntegration.vue';

const systemTab = ref('database');
</script>
