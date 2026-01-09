<template>
  <div class="app-layout">
    <div class="flex-none h-37px ">
      <TitleBar />
    </div>
    <div class="flex flex-row flex-1 max-h-full overflow-hidden">
      <aside class="sidebar">
        <Logo />
        <AppMenu />
      </aside>

      <div class="flex flex-col overflow-hidden flex-1">
        <header class="topbar">
          <Header></Header>
        </header>
        <main class="content">
          <router-view />
        </main>
      </div>
  </div>
</div>
</template>

<script setup lang="ts">
import Logo from "./Logo.vue";
import AppMenu from "./AppMenu.vue";
// import RunStatus from "./RunStatus.vue";
// import LoginStatus from "./LoginStatus.vue";
import Header from "./Header.vue";

import TitleBar from "./TitleBar.vue";
</script>

<style scoped>
.app-layout { display: flex; flex-direction: column; height: 100vh;overflow: hidden; }
.sidebar { width: 100px; background: #f7f7f7; color: #000000; display:flex; flex-direction:column; padding: 0; box-sizing: border-box }
.main-area { flex: 0; display:flex; flex-direction:column; max-height:100%;overflow: hidden; }
.topbar { flex:none; height: 80px; display:flex; align-items:center; justify-content:space-between; padding: 0 16px; border-bottom:1px solid #e6e6e6; background: #fff }
.content { padding: 16px; overflow:hidden; flex:1;max-height: 100%; display:flex; flex-direction:column; }
.status { display:flex; align-items:center; gap:12px }
.auth { display:flex; align-items:center }
/* Page title font */
.pageTop { margin-left: 10px; display: flex; flex-direction: column; gap: 10px;}
/* .content h1 { font-family: Inter, "Microsoft YaHei", "微软雅黑", Arial, sans-serif; } */
.pageTitle { font-size: 24px; font-weight: 600; color: #333 }
.pageSubTitle { font-size: 16px; color: #666 }
@media (prefers-color-scheme: dark) {
  .main-area .topbar { background: #0b1220; border-bottom-color: rgba(255,255,255,0.04); }
  .content { background: #071025; color: #e6eef8 }
}
</style>
