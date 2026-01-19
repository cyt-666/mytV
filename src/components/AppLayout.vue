<template>
  <a-layout class="app-layout">
    <a-layout-sider
      hide-trigger
      collapsible
      :collapsed="collapsed"
      :width="240"
      :collapsed-width="68"
      class="app-sider"
      :class="{ 'is-macos': isMacOS }"
      theme="light"
    >
      <div class="sider-logo" :class="{ 'collapsed': collapsed }">
        <div class="logo-wrapper" @click="$router.push('/')">
          <icon-video-camera :size="28" class="logo-icon" />
          <h1 class="app-title" v-show="!collapsed">MyTV</h1>
        </div>
      </div>

      <a-menu
        :selected-keys="selectedKeys"
        :collapsed="collapsed"
        class="app-menu"
        :class="{ 'collapsed': collapsed }"
        @menu-item-click="handleMenuClick"
      >
        <a-menu-item key="home">
          <template #icon><icon-home /></template>
          首页
        </a-menu-item>
        
        <a-sub-menu key="discover">
          <template #icon><icon-star /></template>
          <template #title>发现</template>
          <a-menu-item key="movies">电影</a-menu-item>
          <a-menu-item key="shows">电视剧</a-menu-item>
          <a-menu-item key="trending">热门</a-menu-item>
        </a-sub-menu>
        
        <a-sub-menu key="tracking">
          <template #icon><icon-play-circle /></template>
          <template #title>追剧</template>
          <a-menu-item key="up-next">待看</a-menu-item>
          <a-menu-item key="calendar">剧集日历</a-menu-item>
        </a-sub-menu>
        
        <a-sub-menu key="my-library">
          <template #icon><icon-bookmark /></template>
          <template #title>我的</template>
          <a-menu-item key="profile">个人中心</a-menu-item>
          <a-menu-item key="watchlist">观看清单</a-menu-item>
          <a-menu-item key="collection">我的片库</a-menu-item>
          <a-menu-item key="history">观看历史</a-menu-item>
        </a-sub-menu>
        
        <a-menu-item key="search">
          <template #icon><icon-search /></template>
          搜索
        </a-menu-item>
      </a-menu>

      <!-- 悬浮胶囊折叠按钮 -->
      <div class="sider-trigger" @click="collapsed = !collapsed">
        <icon-menu-unfold v-if="collapsed" />
        <icon-menu-fold v-else />
      </div>
    </a-layout-sider>

    <a-layout class="layout-main">
      <a-layout-header class="app-header" @mousedown="handleHeaderDrag">
        <div class="header-left">
          <a-button 
            v-if="showGlobalBackButton"
            type="text" 
            class="global-back-button"
            @click="handleGlobalBack"
          >
            <icon-arrow-left :size="26" />
          </a-button>
        </div>
        
        <div class="header-center" data-tauri-drag-region>
          <a-input-search
            v-model="searchQuery"
            class="search-input"
            placeholder="搜索..."
            @search="handleSearch"
            @press-enter="handleSearch"
          >
            <template #prefix>
              <icon-search />
            </template>
          </a-input-search>
        </div>
        
        <div class="header-right">
          <a-space :size="16">
            <a-button type="text" @click="login" v-if="!isLoggedIn" class="user-btn">
              <icon-user />
              登录
            </a-button>
            <a-dropdown v-else>
              <a-button type="text" class="user-btn">
                <a-avatar :size="24" v-if="avatarUrl" style="margin-right: 8px;">
                  <img 
                    :src="avatarUrl" 
                    style="width: 100%; height: 100%; object-fit: cover;" 
                    referrerpolicy="no-referrer"
                  />
                </a-avatar>
                <icon-user v-else style="margin-right: 8px;" />
                {{ userInfo?.username || '用户' }}
                <icon-down />
              </a-button>
              <template #content>
                <a-doption @click="$router.push('/profile')">
                  <icon-user />
                  个人中心
                </a-doption>
                <a-doption @click="logout">
                  <icon-export />
                  退出登录
                </a-doption>
              </template>
            </a-dropdown>
          </a-space>

          <!-- 窗口控制按钮 -->
          <div class="window-controls" v-if="!isMacOS">
            <div class="control-btn" @click="minimizeWindow">
              <icon-minus />
            </div>
            <div class="control-btn" @click="maximizeWindow">
              <icon-fullscreen-exit v-if="isMaximized" />
              <icon-fullscreen v-else />
            </div>
            <div class="control-btn close-btn" @click="closeWindow">
              <icon-close />
            </div>
          </div>
        </div>
      </a-layout-header>

      <a-layout-content class="app-content" ref="contentRef">
        <router-view v-slot="{ Component, route }">
          <keep-alive v-if="shouldKeepAlive(route)">
            <component :is="Component" :key="route.path" />
          </keep-alive>
          <component v-else :is="Component" :key="route.fullPath" />
        </router-view>
      </a-layout-content>
    </a-layout>
  </a-layout>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, provide, onBeforeUnmount, nextTick } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { Message } from '@arco-design/web-vue' // Add this import
