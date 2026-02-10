<template>
  <div class="history-view">
    <div class="page-container">
      <div class="page-header">
        <h1 class="page-title">观看历史</h1>
        <p class="page-subtitle">查看你的观看记录</p>
      </div>

      <!-- 统计卡片 -->
      <div class="stats-cards" v-if="stats">
        <div class="stat-card">
          <div class="stat-number">{{ stats.movies.watched }}</div>
          <div class="stat-label">观看电影</div>
        </div>
        <div class="stat-card">
          <div class="stat-number">{{ stats.shows.watched }}</div>
          <div class="stat-label">观看剧集</div>
        </div>
        <div class="stat-card">
          <div class="stat-number">{{ Math.round((stats.movies.minutes + stats.episodes.minutes) / 60) }}</div>
          <div class="stat-label">观看小时</div>
        </div>
        <div class="stat-card">
          <div class="stat-number">{{ localStats.thisMonth }}</div>
          <div class="stat-label">本月观看</div>
        </div>
      </div>

      <!-- 筛选器 -->
      <div class="toolbar">
        <a-space :size="16">
          <a-select
            v-model="filterType"
            placeholder="类型筛选"
            :style="{ width: '120px' }"
            @change="handleFilterChange"
          >
            <a-option value="">全部</a-option>
            <a-option value="movie">电影</a-option>
            <a-option value="show">电视剧</a-option>
          </a-select>
          
          <a-range-picker
            v-model="dateRange"
            :style="{ width: '240px' }"
            placeholder="选择日期范围"
            @change="handleDateChange"
          />
        </a-space>
        
        <span class="item-count">{{ historyItems.length }} 条记录</span>
      </div>

      <!-- 时间轴历史记录 -->
      <div class="timeline-container" v-if="!loading && timelineGroups.length > 0">
        <div 
          v-for="group in timelineGroups" 
          :key="group.label" 
          class="timeline-group"
        >
          <div class="timeline-header">
            <div class="timeline-dot"></div>
            <h3 class="timeline-label">{{ group.label }}</h3>
            <span class="timeline-count">{{ group.items.length }} 条</span>
          </div>
          <div class="timeline-content">
            <MediaGrid
              :items="group.items"
              :loading="false"
              :loading-more="false"
              :has-more="false"
              :infinite-scroll="false"
              :show-meta="true"
              empty-message=""
            />
          </div>
        </div>
      </div>

      <!-- 加载状态 -->
      <MediaGrid
        v-if="loading"
        :items="[]"
        :loading="true"
        :loading-more="false"
        :has-more="false"
        :infinite-scroll="false"
        empty-message="还没有观看记录"
      />

      <!-- 空状态 -->
      <div v-if="!loading && timelineGroups.length === 0 && historyItems.length === 0" class="empty-state">
        <p>还没有观看记录</p>
      </div>

      <!-- 加载更多 -->
      <div v-if="hasMore && !loading" class="load-more-container">
        <div ref="loadTrigger" class="load-trigger"></div>
        <a-spin v-if="loadingMore" :size="24" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount, nextTick, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { Message } from '@arco-design/web-vue'
import MediaGrid from '../components/MediaGrid.vue'
import type { Movie, Show, UserStats } from '../types/api'
import { useAuth } from '../composables/useAuth'
import { usePageState } from '../composables/usePageState'
import { useUserDataUpdate } from '../composables/useEvent'

interface HistoryItem {
  id: number
  watched_at: string
  type: string
  movie?: Movie
  show?: Show
  episode?: {
    season: number
    number: number
    title?: string
    runtime?: number
    ids: {
      trakt: number
    }
  }
}

type HistoryMedia = (Movie | Show) & { watch_count?: number, watched_at?: string }

const { userInfo } = useAuth()
const { saveState, restoreState } = usePageState('history')

defineOptions({
  name: 'HistoryView'
})

const loading = ref(false)
const loadingMore = ref(false)
const hasMore = ref(true)
const page = ref(1)
const limit = 20 // 每次加载20条
const historyItems = ref<HistoryMedia[]>([])
const filterType = ref('')
const dateRange = ref([])
const stats = ref<UserStats | null>(null)

const loadTrigger = ref<HTMLElement | null>(null)
let observer: IntersectionObserver | null = null

const setupInfiniteScroll = () => {
  if (observer) observer.disconnect()
  
  observer = new IntersectionObserver((entries) => {
    const [entry] = entries
    if (entry.isIntersecting) {
      handleLoadMore()
    }
  }, { rootMargin: '400px', threshold: 0 })
  
  if (loadTrigger.value) {
    observer.observe(loadTrigger.value)
  }
}

