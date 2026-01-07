<template>
  <nav class="menu">
    <router-link to="/detection" class="menu-link" aria-label="实时检测">
      <span class="icon">
        <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><circle cx="11" cy="11" r="6" stroke="currentColor" stroke-width="1.6"/><path d="M21 21l-4.35-4.35" stroke="currentColor" stroke-width="1.6" stroke-linecap="round"/></svg>
      </span>
      <span class="label">实时检测</span>
    </router-link>

    <router-link to="/statistics" class="menu-link" aria-label="数据统计">
      <span class="icon">
        <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M4 20v-8" stroke="currentColor" stroke-width="1.6" stroke-linecap="round"/><path d="M10 20V8" stroke="currentColor" stroke-width="1.6" stroke-linecap="round"/><path d="M16 20v-4" stroke="currentColor" stroke-width="1.6" stroke-linecap="round"/><path d="M22 20v-2" stroke="currentColor" stroke-width="1.6" stroke-linecap="round"/></svg>
      </span>
      <span class="label">数据统计</span>
    </router-link>

    <router-link to="/config" class="menu-link" aria-label="检测配置">
      <span class="icon">
        <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12 15.5a3.5 3.5 0 1 0 0-7 3.5 3.5 0 0 0 0 7z" stroke="currentColor" stroke-width="1.6"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09a1.65 1.65 0 0 0-1-1.51 1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09c.67 0 1.23-.4 1.51-1a1.65 1.65 0 0 0-.33-1.82L3.3 5.6A2 2 0 0 1 6.13 2.77l.06.06c.51.48 1.12.8 1.82.92.38.07.72-.03 1-.24.28-.2.5-.5.6-.86.1-.36.24-.7.44-1.01C10.37.8 10.67.5 11 .5c.33 0 .63.3.78.71.2.31.34.65.44 1.01.1.36.32.66.6.86.28.21.62.31 1 .24.7-.12 1.31-.44 1.82-.92l.06-.06A2 2 0 0 1 20.7 5.6l-.06.06c-.66.66-1 1.54-1 2.45v.09c0 .7.3 1.36.78 1.78.39.35.86.54 1.37.54H21a2 2 0 0 1 0 4h-.09c-.67 0-1.23.4-1.51 1z" stroke="currentColor" stroke-width="1"/></svg>
      </span>
      <span class="label">检测配置</span>
    </router-link>

    <router-link to="/history" class="menu-link" aria-label="历史记录">
      <span class="icon">
        <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M21 12a9 9 0 1 1-9-9" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"/><path d="M12 7v5l3 3" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"/></svg>
      </span>
      <span class="label">历史记录</span>
    </router-link>

    <router-link to="/alerts" class="menu-link" aria-label="报警管理">
      <span class="icon">
        <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M18 8a6 6 0 0 0-12 0c0 7-3 8-3 8h18s-3-1-3-8" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"/><path d="M13.73 21a2 2 0 0 1-3.46 0" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"/></svg>
      </span>
      <span class="label">报警管理</span>
    </router-link>

    <router-link to="/settings" class="menu-link" aria-label="系统设置">
      <span class="icon">
        <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12 15.5a3.5 3.5 0 1 0 0-7 3.5 3.5 0 0 0 0 7z" stroke="currentColor" stroke-width="1.6"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09a1.65 1.65 0 0 0-1-1.51 1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09c.67 0 1.23-.4 1.51-1a1.65 1.65 0 0 0-.33-1.82L3.3 5.6A2 2 0 0 1 6.13 2.77l.06.06c.51.48 1.12.8 1.82.92.38.07.72-.03 1-.24.28-.2.5-.5.6-.86.1-.36.24-.7.44-1.01C10.37.8 10.67.5 11 .5c.33 0 .63.3.78.71.2.31.34.65.44 1.01.1.36.32.66.6.86.28.21.62.31 1 .24.7-.12 1.31-.44 1.82-.92l.06-.06A2 2 0 0 1 20.7 5.6l-.06.06c-.66.66-1 1.54-1 2.45v.09c0 .7.3 1.36.78 1.78.39.35.86.54 1.37.54H21a2 2 0 0 1 0 4h-.09c-.67 0-1.23.4-1.51 1z" stroke="currentColor" stroke-width="1"/></svg>
      </span>
      <span class="label">系统设置</span>
    </router-link>
  </nav>
</template>

<script setup lang="ts">
</script>

<style scoped>
.menu { display:flex; flex-direction:column; gap:12px; align-items:stretch; width:100% }
.menu-link { display:flex; flex-direction:column; align-items:center; justify-content:center; width:100%; height:88px; color: #DCEAFB; text-decoration:none;  padding:8px; box-sizing:border-box; background:transparent; transition: background 160ms ease, transform 160ms ease }
.menu-link .icon { width:40px; height:40px; display:inline-flex; align-items:center; justify-content:center }
.menu-link svg { width:100%; height:100%; color: #C6D9FF }
.menu-link .label { margin-top:8px; font-size:0.78rem; color:#E8F3FF }
.menu-link.router-link-active { background: linear-gradient(180deg, rgba(255,255,255,0.12), rgba(255,255,255,0.06)); color: #ffffff; box-shadow: 0 6px 18px rgba(0,0,0,0.12) }
.menu-link.router-link-active svg { color: #ffffff }

/* hover / focus */
.menu-link:hover, .menu-link:focus { background: rgba(255,255,255,0.06); transform: translateY(-2px) }

@media (max-width: 800px) {
  .menu { flex-direction:row; padding:8px }
  .menu-link { width:auto; height:64px; flex-direction:row; gap:10px; justify-content:flex-start; padding:10px }
  .menu-link .icon { width:28px; height:28px }
  .menu-link .label { margin-top:0; margin-left:8px }
}
</style>
