<script setup lang="ts">
import { defineAsyncComponent, onMounted, provide } from 'vue';
import { usePlatform } from './composables/usePlatform';
import { useGlobalAuth } from './composables/useGlobalAuth';

const { isMacOS } = usePlatform();
const { 
  isLoggedIn, 
  userInfo, 
  loadUserProfile, 
  checkLoginStatus, 
  setupOAuthListener, 
  login, 
  logout,
  avatarUrl
} = useGlobalAuth();

// Provide global state for deep injection (views, etc.)
provide('userInfo', userInfo);
provide('isLoggedIn', isLoggedIn);
provide('refreshUserInfo', loadUserProfile);
provide('authActions', { login, logout, avatarUrl });

// Lazy load layouts
const WindowsLayout = defineAsyncComponent(() => import('./components/WindowsLayout.vue'));
const MacOSLayout = defineAsyncComponent(() => import('./components/MacOSLayout.vue'));

onMounted(() => {
  checkLoginStatus();
  setupOAuthListener();
});
</script>

<template>
  <div id="app" :class="{ 'platform-macos': isMacOS, 'platform-windows': !isMacOS }">
    <component :is="isMacOS ? MacOSLayout : WindowsLayout" />
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
