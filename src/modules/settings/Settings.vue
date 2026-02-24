<template>
  <div class="settings flex flex-col h-full">
    <div class="flex-1 bg-white p-16px rounded-8px overflow-hidden">
      <el-tabs v-model="activeTab">
        <el-tab-pane label="用户管理" name="users">
          <UserManagement />
        </el-tab-pane>

        <el-tab-pane label="系统设置" name="system">
          <SystemSettings />
        </el-tab-pane>
      </el-tabs>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import UserManagement from './UserManagement.vue';
import SystemSettings from './SystemSettings.vue';

const activeTab = ref('users');
</script>

