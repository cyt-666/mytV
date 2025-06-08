<template>
  <div class="app-container">
    <!-- 顶部导航栏 -->
    <a-layout-header class="app-header">
      <div class="header-content">
        <div class="header-left">
          <!-- 全局返回按钮 -->
          <a-tooltip 
            v-if="showGlobalBackButton"
            content="返回上一页 (Ctrl+← / Alt+←)"
            position="bottom"
          >
            <a-button 
              type="text" 
              class="global-back-button"
              @click="handleGlobalBack"
            >
              <icon-arrow-left :size="18" />
            </a-button>
          </a-tooltip>
          
          <h1 class="app-title" @click="$router.push('/')">
            <icon-video-camera :size="24" />
            MyTV
          </h1>
        </div>
        
        <div class="header-center">
          <a-input-search
            v-model="searchQuery"
            class="search-input"
            placeholder="搜索电影、电视剧..."
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
            <a-button type="text" @click="login" v-if="!isLoggedIn">
              <icon-user />
              登录
            </a-button>
            <a-dropdown v-else>
              <a-button type="text">
                <icon-user />
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
        </div>
      </div>
    </a-layout-header>

    <a-layout>
      <!-- 侧边导航 -->
      <a-layout-sider
        class="app-sider"
        :collapsed="collapsed"
        :collapsible="true"
        @collapse="handleCollapse"
        :width="240"
        :collapsed-width="60"
      >
        <a-menu
          :selected-keys="selectedKeys"
          :style="{ height: '100%' }"
          @menu-item-click="handleMenuClick"
        >
          <a-menu-item key="home">
            <icon-home />
            <span>首页</span>
          </a-menu-item>
          
          <a-sub-menu key="discover">
            <template #title>
              <icon-star />
              <span>发现</span>
            </template>
            <a-menu-item key="movies">电影</a-menu-item>
            <a-menu-item key="shows">电视剧</a-menu-item>
            <a-menu-item key="trending">热门</a-menu-item>
          </a-sub-menu>
          
          <a-sub-menu key="my-library">
            <template #title>
              <icon-bookmark />
              <span>我的</span>
            </template>
            <a-menu-item key="watchlist">观看清单</a-menu-item>
            <a-menu-item key="collection">收藏</a-menu-item>
            <a-menu-item key="history">观看历史</a-menu-item>
          </a-sub-menu>
          
          <a-menu-item key="search">
            <icon-search />
            <span>搜索</span>
          </a-menu-item>
        </a-menu>
      </a-layout-sider>

      <!-- 主要内容区域 -->
      <a-layout-content class="main-content">
        <router-view v-slot="{ Component, route }">
          <keep-alive v-if="shouldKeepAlive(route)" :include="keepAlivePages">
            <component :is="Component" :key="route.fullPath" />
          </keep-alive>
          <component v-else :is="Component" :key="route.fullPath" />
        </router-view>
      </a-layout-content>
    </a-layout>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, provide, onBeforeUnmount } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { 
  IconVideoCamera, IconSearch, IconUser, IconDown, IconExport,
  IconHome, IconStar, IconBookmark, IconArrowLeft
} from '@arco-design/web-vue/es/icon'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import type { User, Token } from '../types/api'

const router = useRouter()
const route = useRoute()

// 状态管理
const searchQuery = ref('')
const collapsed = ref(false)
const isLoggedIn = ref(false)
const userInfo = ref<User | null>(null)
const showGlobalBackButton = ref(false)

// 需要缓存的页面组件名称
const keepAlivePages = ref(['HomeView', 'SearchView', 'WatchlistView'])

// 计算当前选中的菜单项
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
  if (path === '/watchlist') return ['watchlist']
  if (path === '/collection') return ['collection']
  if (path === '/history') return ['history']
  if (path.startsWith('/movie') || path.startsWith('/show')) {
    return ['home'] // 详情页时高亮首页
  }
  return ['home']
})

// 检查登录状态
const checkLoginStatus = async () => {
  try {
    const token = await invoke<boolean>('check_login_status')
    if (token) {
      isLoggedIn.value = true
      // 获取用户信息
      await loadUserProfile()
    }
  } catch (error) {
    console.error('检查登录状态失败:', error)
  }
}

// 获取用户信息
const loadUserProfile = async () => {
  try {
    const profile = await invoke('get_user_profile')
    if (profile) {
      userInfo.value = (profile as any).user // 从UserProfile中提取user
    }
  } catch (error) {
    console.error('获取用户信息失败:', error)
  }
}

// 提供给子组件使用
provide('userInfo', userInfo)
provide('isLoggedIn', isLoggedIn)
provide('refreshUserInfo', loadUserProfile)

// 处理OAuth回调
const setupOAuthListener = () => {
  listen<string>("oauth-callback", async (event) => {
    const code = event.payload;
    if (code.length > 0) {
      try {
        const token = await invoke("get_token", { code });
        console.log('获取到token:', token);
        isLoggedIn.value = true;
        await loadUserProfile();
      } catch (error) {
        console.error('获取token失败:', error);
      }
    }
  });
}

