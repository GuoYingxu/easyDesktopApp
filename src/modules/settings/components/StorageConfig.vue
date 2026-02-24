<template>
  <div class="storage-config p-4">
    <el-form :model="storageConfig" label-width="120px">
      <el-form-item label="目录路径">
        <el-input v-model="storageConfig.path" placeholder="E:/data/storage" />
      </el-form-item>
      <el-form-item>
        <el-button size="small" @click="chooseStorageDir">选择目录</el-button>
      </el-form-item>
      <el-form-item>
        <el-button type="primary" @click="saveStorageConfig">保存存储配置</el-button>
      </el-form-item>
    </el-form>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

const storageConfig = ref({
  path: 'E:/data/storage'
});

function chooseStorageDir() {
  // placeholder: in a real Tauri/Electron app open native dialog
  console.info('choose storage dir (native dialog not implemented)');
}

function saveStorageConfig() {
  console.info('Save storage config', JSON.parse(JSON.stringify(storageConfig.value)));
}
</script>

<style scoped>
.p-4 { padding: 16px }
</style>