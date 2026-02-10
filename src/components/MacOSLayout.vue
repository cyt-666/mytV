<template>
  <div class="macos-layout">
    <!-- Sidebar (Vibrant Material) -->
    <aside class="macos-sidebar">
      <!-- Traffic Lights & Toolbar Region -->
      <div class="window-drag-region" data-tauri-drag-region>
        <div 
          v-if="showGlobalBackButton"
          class="macos-titlebar-back-btn"
          @click.stop="handleGlobalBack"
        >
          <icon-left />
        </div>
      </div>

      <!-- Search Field -->
      <div class="sidebar-search-wrapper">
        <div class="macos-search-input">
          <icon-search class="search-icon" />
          <input 
            v-model="searchQuery" 
            placeholder="搜索" 
            @keydown.enter="handleSearch"
          />
        </div>
      </div>

      <!-- Navigation Menu -->
      <div class="sidebar-scroll-area">
        <!-- Section: Discovery -->
        <div class="sidebar-group">
          <div class="group-label">发现</div>
          <div 
            class="sidebar-item" 
            :class="{ active: isActive('/') }"
            @click="navTo('/')"
          >
            <icon-home class="item-icon" />
            <span class="item-label">首页</span>
          </div>
          <div 
            class="sidebar-item" 
            :class="{ active: isActive('/movies') }"
            @click="navTo('/movies')"
          >
            <icon-star class="item-icon" />
            <span class="item-label">电影</span>
          </div>
          <div 
            class="sidebar-item" 
            :class="{ active: isActive('/shows') }"
            @click="navTo('/shows')"
          >
            <icon-play-circle class="item-icon" />
            <span class="item-label">电视剧</span>
          </div>
          <div 
            class="sidebar-item" 
            :class="{ active: isActive('/browse') }"
            @click="navTo('/browse')"
          >
            <icon-apps class="item-icon" />
            <span class="item-label">分类</span>
          </div>
        </div>

        <!-- Section: Library -->
        <div class="sidebar-group">
          <div class="group-label">我的</div>
          <div 
            class="sidebar-item" 
            :class="{ active: isActive('/up-next') }"
            @click="navTo('/up-next')"
          >
            <icon-clock-circle class="item-icon" />
            <span class="item-label">待看</span>
          </div>
          <div 
            class="sidebar-item" 
            :class="{ active: isActive('/watchlist') }"
            @click="navTo('/watchlist')"
          >
            <icon-bookmark class="item-icon" />
            <span class="item-label">观看清单</span>
          </div>
          <div 
            class="sidebar-item" 
            :class="{ active: isActive('/history') }"
            @click="navTo('/history')"
          >
            <icon-history class="item-icon" />
            <span class="item-label">观看历史</span>
          </div>
          <div 
            class="sidebar-item" 
            :class="{ active: isActive('/collection') }"
            @click="navTo('/collection')"
          >
            <icon-folder class="item-icon" />
            <span class="item-label">我的片库</span>
          </div>
          <div 
            class="sidebar-item" 
            :class="{ active: isActive('/calendar') }"
            @click="navTo('/calendar')"
          >
            <icon-calendar class="item-icon" />
            <span class="item-label">剧集日历</span>
          </div>
        </div>
      </div>

      <!-- User Profile (Bottom) -->
      <div class="sidebar-footer">
        <a-dropdown trigger="click" :disabled="!isLoggedIn" @select="handleMenuSelect">
          <div class="user-profile-card" @click="handleGuestClick">
             <a-avatar :size="32" class="user-avatar">
                <img v-if="avatarUrl" :src="avatarUrl" referrerpolicy="no-referrer"/>
                <icon-user v-else />
             </a-avatar>
             <div class="user-info">
               <div class="user-name">{{ userInfo?.username || '游客' }}</div>
               <div class="user-status">{{ isLoggedIn ? '在线' : '点击登录' }}</div>
             </div>
             <icon-settings class="settings-icon" @click.stop="navTo('/settings')" />
          </div>
          <template #content>
            <a-doption value="profile">
              <template #icon><icon-user /></template>
              个人中心
            </a-doption>
            <a-doption value="logout">
              <template #icon><icon-export /></template>
              退出登录
            </a-doption>
          </template>
        </a-dropdown>
      </div>
    </aside>

      <!-- Main Content -->
    <main class="macos-main">
      <!-- Main Content Drag Region -->
      <div class="main-drag-region" data-tauri-drag-region></div>
      
       <div class="content-scroll-container" ref="contentRef">
          <router-view v-slot="{ Component, route }">
            <keep-alive v-if="shouldKeepAlive(route)">
              <component :is="Component" :key="route.path" />
            </keep-alive>
            <component v-else :is="Component" :key="route.fullPath" />
          </router-view>
       </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, inject, watch, nextTick, computed, onMounted, onBeforeUnmount } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import type { Ref } from 'vue';