// 处理搜索
const handleSearch = () => {
  if (searchQuery.value.trim()) {
    router.push({
      name: 'search',
      query: { q: searchQuery.value.trim() }
    })
  }
}

// 处理菜单点击
const handleMenuClick = (key: string) => {
  switch (key) {
    case 'home':
      router.push('/')
      break
    case 'search':
      router.push('/search')
      break
    case 'watchlist':
      router.push('/watchlist')
      break
    case 'collection':
      router.push('/collection')
      break
    case 'history':
      router.push('/history')
      break
    case 'movies':
      router.push('/?type=movies')
      break
    case 'shows':
      router.push('/?type=shows')
      break
    case 'trending':
      router.push('/?type=trending')
      break
  }
}

// 处理侧边栏折叠
const handleCollapse = (val: boolean) => {
  collapsed.value = val
}

// 登录功能
const login = async () => {
  try {
    const result = await invoke('start_trakt_user_auth')
    console.log('开始用户认证:', result)
  } catch (error) {
    if (error === 200) {
      // 已经登录
      isLoggedIn.value = true
      await loadUserProfile()
    } else {
      console.error('登录失败:', error)
    }
  }
}

// 退出登录
const logout = async () => {
  try {
    await invoke('revoke_token')
    isLoggedIn.value = false
    userInfo.value = null
  } catch (error) {
    console.error('退出登录失败:', error)
    // 即使API调用失败，也重置前端状态
    isLoggedIn.value = false
    userInfo.value = null
  }
}

// 监听路由变化，控制全局返回按钮显示
watch(route, (newRoute) => {
  // 在详情页面显示全局返回按钮
  const isDetailPage = newRoute.path.startsWith('/movie/') || 
                      newRoute.path.startsWith('/show/') ||
                      newRoute.path === '/profile' ||
                      newRoute.path === '/search'
  showGlobalBackButton.value = isDetailPage
}, { immediate: true })

// 处理全局返回按钮
const handleGlobalBack = () => {
  // 如果有历史记录，则返回上一页，否则跳转到首页
  if (window.history.length > 1) {
    router.back()
  } else {
    router.push('/')
  }
}

// 添加键盘快捷键支持
const handleKeyDown = (event: KeyboardEvent) => {
  // Alt + 左箭头 或 Ctrl + 左箭头 返回上一页
  if ((event.altKey || event.ctrlKey) && event.key === 'ArrowLeft') {
    event.preventDefault()
    if (showGlobalBackButton.value) {
      handleGlobalBack()
    }
  }
}

onMounted(() => {
  checkLoginStatus()
  setupOAuthListener()
  document.addEventListener('keydown', handleKeyDown)
})

// 在组件卸载时清理事件监听器
onBeforeUnmount(() => {
  document.removeEventListener('keydown', handleKeyDown)
})

// 监听路由变化，清空搜索框
watch(route, () => {
  if (route.name !== 'search') {
    searchQuery.value = ''
  }
})

// 根据路由决定是否缓存组件
const shouldKeepAlive = (route: any) => {
  // 详情页面不缓存（因为它们是动态的）
  if (route.path.startsWith('/movie/') || route.path.startsWith('/show/')) {
    return false
  }
  
  // 其他页面可以缓存
  return ['/', '/search', '/watchlist', '/collection', '/history', '/profile'].includes(route.path)
}
</script>

<style scoped>
.app-header {
  background: white;
  border-bottom: 1px solid #f0f0f0;
  padding: 0;
  height: 64px;
  line-height: 64px;
  position: sticky;
  top: 0;
  z-index: 100;
}

.header-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  max-width: 1400px;
  margin: 0 auto;
  padding: 0 24px;
  height: 100%;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.global-back-button {
  padding: 8px;
  border-radius: 8px;
  transition: all 0.3s ease;
  color: #4a5568;
  position: relative;
  overflow: hidden;
}

.global-back-button::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(22, 93, 255, 0.1);
  border-radius: 8px;
  transform: translateX(-100%);
  transition: transform 0.3s ease;
}

.global-back-button:hover::before {
  transform: translateX(0);
}

.global-back-button:hover {
  color: #165dff;
  transform: translateX(-2px);
}

.global-back-button:active {
  transform: translateX(-1px) scale(0.95);
}

.app-title {
  display: flex;
  align-items: center;
  gap: 8px;
  margin: 0;
  font-size: 20px;
  font-weight: 700;
  color: #1d1d1f;
  cursor: pointer;
  user-select: none;
}

.header-center {
  flex: 1;
  max-width: 500px;
  margin: 0 40px;
}

.header-right {
  flex: 0 0 auto;
}

.app-sider {
  background: white;
  border-right: 1px solid #f0f0f0;
}

.main-content {
  background: #f5f5f7;
  min-height: calc(100vh - 64px);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .header-center {
    margin: 0 16px;
  }
  
  .header-content {
    padding: 0 16px;
  }
  
  .app-sider {
    position: fixed;
    left: 0;
    top: 64px;
    bottom: 0;
    z-index: 99;
  }
  
  .main-content {
    margin-left: 0;
  }
}
</style> 