import { 
  IconVideoCamera, IconSearch, IconUser, IconDown, IconExport,
  IconHome, IconStar, IconBookmark, IconArrowLeft,
  IconMenuFold, IconMenuUnfold,
  IconMinus, IconFullscreen, IconFullscreenExit, IconClose,
  IconPlayCircle
} from '@arco-design/web-vue/es/icon'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { getCurrentWindow } from '@tauri-apps/api/window'
import type { User } from '../types/api'

const router = useRouter()
const route = useRoute()
const appWindow = getCurrentWindow()
const contentRef = ref()

// 状态管理
const searchQuery = ref('')
const collapsed = ref(false)
const isLoggedIn = ref(false)
const userInfo = ref<User | null>(null)
const showGlobalBackButton = ref(false)
const isMaximized = ref(false) // 追踪最大化状态
const isMacOS = ref(navigator.userAgent.includes('Mac OS X')) // 检测 macOS

const avatarUrl = ref<string | null>(null)

watch(() => userInfo.value?.images?.avatar?.full, async (url) => {
  if (!url) {
    avatarUrl.value = null
    return
  }
  
  if (isMacOS.value) {
    try {
      const proxied = await invoke<string>('get_proxied_image', { url })
      avatarUrl.value = proxied
      return
    } catch (e) {
      console.warn('Avatar proxy failed, falling back to direct URL', e)
    }
  }
  
  avatarUrl.value = url.startsWith('http') ? url.replace(/^http:/, 'https:') : `https://${url}`
}, { immediate: true })

const selectedKeys = computed(() => {
  const path = route.path
  const type = route.query.type as string
  if (path === '/') {
    if (type === 'movies') return ['movies']
    if (type === 'shows') return ['shows'] 
    if (type === 'trending') return ['trending']
    return ['home']
  }
  if (path === '/search') return ['search']
  if (path === '/up-next') return ['up-next']
  if (path === '/calendar') return ['calendar']
  if (path === '/watchlist') return ['watchlist']
  if (path === '/collection') return ['collection']
  if (path === '/history') return ['history']
  if (path === '/profile' || path === '/settings') return ['profile']
  if (path.startsWith('/movie') || path.startsWith('/show')) {
    return ['home']
  }
  return ['home']
})

const checkLoginStatus = async () => {
  try {
    const token = await invoke<boolean>('check_login_status')
    if (token) {
      isLoggedIn.value = true
      await loadUserProfile()
    }
  } catch (error) { console.error(error) }
}

const loadUserProfile = async () => {
  try {
    const profile = await invoke('get_user_profile')
    console.log('User Profile:', profile)
    if (profile) userInfo.value = (profile as any).user
  } catch (error) { console.error(error) }
}

provide('userInfo', userInfo)
provide('isLoggedIn', isLoggedIn)
provide('refreshUserInfo', loadUserProfile)

const setupOAuthListener = () => {
  listen<string>("oauth-callback", async (event) => {
    if (event.payload) {
      try {
        await invoke("get_token", { code: event.payload })
        
        Message.success('登录成功，正在同步数据...')
        
        // 强制重新加载用户信息（带重试机制）
        await loadUserProfile()
        
        if (!userInfo.value) {
          console.log('用户信息加载失败，1秒后重试...')
          await new Promise(r => setTimeout(r, 1000))
          await loadUserProfile()
        }
        
        // 显式更新状态
        if (userInfo.value) {
          isLoggedIn.value = true
          Message.success(`欢迎回来, ${userInfo.value.username}`)
        } else {
          // 如果获取 profile 失败，尝试仅设置登录态
          isLoggedIn.value = true
        }
      } catch (e) {
        console.error(e)
        Message.error('登录失败，请重试')
      }
    }
  })
}