import type { User } from '../types/api';
import { 
  IconHome, IconStar, IconPlayCircle, IconApps, 
  IconClockCircle, IconBookmark, IconHistory, IconFolder, IconCalendar,
  IconSearch, IconUser, IconSettings, IconExport, IconLeft
} from '@arco-design/web-vue/es/icon';

const router = useRouter();
const route = useRoute();
const contentRef = ref();

// Injected State
const userInfo = inject<Ref<User | null>>('userInfo', ref(null));
const isLoggedIn = inject<Ref<boolean>>('isLoggedIn', ref(false));
const authActions = inject<any>('authActions', {});
const { login, logout, avatarUrl } = authActions;

const searchQuery = ref('');

const showGlobalBackButton = computed(() => {
  if (['/', '/search'].includes(route.path)) return false;
  return window.history.length > 1;
});

const handleGlobalBack = () => {
  router.back();
};

const navTo = (path: string) => {
  router.push(path);
};

const isActive = (path: string) => {
  if (path === '/' && route.path === '/') return true;
  if (path !== '/' && route.path.startsWith(path)) return true;
  return false;
};

const handleSearch = () => {
  if (searchQuery.value.trim()) {
    router.push({ name: 'search', query: { q: searchQuery.value.trim() } });
  }
};

const handleGuestClick = () => {
  if (!isLoggedIn.value) {
    login();
  }
};

const handleMenuSelect = (value: any) => {
  if (value === 'profile') {
    navTo('/profile');
  } else if (value === 'logout') {
    if (typeof logout === 'function') logout();
  }
};

const shouldKeepAlive = (route: any) => {
  if (route.path.startsWith('/movie/') || route.path.startsWith('/show/')) return false;
  return ['/', '/search', '/watchlist', '/collection', '/history', '/profile', '/browse', '/movies', '/shows'].includes(route.path);
};

// Scroll to top on route change
watch(route, async () => {
  await nextTick();
  if (contentRef.value) {
    contentRef.value.scrollTop = 0;
  }
});

const handleKeyDown = (event: KeyboardEvent) => {
  if (event.altKey && event.key === 'ArrowLeft') {
    event.preventDefault();
    if (showGlobalBackButton.value) {
      handleGlobalBack();
    }
  }
};

onMounted(() => {
  document.addEventListener('keydown', handleKeyDown);
});

onBeforeUnmount(() => {
  document.removeEventListener('keydown', handleKeyDown);
});
</script>

<style scoped>
.macos-layout {
  /* Traffic Lights Metrics */
  --macos-traffic-lights-safe-width: 72px;
  --macos-traffic-lights-gap: 14px;
  --macos-back-btn-y-offset: 3px;

  display: flex;
  height: 100vh;
  width: 100%;
  background: transparent;
  font-family: -apple-system, BlinkMacSystemFont, "SF Pro Text", "Helvetica Neue", sans-serif;
  overflow: hidden;
  overflow-x: hidden;
}

/* Sidebar */
.macos-sidebar {
  width: 260px;
  background-color: rgba(255, 255, 255, 0.1); /* Subtle tint over vibrancy */
  border-right: 1px solid var(--glass-border);
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
  user-select: none;
  /* padding-bottom: 16px; Removed to allow footer to be a full bar */
  overflow-x: hidden;
}

.window-drag-region {
  height: 52px; /* Top area for traffic lights */
  width: 100%;
  -webkit-app-region: drag;
  display: flex;
  align-items: center;
  padding-left: calc(var(--macos-traffic-lights-safe-width) + var(--macos-traffic-lights-gap));
}

.macos-titlebar-back-btn {
  height: 28px;
  padding: 0 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 14px; /* Capsule shape */
  color: var(--glass-text);
  font-size: 14px;
  cursor: default; /* Keep default cursor for toolbar items typically */
  -webkit-app-region: no-drag;
  transition: background 0.2s, color 0.2s;
  background: transparent;
  border: none;
  transform: translateY(var(--macos-back-btn-y-offset));
}

