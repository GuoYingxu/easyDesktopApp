<template>
  <nav class="menu">
    <router-link to="/detection" class="menu-link" aria-label="实时检测">
      <span class="icon">
        <img src="../assets/Vector.svg">
      </span>
      <span class="label">缺陷检测</span>
    </router-link>

    <router-link to="/statistics" class="menu-link" aria-label="数据统计">
      <span class="icon">
        <img src="../assets/li_history.svg">
      </span>
      <span class="label">数据统计</span>
    </router-link>

    <router-link to="/config" class="menu-link" aria-label="检测配置">
      <span class="icon">
        <img src="../assets/setting.svg">
      </span>
      <span class="label">检测设置</span>
    </router-link>

    <router-link to="/alerts" class="menu-link" aria-label="产品信息">
      <span class="icon">
        <img src="../assets/production_info.svg"></img>
      </span>
      <span class="label">产品信息</span>
    </router-link>

    <router-link to="/settings" class="menu-link" aria-label="系统设置">
      <span class="icon">
        <img src="../assets/Systemsetting.svg">
      </span>
      <span class="label">系统设置</span>
    </router-link>
  </nav>
</template>

<script setup lang="ts">
</script>

<style scoped>
.menu { flex :1 ;display:flex; flex-direction:column; gap:12px; align-items:stretch; width:100%;border-right: 1px solid #e6e6e6; }
.menu-link { display:flex; flex-direction:column; align-items:center; justify-content:center; width:100%; height:88px; color: #DCEAFB; text-decoration:none;  padding:8px; box-sizing:border-box; background:transparent; transition: background 160ms ease, transform 160ms ease }
.menu-link .icon { width:40px; height:40px; display:inline-flex; align-items:center; justify-content:center }
.menu-link svg { width:100%; height:100%; color: #C6D9FF }
.menu-link .label { margin-top:8px; font-size:0.78rem; color:#4e4f4f }
.menu-link.router-link-active { background: white; color: #e00808; font-weight: bold; }
.menu-link.router-link-active .label{ background: white; color: #000000; font-size:14px; font-weight: bold; }
/* hover / focus */
.menu-link:hover, .menu-link:focus { background: #ffffff; transform: translateY(-2px);font-weight: bold; }

@media (max-width: 800px) {
  .menu { flex-direction:row; padding:8px }
  .menu-link { width:auto; height:64px; flex-direction:row; gap:10px; justify-content:flex-start; padding:10px }
  .menu-link .icon { width:28px; height:28px }
  .menu-link .label { margin-top:0; margin-left:8px }
}
</style>
