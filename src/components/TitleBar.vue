<template>
  <header class="titlebar">
    <div class="titlebar-left drag-area">
      <img src="/src/assets/titleLogo.svg" alt="logo" class="titlebar-logo" />
      <span class="titlebar-title">冰箱外观检测系统</span>
    </div>
    <div class="titlebar-right no-drag">
      <button class="tb-btn" @click="minimize">_</button>
      <button class="tb-btn" @click="toggleMaximize">▢</button>
      <button class="tb-btn close" @click="close">✕</button>
    </div>
  </header>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'

const maximized = ref(false)
async function updateMaximized() {
  try {
    maximized.value = await getCurrentWindow().isMaximized()
  } catch {
    maximized.value = false
  }
}

onMounted(() => {
  updateMaximized()
  // listen for changes (best-effort)
  try {
    const appWindow = getCurrentWindow()
    // @ts-ignore
    appWindow.listen && appWindow.listen('tauri://resize', () => updateMaximized())
  } catch {}
})

async function minimize() {
  await getCurrentWindow().minimize()
}

async function toggleMaximize() {
  console.log('toggleMaximize')
  const appWindow = getCurrentWindow()
  if (await appWindow.isMaximized()) {
    await appWindow.unmaximize()
  } else {
    await appWindow.maximize()
  }
  updateMaximized()
}

async function close() {
  await getCurrentWindow().close()
}
</script>

<style scoped>
.titlebar{
  height: 36px;
  display:flex;
  flex-direction: row;
  align-items:center;
  justify-content:start;
  padding: 0 8px;
  background:  white ; 
  color: #2c2c2c;
  -webkit-user-select: none;
  border-bottom: 1px solid #e6e6e6;
}
.drag-area{ -webkit-app-region: drag; display:flex; flex:1; align-items:center; gap:8px }
.no-drag{ -webkit-app-region: no-drag; display:flex; gap:6px }
.titlebar-logo{ width:20px; height:20px }
.titlebar-title{ font-weight:400; font-family: "Microsoft YaHei", sans-serif }
.tb-btn{ font-weight: bold; background: transparent; border: none; color: #050505; width:36px; height:28px; cursor:pointer }
.tb-btn.close{ font-weight: bold; color: #000000 }
</style>
