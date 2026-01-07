<template>
  <div class="app-layout">
    <aside class="sidebar">
      <Logo />
      <AppMenu />
    </aside>

    <div class="main-area">
      <TitleBar />
      <header class="topbar">
        <div class="pageTop">
          <div class="pageTitle">{{ $route.meta.title }}</div>
          <div class="pageSubTitle">冰箱门外观质量检测系统</div>
        </div>
        <div class="PageStatus">
          <div class="status">
            <RunStatus />
          </div>
          <div class="auth">
            <LoginStatus />
          </div>
        </div>
      </header>

      <main class="content">
        <router-view />
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import Logo from "./Logo.vue";
import AppMenu from "./AppMenu.vue";
import RunStatus from "./RunStatus.vue";
import LoginStatus from "./LoginStatus.vue";
import TitleBar from "./TitleBar.vue";
</script>

<style scoped>
.app-layout { display: flex; height: 100vh; }
.sidebar { width: 100px; background: #2B63D1; color: #fff; display:flex; flex-direction:column; padding: 0; box-sizing: border-box }
.main-area { flex: 1; display:flex; flex-direction:column; }
.topbar { height: 80px; display:flex; align-items:center; justify-content:space-between; padding: 0 16px; border-bottom:1px solid #e6e6e6; background: #fff }
.content { padding: 16px; overflow:auto; }
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
