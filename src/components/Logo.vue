<template>
  <div class="logo">
    <img src="/src/assets/logo.webp" alt="Haire logo" class="logo-img" />
  </div>
</template>

<script setup lang="ts">
</script>

<style scoped>
.logo { display:flex; align-items:start;justify-content: center; gap:12px; margin-top: 5px;  }
.logo-img { width:calc(100% - 10px); height:auto; object-fit:contain }
</style>
