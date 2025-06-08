<script setup lang="ts">
import AppLayout from './components/AppLayout.vue';



// 这些原有的API调用逻辑可以移到对应的组件中使用
</script>

<template>
  <div id="app">
    <AppLayout />
  </div>
</template>

<style>
@import './assets/style.css';

#app {
  font-family: Inter, -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}
</style>