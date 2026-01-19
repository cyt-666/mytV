<template>
  <div class="profile-view">
    <div class="page-container">
      <div class="profile-header">
        <div class="profile-info">
          <div class="avatar">
            <a-avatar :size="80" v-if="userInfo && avatarUrl">
              <img 
                :src="avatarUrl" 
                :alt="userInfo?.username || 'User'" 
                style="width: 100%; height: 100%; object-fit: cover;"
                referrerpolicy="no-referrer"
              />
            </a-avatar>
            <a-avatar :size="80" v-else>
              <icon-user />
            </a-avatar>
          </div>
          <div class="user-details">
            <!-- 临时调试：显示图片 URL -->
            <!-- <div style="font-size: 10px; color: red;">{{ userInfo?.images?.avatar?.full }}</div> -->
            <h1 class="username">{{ userInfo?.username || '用户名' }}</h1>
            <p class="user-bio">
              {{ userInfo?.name || 'Trakt 用户' }} • 
              加入于 {{ formatJoinDate(userInfo?.joined_at) }}
            </p>
          </div>
        </div>
        
        <a-button type="primary" @click="$router.push('/settings')">
          <icon-settings />
          设置
        </a-button>
      </div>

      <!-- 统计概览 -->
      <div class="stats-overview">
        <div class="stat-item">
          <div class="stat-value">{{ loading ? '-' : (userStats?.movies.watched || 0) }}</div>
          <div class="stat-label">观看电影</div>
        </div>
        <div class="stat-item">
          <div class="stat-value">{{ loading ? '-' : (userStats?.shows.watched || 0) }}</div>
          <div class="stat-label">观看剧集</div>
        </div>
        <div class="stat-item">
          <div class="stat-value">{{ loading ? '-' : formatMinutesToHours((userStats?.movies.minutes || 0) + (userStats?.episodes.minutes || 0)) }}</div>
          <div class="stat-label">观看小时</div>
        </div>
        <div class="stat-item">
          <div class="stat-value">{{ loading ? '-' : ((userStats?.movies.collected || 0) + (userStats?.shows.collected || 0)) }}</div>
          <div class="stat-label">收藏作品</div>
        </div>
      </div>

      <!-- 功能菜单 -->
      <div class="feature-menu">
        <a-list :bordered="false">
          <a-list-item @click="$router.push('/watchlist')">
            <a-list-item-meta>
              <template #avatar>
                <icon-bookmark />
              </template>
              <template #title>我的观看清单</template>
              <template #description>管理想要观看的内容</template>
            </a-list-item-meta>
            <template #actions>
              <icon-right />
            </template>
          </a-list-item>

          <a-list-item @click="$router.push('/collection')">
            <a-list-item-meta>
              <template #avatar>
                <icon-heart />
              </template>
              <template #title>我的收藏</template>
              <template #description>已收藏的影视作品</template>
            </a-list-item-meta>
            <template #actions>
              <icon-right />
            </template>
          </a-list-item>

          <a-list-item @click="$router.push('/history')">
            <a-list-item-meta>
              <template #avatar>
                <icon-calendar-clock />
              </template>
              <template #title>观看历史</template>
              <template #description>查看观看记录和统计</template>
            </a-list-item-meta>
            <template #actions>
              <icon-right />
            </template>
          </a-list-item>
        </a-list>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { 
  IconUser, IconSettings, IconBookmark, 
  IconHeart, IconCalendarClock, IconRight 
} from '@arco-design/web-vue/es/icon'
import { useAuth } from '../composables/useAuth'
import { UserStats } from '../types/api'
import { invoke } from "@tauri-apps/api/core"

const { userInfo, isLoggedIn } = useAuth()

const userStats = ref<UserStats | null>(null)
const loading = ref(false)
const isMacOS = navigator.userAgent.includes('Mac OS X')

// 头像 URL 处理
const avatarUrl = ref<string | null>(null)

watch(() => userInfo.value?.images?.avatar?.full, async (url) => {
  if (!url) {
    avatarUrl.value = null
    return
  }
  
  if (isMacOS) {
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

// 监听登录状态变化
watch(isLoggedIn, (val) => {
  if (val && userInfo.value?.ids?.slug) {
    fetchUserStats()
  } else {
    userStats.value = null
  }
})

// 获取用户统计数据
const fetchUserStats = async () => {
  if (!userInfo.value?.ids?.slug) return
  
  loading.value = true
  try {
    const stats = await invoke<UserStats>("get_user_stats", { id: userInfo.value.ids.slug })
    userStats.value = stats
  } catch (error) {
    console.error('获取用户统计数据失败:', error)
  } finally {
    loading.value = false
  }
}

// 组件挂载时获取数据
onMounted(() => {
  if (isLoggedIn.value) {
    fetchUserStats()
  }
})

// 格式化分钟数为小时
const formatMinutesToHours = (minutes?: number) => {
  if (!minutes) return '0'
  return Math.round(minutes / 60).toString()
}

// 格式化加入日期
const formatJoinDate = (dateString?: string) => {
  if (!dateString) return '未知'
  return new Date(dateString).getFullYear() + '年'
}
</script>

<style scoped>
.profile-view {
  min-height: 100vh;
}

.profile-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 32px 0;
  border-bottom: 1px solid #f0f0f0;
  margin-bottom: 32px;
}

.profile-info {
  display: flex;
  align-items: center;
  gap: 20px;
}

.avatar {
  flex-shrink: 0;
}

.username {
  font-size: 24px;
  font-weight: 700;
  margin: 0 0 4px 0;
  color: #1d1d1f;
}

.user-bio {
  font-size: 14px;
  color: #8e8e93;
  margin: 0;
}

.stats-overview {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 24px;
  margin-bottom: 32px;
}

.stat-item {
  background: white;
  padding: 24px;
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
  text-align: center;
}

.stat-value {
  font-size: 28px;
  font-weight: 700;
  color: #1d1d1f;
  margin-bottom: 4px;
}

.stat-label {
  font-size: 14px;
  color: #8e8e93;
}

.feature-menu {
  background: white;
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
}

.feature-menu .arco-list-item {
  cursor: pointer;
  transition: background-color 0.2s ease;
}

.feature-menu .arco-list-item:hover {
  background: #f8f9fa;
}

@media (max-width: 768px) {
  .profile-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 20px;
  }
  
  .stats-overview {
    grid-template-columns: repeat(2, 1fr);
    gap: 16px;
  }
  
  .stat-item {
    padding: 16px;
  }
  
  .stat-value {
    font-size: 24px;
  }
}
</style> 