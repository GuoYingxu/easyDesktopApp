<template>
  <div class="login-status">
    <template v-if="user">
      <span class="user">{{ user }}</span>
      <button @click="logout">Logout</button>
    </template>
    <template v-else>
      <button @click="login">Login</button>
    </template>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
const user = ref<string | null>(null)
function login() { user.value = 'User' }
function logout() { user.value = null }
</script>

<style scoped>
.login-status { display:flex; align-items:center; gap:8px }
.user { font-weight:600 }
button { padding:4px 8px; border-radius:6px }
</style>