const handleSearch = () => {
  if (searchQuery.value.trim()) {
    router.push({ name: 'search', query: { q: searchQuery.value.trim() } })
  }
}

const handleMenuClick = (key: string) => {
  switch (key) {
    case 'home': router.push('/'); break;
    case 'search': router.push('/search'); break;
    case 'up-next': router.push('/up-next'); break;
    case 'calendar': router.push('/calendar'); break;
    case 'watchlist': router.push('/watchlist'); break;
    case 'collection': router.push('/collection'); break;
    case 'history': router.push('/history'); break;
    case 'profile': router.push('/profile'); break;
    case 'movies': router.push('/?type=movies'); break;
    case 'shows': router.push('/?type=shows'); break;
    case 'trending': router.push('/?type=trending'); break;
  }
}

const login = async () => {
  try { await invoke('start_trakt_user_auth') }
  catch (e) { if (e === 200) { isLoggedIn.value = true; await loadUserProfile() } }
}

const logout = async () => {
  try { await invoke('revoke_token'); isLoggedIn.value = false; userInfo.value = null }
  catch (e) { isLoggedIn.value = false; userInfo.value = null }
}

const handleGlobalBack = () => {
  if (window.history.length > 1) router.back()
  else router.push('/')
}

// 窗口控制
const minimizeWindow = () => appWindow.minimize()
const maximizeWindow = async () => {
  await appWindow.toggleMaximize()
  isMaximized.value = await appWindow.isMaximized()
}
const closeWindow = () => appWindow.close()

// macOS 窗口拖拽：使用 startDragging API 解决 Overlay 模式下无法拖拽的问题
const handleHeaderDrag = async (e: MouseEvent) => {
  // 检查点击目标是否是交互元素（按钮、输入框等）
  const target = e.target as HTMLElement
  const isInteractive = target.closest('button, input, a, .arco-input-wrapper, .user-btn, .control-btn, .window-controls')
  
  // 只有在非交互元素上且是左键点击时才触发拖拽
  if (!isInteractive && e.buttons === 1 && isMacOS.value) {
    await appWindow.startDragging()
  }
}

const updateMaximizeState = async () => {
  isMaximized.value = await appWindow.isMaximized()
}

watch(route, async (newRoute) => {
  showGlobalBackButton.value = newRoute.path !== '/' && newRoute.path !== '/search'
  
  // 路由切换时滚动到顶部
  await nextTick()
  if (contentRef.value?.$el) {
    contentRef.value.$el.scrollTo({ top: 0 })
  }
}, { immediate: true })

const shouldKeepAlive = (route: any) => {
  if (route.path.startsWith('/movie/') || route.path.startsWith('/show/')) {
    return false
  }
  return ['/', '/search', '/watchlist', '/collection', '/history', '/profile'].includes(route.path)
}

const handleKeyDown = (event: KeyboardEvent) => {
  if ((event.altKey || event.ctrlKey) && event.key === 'ArrowLeft') {
    event.preventDefault()
    if (showGlobalBackButton.value) {
      handleGlobalBack()
    }
  }
}

onMounted(async () => {
  checkLoginStatus()
  setupOAuthListener()
  document.addEventListener('keydown', handleKeyDown)
  
  updateMaximizeState()
  await listen('tauri://resize', updateMaximizeState)
})

onBeforeUnmount(() => {
  document.removeEventListener('keydown', handleKeyDown)
})
</script>

<style scoped>
/* 全局容器 */
.app-layout {
  height: 100vh;
  width: 100vw;
  overflow: hidden;
  background: #ffffff; /* 整体白色背景，去除分割线感 */
  font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', sans-serif;
  -webkit-font-smoothing: antialiased;
}

/* 侧边栏 - 现代流媒体风格 */
.app-sider {
  background-color: #ffffff;
  position: relative;
  z-index: 10;
  border-right: none !important; /* 强制去掉分割线 */
  box-shadow: none !important;
}

/* macOS 适配：增加顶部内边距，避开红绿灯 */
.app-sider.is-macos {
  padding-top: 32px; 
}
.app-sider.is-macos .sider-logo {
  /* Logo区域需要设为可拖拽，方便移动窗口 */
  -webkit-app-region: drag;
}

/* 强制覆盖 Arco 默认边框 */
:deep(.arco-layout-sider) {
  border-right: none !important;
}
:deep(.arco-layout-sider-light) {
  border-right: none !important;
  box-shadow: none !important;
}