watch(loadTrigger, (el) => {
  if (el && observer) {
    observer.disconnect()
    observer.observe(el)
  }
})

// 简单的本地统计
const localStats = ref({
  thisMonth: 0
})

// 监听 Stats 更新
useUserDataUpdate((payload) => {
  if (!userInfo.value?.ids?.slug) return
  
  if (payload.key === `stats_${userInfo.value.ids.slug}`) {
    stats.value = payload.data as UserStats
  }
})

// 监听 History 更新 (仅第一页)
useUserDataUpdate((payload) => {
  if (!userInfo.value?.ids?.slug) return
  
  const cacheKey = `history_${userInfo.value.ids.slug}_p1_l${limit}`
  if (payload.key === cacheKey) {
    console.log('收到 History 第一页更新')
    // 重置并重新处理
    // 注意：这可能会导致用户正在查看的列表突然变化
    // 更好的做法是提示用户刷新，或者只在用户还在第一页时静默刷新
    if (page.value === 1) {
      // 重新处理第一页数据
      
      // 这里简化处理，直接替换
      // 实际应该合并去重
      // 为了安全起见，这里仅打印日志，不强制刷新，以免打断用户
      // 或者我们可以弹出一个 Toast "有新的观看记录，点击刷新"
      Message.info({
        content: '发现新的观看记录，请刷新页面查看',
        duration: 3000
      })
    }
  }
})

const filteredItems = computed(() => {
  let items = [...historyItems.value]
  
  if (filterType.value) {
    items = items.filter(item => {
      if (item.media_type) {
        return item.media_type === filterType.value
      }
      const isMovie = 'tagline' in item
      return filterType.value === 'movie' ? isMovie : !isMovie
    })
  }
  
  items.sort((a, b) => {
    if (a.watched_at && b.watched_at) {
      return new Date(b.watched_at).getTime() - new Date(a.watched_at).getTime()
    }
    return 0
  })
  
  return items
})

interface TimelineGroup {
  label: string
  items: HistoryMedia[]
}

const timelineGroups = computed<TimelineGroup[]>(() => {
  const items = filteredItems.value
  if (items.length === 0) return []
  
  const now = new Date()
  const today = new Date(now.getFullYear(), now.getMonth(), now.getDate())
  const yesterday = new Date(today.getTime() - 24 * 60 * 60 * 1000)
  
  const groupsMap = new Map<string, HistoryMedia[]>()
  
  for (const item of items) {
    let label: string
    
    if (!item.watched_at) {
      label = '未知日期'
    } else {
      const watchedDate = new Date(item.watched_at)
      const watchedDay = new Date(watchedDate.getFullYear(), watchedDate.getMonth(), watchedDate.getDate())
      
      if (watchedDay.getTime() >= today.getTime()) {
        label = '今天'
      } else if (watchedDay.getTime() >= yesterday.getTime()) {
        label = '昨天'
      } else {
        label = `${watchedDate.getMonth() + 1}月${watchedDate.getDate()}日`
      }
    }
    
    if (!groupsMap.has(label)) {
      groupsMap.set(label, [])
    }
    groupsMap.get(label)!.push(item)
  }
  
  return Array.from(groupsMap.entries()).map(([label, items]) => ({ label, items }))
})

const saveHistoryState = () => {
  const state = {
    historyItems: historyItems.value,
    page: page.value,
    hasMore: hasMore.value,
    filterType: filterType.value,
    stats: stats.value,
    localStats: localStats.value,
    scrollPosition: window.scrollY,
    timestamp: Date.now()
  }
  saveState(state)
}

const handleFilterChange = () => {
  saveHistoryState()
}

const restoreHistoryState = () => {
  const state = restoreState()
  if (state && state.timestamp) {
    const tenMinutes = 10 * 60 * 1000
    if (Date.now() - state.timestamp < tenMinutes) {
      historyItems.value = state.historyItems || []
      page.value = state.page || 1
      hasMore.value = state.hasMore !== undefined ? state.hasMore : true
      filterType.value = state.filterType || ''
      stats.value = state.stats || null
      localStats.value = state.localStats || { thisMonth: 0 }
      
      if (state.scrollPosition > 0) {
        nextTick(() => {
          window.scrollTo({ top: state.scrollPosition, behavior: 'smooth' })
        })
      }
      return true
    }
  }
  return false
}

