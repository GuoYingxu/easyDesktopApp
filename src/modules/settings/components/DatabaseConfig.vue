<template>
  <div class="database-config p-4">
    <el-form :model="dbConfig" label-width="120px">
      <el-form-item label="主机">
        <el-input v-model="dbConfig.host" placeholder="127.0.0.1" />
      </el-form-item>
      <el-form-item label="端口">
        <el-input v-model="dbConfig.port" />
      </el-form-item>
      <el-form-item label="用户名">
        <el-input v-model="dbConfig.user" />
      </el-form-item>
      <el-form-item label="密码">
        <el-input v-model="dbConfig.password" show-password />
      </el-form-item>
      <el-form-item label="数据库名">
        <el-input v-model="dbConfig.database" />
      </el-form-item>
      <el-form-item>
        <el-button type="primary" @click="saveDbConfig">保存数据库配置</el-button>
      </el-form-item>
    </el-form>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

const dbConfig = ref({
  host: '127.0.0.1',
  port: 5432,
  user: 'sa',
  password: '',
  database: 'easydb'
});

function saveDbConfig() {
  console.info('Save DB config', JSON.parse(JSON.stringify(dbConfig.value)));
}
</script>

<style scoped>
.p-4 { padding: 16px }
</style>