/* Logo 区域 */
.sider-logo {
  height: 80px;
  display: flex;
  align-items: center;
  padding-left: 16px;
  margin-bottom: 8px;
}

.logo-wrapper {
  display: flex;
  align-items: center;
  gap: 12px;
  cursor: pointer;
  -webkit-app-region: drag;
  padding: 8px 12px;
  border-radius: 12px;
  transition: background-color 0.2s;
}

.logo-wrapper:hover {
  background-color: rgba(0,0,0,0.03);
}

.logo-icon { 
  color: #165dff; 
  filter: drop-shadow(0 4px 10px rgba(22, 93, 255, 0.3)); /* 发光 Logo */
  flex-shrink: 0;
}

.app-title {
  margin: 0;
  font-size: 20px;
  font-weight: 800;
  background: linear-gradient(135deg, #1d1d1f 0%, #333 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  letter-spacing: -0.5px;
}

/* Logo 折叠适配 */
.sider-logo.collapsed {
  justify-content: center;
  padding-left: 0;
}
.sider-logo.collapsed .logo-wrapper {
  padding: 8px;
  justify-content: center;
}

/* 菜单样式 */
.app-menu {
  padding: 0 16px; /* 增加左右留白 */
  transition: padding 0.2s;
}
.app-menu.collapsed {
  padding: 0 8px; /* 折叠时减小留白，确保图标居中 */
}

:deep(.arco-menu-inner) {
  padding: 0;
  overflow: hidden;
}

:deep(.arco-menu-item), 
:deep(.arco-menu-group-title), 
:deep(.arco-menu-pop-header), 
:deep(.arco-menu-inline-header) {
  background-color: transparent;
  line-height: 48px; /* 更高的点击区域 */
  height: 48px;
  margin-bottom: 8px;
  border-radius: 16px; /* 大圆角 */
  color: #4e5969;
  font-size: 15px;
  font-weight: 500;
  display: flex;
  align-items: center;
}

/* 悬停效果 */
:deep(.arco-menu-item:hover), 
:deep(.arco-menu-pop-header:hover), 
:deep(.arco-menu-inline-header:hover) {
  background-color: #f7f8fa;
  color: #1d1d1f;
}

/* 选中效果 - 柔和色块 */
:deep(.arco-menu-selected) { 
  background-color: #f0f4ff !important; /* 极淡蓝 */
  color: #165dff !important;
  font-weight: 600;
}

:deep(.arco-menu-icon) { 
  margin-right: 14px; 
  font-size: 20px;
  transition: transform 0.2s;
}

:deep(.arco-menu-selected .arco-menu-icon) {
  color: #165dff; 
  transform: scale(1.1);
}

/* 折叠后的菜单修正 */
:deep(.arco-menu-collapsed .arco-menu-item),
:deep(.arco-menu-collapsed .arco-menu-inline-header),
:deep(.arco-menu-collapsed .arco-menu-pop-header) {
  justify-content: center !important;
  padding: 0 !important;
  display: flex !important;
  align-items: center !important;
}

:deep(.arco-menu-collapsed .arco-menu-icon) {
  margin-right: 0 !important;
  margin-left: 0 !important;
  font-size: 20px !important;
  display: flex;
  justify-content: center;
  align-items: center;
  width: 100%;
}

/* 确保子菜单标题图标也居中 */
:deep(.arco-menu-collapsed .arco-menu-inline-header .arco-icon),
:deep(.arco-menu-collapsed .arco-menu-pop-header .arco-icon) {
  margin: 0 !important;
}

/* 彻底隐藏折叠后的文字 */
:deep(.arco-menu-collapsed .arco-menu-title),
:deep(.arco-menu-collapsed .arco-menu-item .arco-menu-title),
:deep(.arco-menu-collapsed .arco-menu-inline-header .arco-menu-title),
:deep(.arco-menu-collapsed .arco-menu-pop-header .arco-menu-title) {
  display: none !important;
  width: 0 !important;
  opacity: 0 !important;
}

:deep(.arco-menu-collapsed .arco-menu-inline-header .arco-menu-icon-suffix),
:deep(.arco-menu-collapsed .arco-menu-pop-header .arco-menu-icon-suffix) {
  display: none !important;
}

/* 修复折叠后的子菜单缩进导致的不对齐 */
:deep(.arco-menu-collapsed .arco-menu-item-indent) {
  display: none !important;
  width: 0 !important;
}

/* 选中状态修正 */
:deep(.arco-menu-collapsed .arco-menu-selected) {
  justify-content: center !important;
}

/* 确保 Logo 区域折叠时也居中 */
.sider-logo.collapsed .logo-wrapper {
  justify-content: center !important;
  padding: 0 !important;
}


/* 底部触发器 - 悬浮胶囊 */
.sider-trigger {
  position: absolute;
  bottom: 24px;
  left: 16px;
  right: 16px;
  height: 44px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  background-color: transparent;
  color: #86909c;
  transition: all 0.2s;
}
.sider-trigger:hover {
  background-color: #f7f8fa;
  color: #1d1d1f;
}

/* 右侧主布局容器 */
.layout-main {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
  background-color: #ffffff; /* 与左侧一致，无缝融合 */
}

/* Header - 极简 */
.app-header {
  height: 72px;
  background: rgba(255,255,255,0.8);
  backdrop-filter: blur(20px);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 16px 0 24px; /* 右侧减小到 16px，左侧保持 24px */
  -webkit-app-region: drag;
  flex-shrink: 0;
  z-index: 20;
}

.header-right { 
  display: flex; 
  align-items: center; 
  -webkit-app-region: no-drag; 
}

.window-controls {
  display: flex;
  align-items: center;
  margin-left: 20px;
  gap: 8px; /* 增加按钮间距 */
  -webkit-app-region: no-drag; /* 双重保险：确保该区域不可拖拽 */
}

.control-btn {
  width: 40px; /* 增大宽度 */
  height: 40px; /* 增大高度 */
  border-radius: 8px; /* 更大的圆角 */
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  color: #4e5969;
  transition: all 0.2s;
  font-size: 18px; /* 增大图标 */
}

.control-btn:hover {
  background-color: rgba(0,0,0,0.06);
  color: #1d1d1f;
}

.close-btn:hover {
  background-color: #ff4d4f;
  color: white;
  box-shadow: 0 2px 8px rgba(255, 77, 79, 0.3); /* 关闭按钮增加阴影 */
}

.header-left { 
  display: flex;
  align-items: center;
  min-width: 40px;
  -webkit-app-region: no-drag; 
}
.search-input { -webkit-app-region: no-drag; }

/* 搜索框 - 胶囊 */
.search-input { width: 320px; transition: width 0.3s; }
.search-input:focus-within { width: 360px; }

:deep(.arco-input-wrapper) { 
  border-radius: 20px; 
  background: #f2f3f5; 
  border: 1px solid transparent; 
  height: 40px;
  padding: 0 16px;
}
:deep(.arco-input-wrapper:focus-within) { 
  background: #ffffff; 
  border-color: #165dff;
  box-shadow: 0 4px 12px rgba(22, 93, 255, 0.1);
}

/* 按钮样式 */
.user-btn { 
  color: #1d1d1f; 
  font-size: 14px; 
  font-weight: 600;
  height: 40px;
  border-radius: 20px;
  padding: 0 20px;
}
.user-btn:hover { background-color: #f7f8fa; }

.global-back-button {
  width: 40px;
  height: 40px;
  padding: 0;
  border-radius: 50%;
  color: #1d1d1f;
  background: transparent;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
}
.global-back-button:hover {
  background-color: #f2f3f5; /* 悬停时显示浅灰背景 */
  transform: scale(1.1); /* 稍微放大更多一点 */
}
.global-back-button:active {
  transform: scale(0.95);
  background-color: #e5e6eb;
}
.global-back-button:active {
  transform: scale(0.95);
}

/* 内容区域 */
.app-content {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  position: relative;
  z-index: 5;
  padding-top: 0; /* Header 融合模式下，不需要额外 padding，或者按需调整 */
}

/* 滚动条 */
::-webkit-scrollbar { width: 6px; height: 6px; }
::-webkit-scrollbar-track { background: transparent; }
::-webkit-scrollbar-thumb { background: rgba(0,0,0,0.1); border-radius: 3px; }
::-webkit-scrollbar-thumb:hover { background: rgba(0,0,0,0.2); }

@media (max-width: 768px) {
  .app-header { padding: 0 20px; height: 64px; }
  .search-input { width: 200px; }
  .sider-trigger { bottom: 16px; }
}
</style>