.macos-titlebar-back-btn:hover {
  background: var(--macos-toolbar-hover-bg);
}

.macos-titlebar-back-btn:active {
  background: var(--macos-toolbar-active-bg);
}

@media (max-width: 700px) {
  .macos-titlebar-back-btn {
    display: none;
  }
}

/* Search */
.sidebar-search-wrapper {
  padding: 0 16px 16px 24px;
}

.macos-search-input {
  background: rgba(0, 0, 0, 0.05);
  border-radius: 8px;
  height: 32px;
  display: flex;
  align-items: center;
  padding: 0 8px;
  transition: background 0.2s, box-shadow 0.2s;
  border: 1px solid var(--glass-border);
}

.macos-search-input:focus-within {
  background: var(--glass-bg-elevated);
  box-shadow: 0 0 0 2px rgba(0, 122, 255, 0.3); /* macOS Focus Ring */
}

.search-icon {
  color: var(--glass-text-secondary);
  margin-right: 6px;
  font-size: 14px;
}

.macos-search-input input {
  border: none;
  background: transparent;
  flex: 1;
  font-size: 13px;
  color: var(--glass-text);
  outline: none;
}
.macos-search-input input::placeholder {
  color: var(--glass-text-secondary);
}

/* Menu */
.sidebar-scroll-area {
  flex: 1;
  overflow-y: auto;
  padding: 0 12px;
}

.sidebar-group {
  margin-bottom: 24px;
}

.group-label {
  font-size: 11px;
  font-weight: 600;
  color: var(--glass-text-secondary);
  margin-bottom: 6px;
  padding-left: 12px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.sidebar-item {
  display: flex;
  align-items: center;
  height: 34px;
  padding: 0 12px;
  margin-bottom: 2px;
  border-radius: 6px;
  cursor: default;
  color: var(--glass-text);
  transition: background 0.15s;
}

.sidebar-item:hover {
  background-color: rgba(0, 0, 0, 0.04);
}

.sidebar-item.active {
  background-color: rgba(0, 0, 0, 0.08); /* macOS Active State (Inactive Window) */
  /* If window active: background-color: #007aff; color: #fff; */
  /* Since we can't easily detect window focus state in CSS, we use a neutral active state */
  color: var(--glass-text);
  font-weight: 500;
}

.item-icon {
  font-size: 16px;
  margin-right: 10px;
  opacity: 0.8;
}

.sidebar-item.active .item-icon {
  opacity: 1;
  color: #007aff;
}

.item-label {
  font-size: 13px;
}

/* Footer */
.sidebar-footer {
  padding: 12px 16px;
  border-top: 1px solid var(--glass-border);
  background: var(--glass-bg); /* Use glass variable for consistency */
  backdrop-filter: var(--glass-blur); /* Ensure consistent blur */
}

.user-profile-card {
  display: flex;
  align-items: center;
  padding: 8px;
  border-radius: 8px;
  cursor: pointer;
  transition: background 0.2s;
}

.user-profile-card:hover {
  background: rgba(0,0,0,0.04);
}

.user-avatar {
  background-color: #ccc;
  flex-shrink: 0;
}

.user-info {
  margin-left: 10px;
  flex: 1;
  overflow: hidden;
}

.user-name {
  font-size: 13px;
  font-weight: 600;
  color: var(--glass-text);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.user-status {
  font-size: 11px;
  color: #888;
}

.settings-icon {
  color: #999;
  font-size: 16px;
  padding: 4px;
  border-radius: 4px;
}
.settings-icon:hover {
  background: rgba(0,0,0,0.1);
  color: #333;
}

/* Main Content */
.macos-main {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: transparent; /* Changed from var(--glass-bg) to transparent to show vibrancy */
  position: relative;
  min-width: 0;
  overflow: hidden;
  overflow-x: hidden;
}

.main-drag-region {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 52px;
  z-index: 5;
  -webkit-app-region: drag;
}

.content-scroll-container {
  flex: 1;
  width: 100%;
  overflow-y: auto;
  overflow-x: hidden;
  padding-top: 52px;
}

/* Scrollbar tweaks */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}
::-webkit-scrollbar-track {
  background: transparent;
}
::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.2);
  border-radius: 4px;
  border: 2px solid transparent;
  background-clip: content-box;
}
::-webkit-scrollbar-thumb:hover {
  background-color: rgba(0, 0, 0, 0.4);
}
</style>