const loadStats = async () => {
  if (!userInfo.value?.username) return
  
  try {
    const result = await invoke<UserStats>('get_user_stats', {
      id: userInfo.value.username
    })
    stats.value = result
  } catch (error) {
    console.error('加载用户统计失败:', error)
  }
}

const loadHistory = async (isLoadMore = false) => {
  if (!userInfo.value?.username) {
    Message.warning('请先登录')
    return
  }
  
  if (isLoadMore) {
    loadingMore.value = true
  } else {
    loading.value = true
    page.value = 1
    historyItems.value = []
    hasMore.value = true
  }

  try {
    let addedCount = 0
    let attempt = 0
    const maxAttempts = 5 
    
    while (addedCount === 0 && hasMore.value && attempt < maxAttempts) {
      attempt++
      
      const results = await invoke<HistoryItem[]>('get_history', {
        id: userInfo.value.username,
        page: page.value,
        limit: limit
      })
      
      if (results.length === 0) {
        hasMore.value = false
      }
      
      const items: HistoryMedia[] = []
      
      for (const item of results) {
        if (item.type === 'movie' && item.movie) {
          item.movie.media_type = 'movie'
          ;(item.movie as HistoryMedia).watch_count = 1
          ;(item.movie as HistoryMedia).watched_at = item.watched_at
          items.push(item.movie as HistoryMedia)
        } else if (item.type === 'episode' && item.show) {
          item.show.media_type = 'show'
          ;(item.show as HistoryMedia).watch_count = 1
          ;(item.show as HistoryMedia).watched_at = item.watched_at
          if (item.episode) {
             ;(item.show as any).episode = item.episode
          }
          items.push(item.show as HistoryMedia)
        }
      }

      const now = new Date()
      const thisMonth = new Date(now.getFullYear(), now.getMonth(), 1)
      const newThisMonthCount = results.filter(item => {
        const watchedDate = new Date(item.watched_at)
        return watchedDate >= thisMonth
      }).length

      if (isLoadMore) {
        historyItems.value = [...historyItems.value, ...items]
        localStats.value.thisMonth += newThisMonthCount
      } else {
        if (attempt === 1) {
          historyItems.value = items
          localStats.value.thisMonth = newThisMonthCount
        } else {
          historyItems.value = [...historyItems.value, ...items]
          localStats.value.thisMonth += newThisMonthCount
        }
      }
      
      addedCount = items.length
      page.value++
      
      if (addedCount > 0 || !hasMore.value) break
    }
    
    saveHistoryState()
  } catch (error) {
    console.error('加载观看历史失败:', error)
    Message.error('加载观看历史失败，请稍后重试')
  } finally {
    loading.value = false
    loadingMore.value = false
    
    // 如果还有更多数据，且 loadTrigger 仍在视口内（例如数据没填满屏幕），继续加载
    if (hasMore.value && loadTrigger.value && observer) {
      nextTick(() => {
        // 由于 IntersectionObserver 无法直接检测当前是否可见，我们尝试重新 observe
        // 或者简单判断：如果页面高度不够长，继续加载
        const docHeight = document.documentElement.scrollHeight
        const winHeight = window.innerHeight
        
        // 如果内容高度不足以产生滚动条，或者接近底部，继续加载
        if (docHeight <= winHeight + 100) {
          handleLoadMore()
        }
      })
    }
  }
}

const handleLoadMore = () => {
  if (!loadingMore.value && hasMore.value) {
    loadHistory(true)
  }
}

const handleDateChange = () => {
  console.log('日期范围变化:', dateRange.value)
}

onMounted(() => {
  const restored = restoreHistoryState()
  if (!restored) {
    loadStats()
    loadHistory()
  } else if (!stats.value) {
    loadStats()
  }
  setupInfiniteScroll()
})

onBeforeUnmount(() => {
  if (observer) {
    observer.disconnect()
    observer = null
  }
  saveHistoryState()
})
</script>

<style scoped>
.history-view {
  min-height: 100vh;
}

.page-header {
  margin-bottom: 32px;
}

.page-title {
  font-size: 34px;
  font-weight: 700;
  letter-spacing: -0.02em;
  margin: 0 0 8px 0;
  color: var(--glass-text, rgba(0, 0, 0, 0.88));
}

.page-subtitle {
  font-size: 17px;
  color: var(--glass-text-secondary, rgba(60, 60, 67, 0.6));
  margin: 0;
}

.stats-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 20px;
  margin-bottom: 32px;
}

