<template>
  <div class="system-integration p-4">
    <el-form :model="mesConfig" label-width="120px">
      <el-form-item label="启用 MES">
        <el-switch v-model="mesConfig.enabled" />
      </el-form-item>
      <el-form-item label="MES 地址">
        <el-input v-model="mesConfig.url" placeholder="http://mes.example/api" />
      </el-form-item>
      <el-form-item label="访问令牌">
        <el-input v-model="mesConfig.token" />
      </el-form-item>
      <el-form-item>
        <el-button type="primary" @click="saveMesConfig">保存 MES 配置</el-button>
        <el-button type="info" @click="testMesConnection">测试连接</el-button>
      </el-form-item>
    </el-form>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

const mesConfig = ref({ enabled: false, url: '', token: '' });

function saveMesConfig() { 
  console.info('Save MES config', JSON.parse(JSON.stringify(mesConfig.value))); 
}

function testMesConnection() { 
  console.info('Test MES connection to', mesConfig.value.url); 
}
</script>

<style scoped>
.text-md { font-size: 14px }
.font-medium { font-weight: 500 }
.mb-2 { margin-bottom: 8px }
.mb-4 { margin-bottom: 16px }
.p-4 { padding: 16px }
</style>