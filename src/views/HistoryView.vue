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

      <!-- 历史记录 -->
      <MediaGrid
        :items="filteredItems"
        :loading="loading"
        :loading-more="loadingMore"
        :has-more="hasMore"
        :infinite-scroll="true"
        :show-meta="true"
        empty-message="还没有观看记录"
        @load-more="handleLoadMore"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { Message } from '@arco-design/web-vue'
import MediaGrid from '../components/MediaGrid.vue'
import type { Movie, Show, UserStats } from '../types/api'
import { useAuth } from '../composables/useAuth'
import { usePageState } from '../composables/usePageState'

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

type HistoryMedia = (Movie | Show) & { watch_count?: number }

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

// 简单的本地统计（仅用于显示本月数据等无法从API直接获取的特定统计）
const localStats = ref({
  thisMonth: 0
})

const filteredItems = computed(() => {
  let items = [...historyItems.value]
  
  if (filterType.value) {
    items = items.filter(item => {
      // 优先使用显式的 media_type
      if (item.media_type) {
        return item.media_type === filterType.value
      }
      const isMovie = 'tagline' in item
      return filterType.value === 'movie' ? isMovie : !isMovie
    })
  }
  
  return items
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
    const maxAttempts = 5 // 防止无限循环
    
    // 初始化去重集合
    const seenShows = new Set<number>()
    const seenMovies = new Set<number>()
    
    // 如果是加载更多，需要保留已有的去重记录
    if (isLoadMore) {
      historyItems.value.forEach(item => {
        if ('ids' in item && item.ids?.trakt) {
          if (item.media_type === 'show') {
             seenShows.add(item.ids.trakt)
          } else if (item.media_type === 'movie') {
             seenMovies.add(item.ids.trakt)
          }
        }
      })
    }

    // 循环加载直到获取到新数据或没有更多数据
    while (addedCount === 0 && hasMore.value && attempt < maxAttempts) {
      attempt++
      
      const results = await invoke<HistoryItem[]>('get_history', {
        id: userInfo.value.username,
        page: page.value,
        limit: limit
      })
      
      // 如果返回的数据少于limit，说明没有更多数据了
      if (results.length < limit) {
        hasMore.value = false
      }
      
      const items: HistoryMedia[] = []
      
      for (const item of results) {
        if (item.type === 'movie' && item.movie) {
          const movieId = item.movie.ids?.trakt
          if (movieId) {
            if (seenMovies.has(movieId)) {
              // 已存在，增加计数
              const existing = items.find(i => i.ids?.trakt === movieId && i.media_type === 'movie')
              if (existing) {
                existing.watch_count = (existing.watch_count || 1) + 1
              } else {
                // 检查历史列表
                const existingOld = historyItems.value.find(i => i.ids?.trakt === movieId && i.media_type === 'movie')
                if (existingOld) {
                  existingOld.watch_count = (existingOld.watch_count || 1) + 1
                }
              }
            } else {
              seenMovies.add(movieId)
              item.movie.media_type = 'movie'
              ;(item.movie as HistoryMedia).watch_count = 1
              items.push(item.movie as HistoryMedia)
            }
          }
        } else if (item.type === 'episode' && item.show) {
          const showId = item.show.ids?.trakt
          if (showId) {
            if (seenShows.has(showId)) {
              const existing = items.find(i => i.ids?.trakt === showId && i.media_type === 'show')
              if (existing) {
                existing.watch_count = (existing.watch_count || 1) + 1
              } else {
                const existingOld = historyItems.value.find(i => i.ids?.trakt === showId && i.media_type === 'show')
                if (existingOld) {
                  existingOld.watch_count = (existingOld.watch_count || 1) + 1
                }
              }
            } else {
              seenShows.add(showId)
              item.show.media_type = 'show'
              ;(item.show as HistoryMedia).watch_count = 1
              items.push(item.show as HistoryMedia)
            }
          }
        }
      }
      
      // 计算本月观看数
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
      
      // 如果获取到了新数据，或者已经没有更多数据了，跳出循环
      if (addedCount > 0 || !hasMore.value) break
    }
    
    saveHistoryState()
  } catch (error) {
    console.error('加载观看历史失败:', error)
    Message.error('加载观看历史失败，请稍后重试')
  } finally {
    loading.value = false
    loadingMore.value = false
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
    // 如果恢复了列表但没有统计数据（理论上一起恢复），重新加载统计
    loadStats()
  }
})

onBeforeUnmount(() => {
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
  font-size: 28px;
  font-weight: 700;
  margin: 0 0 8px 0;
  color: #1d1d1f;
}

.page-subtitle {
  font-size: 16px;
  color: #8e8e93;
  margin: 0;
}

.stats-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 20px;
  margin-bottom: 32px;
}

.stat-card {
  background: white;
  padding: 24px;
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
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
  background: white;
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
}

.item-count {
  font-size: 14px;
  color: #8e8e93;
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