.stat-card {
  background: var(--glass-bg, rgba(255, 255, 255, 0.6));
  backdrop-filter: var(--glass-blur, blur(20px));
  -webkit-backdrop-filter: var(--glass-blur, blur(20px));
  padding: 24px;
  border-radius: 16px;
  border: 1px solid var(--glass-border, rgba(255, 255, 255, 0.3));
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.04);
  text-align: center;
}

.stat-number {
  font-size: 32px;
  font-weight: 700;
  color: #1d1d1f;
  margin-bottom: 4px;
}

.stat-label {
  font-size: 14px;
  color: #8e8e93;
}

.toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
  padding: 16px 20px;
  background: var(--glass-bg, rgba(255, 255, 255, 0.6));
  backdrop-filter: var(--glass-blur, blur(20px));
  -webkit-backdrop-filter: var(--glass-blur, blur(20px));
  border-radius: 16px;
  border: 1px solid var(--glass-border, rgba(255, 255, 255, 0.3));
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.04);
}
/* ... */
.timeline-count {
  font-size: 14px;
  color: #8e8e93;
  background: rgba(0, 0, 0, 0.05);
  padding: 2px 10px;
  border-radius: 12px;
}

.timeline-content {
  background: var(--glass-bg, rgba(255, 255, 255, 0.6));
  backdrop-filter: var(--glass-blur, blur(20px));
  -webkit-backdrop-filter: var(--glass-blur, blur(20px));
  border-radius: 16px;
  padding: 16px;
  border: 1px solid var(--glass-border, rgba(255, 255, 255, 0.3));
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.04);
}

.stat-number {
  font-size: 32px;
  font-weight: 700;
  color: var(--glass-text, #1d1d1f);
  margin-bottom: 4px;
}

.stat-label {
  font-size: 14px;
  color: var(--glass-text-secondary, #8e8e93);
}

.toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
  padding: 16px 20px;
  background: var(--glass-bg, rgba(255, 255, 255, 0.6));
  backdrop-filter: var(--glass-blur, blur(20px));
  -webkit-backdrop-filter: var(--glass-blur, blur(20px));
  border-radius: 16px;
  border: 1px solid var(--glass-border, rgba(255, 255, 255, 0.3));
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.04);
}

.item-count {
  font-size: 14px;
  color: var(--glass-text-secondary, #8e8e93);
}

/* 时间轴样式 */
.timeline-container {
  position: relative;
  padding-left: 24px;
}

.timeline-container::before {
  content: '';
  position: absolute;
  left: 7px;
  top: 0;
  bottom: 0;
  width: 2px;
  background: linear-gradient(to bottom, #165dff, #86c1ff);
  border-radius: 1px;
}

.timeline-group {
  position: relative;
  margin-bottom: 32px;
}

.timeline-group:last-child {
  margin-bottom: 0;
}

.timeline-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 16px;
  position: relative;
}

.timeline-dot {
  position: absolute;
  left: -24px;
  width: 16px;
  height: 16px;
  background: #165dff;
  border-radius: 50%;
  border: 3px solid #fff;
  box-shadow: 0 2px 8px rgba(22, 93, 255, 0.3);
}

.timeline-label {
  font-size: 18px;
  font-weight: 600;
  color: var(--glass-text, #1d1d1f);
  margin: 0;
}

.timeline-count {
  font-size: 14px;
  color: var(--glass-text-secondary, #8e8e93);
  background: var(--glass-overlay-bg, #f5f5f5);
  padding: 2px 10px;
  border-radius: 12px;
}

.timeline-content {
  background: var(--glass-bg, rgba(255, 255, 255, 0.6));
  backdrop-filter: var(--glass-blur, blur(20px));
  -webkit-backdrop-filter: var(--glass-blur, blur(20px));
  border-radius: 16px;
  padding: 16px;
  border: 1px solid var(--glass-border, rgba(255, 255, 255, 0.3));
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.04);
}

.empty-state {
  text-align: center;
  padding: 60px 20px;
  color: var(--glass-text-secondary, #8e8e93);
}

.load-more-container {
  display: flex;
  justify-content: center;
  padding: 40px 20px;
}

.load-trigger {
  height: 20px;
  width: 100%;
}

@media (max-width: 768px) {
  .stats-cards {
    grid-template-columns: repeat(2, 1fr);
    gap: 16px;
  }
  
  .stat-card {
    padding: 16px;
  }
  
  .stat-number {
    font-size: 24px;
  }
}
</style>
