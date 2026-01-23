<template>
  <div class="watchlist-view">
    <div class="page-container">
      <div class="page-header">
        <h1 class="page-title">我的观看清单</h1>
        <p class="page-subtitle">管理你想要观看的电影和电视剧</p>
      </div>

      <!-- 筛选和排序 -->
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
            <a-option value="season">季度</a-option>
          </a-select>
          
          <a-select
            v-model="sortBy"
            placeholder="排序方式"
            :style="{ width: '140px' }"
            @change="handleSortChange"
          >
            <a-option value="added_desc">最新添加</a-option>
            <a-option value="added_asc">最早添加</a-option>
            <a-option value="title_asc">标题A-Z</a-option>
            <a-option value="title_desc">标题Z-A</a-option>
            <a-option value="year_desc">年份新-旧</a-option>
            <a-option value="year_asc">年份旧-新</a-option>
          </a-select>
        </a-space>
        
        <a-space :size="12">
          <span class="item-count">{{ filteredItems.length }} 个项目</span>
          <a-button type="outline" @click="clearAll" v-if="watchlistItems.length > 0">
            <icon-delete />
            清空清单
          </a-button>
        </a-space>
      </div>

      <!-- 内容展示 -->
      <MediaGrid
        :items="filteredItems"
        :loading="loading"
        :show-meta="true"
        empty-message="你的观看清单是空的，去发现一些好电影吧！"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { Message } from '@arco-design/web-vue'
import { IconDelete } from '@arco-design/web-vue/es/icon'
import MediaGrid from '../components/MediaGrid.vue'
import type { Movie, Show, Season } from '../types/api'
import { usePageState } from '../composables/usePageState'
import { useAuth } from '../composables/useAuth'

interface WatchlistItem {
  listed_at: string
  movie?: Movie
  show?: Show
  season?: Season
}

interface ExtendedMedia extends Movie, Show {
  listed_at?: string
  media_type: 'movie' | 'show' | 'season'
  season_number?: number
}

const { userInfo } = useAuth()

// 使用状态管理
const { saveState, restoreState } = usePageState('watchlist')

// 定义组件名称用于keep-alive
defineOptions({
  name: 'WatchlistView'
})

// 响应式数据
const loading = ref(false)
const watchlistItems = ref<ExtendedMedia[]>([])
const filterType = ref('')
const sortBy = ref('added_desc')

// 监听用户登录状态变化
watch(() => userInfo.value, (newVal) => {
  if (newVal?.username) {
    loadWatchlist()
  } else {
    watchlistItems.value = []
  }
})

// 计算属性
const filteredItems = computed(() => {
  let items = [...watchlistItems.value]
  
  // 类型筛选
  if (filterType.value) {
    items = items.filter(item => item.media_type === filterType.value)
  }
  
  // 排序
  items.sort((a, b) => {
    switch (sortBy.value) {
      case 'added_desc':
        if (a.listed_at && b.listed_at) {
          return new Date(b.listed_at).getTime() - new Date(a.listed_at).getTime()
        }
        return 0
      case 'added_asc':
        if (a.listed_at && b.listed_at) {
          return new Date(a.listed_at).getTime() - new Date(b.listed_at).getTime()
        }
        return 0
      case 'title_asc':
        return (a.title || '').localeCompare(b.title || '')
      case 'title_desc':
        return (b.title || '').localeCompare(a.title || '')
      case 'year_desc':
        return (b.year || 0) - (a.year || 0)
      case 'year_asc':
        return (a.year || 0) - (b.year || 0)
      default:
        return 0
    }
  })
  
  return items
})

// 状态管理
const saveWatchlistState = () => {
  const state = {
    filterType: filterType.value,
    sortBy: sortBy.value,
    timestamp: Date.now()
  }
  saveState(state)
}

const restoreWatchlistState = () => {
  const state = restoreState()
  if (state && state.timestamp) {
    const tenMinutes = 10 * 60 * 1000
    if (Date.now() - state.timestamp < tenMinutes) {
      filterType.value = state.filterType || ''
      sortBy.value = state.sortBy || 'added_desc'
      return true
    }
  }
  return false
}

// 方法
const loadWatchlist = async () => {
  if (!userInfo.value?.username) {
    Message.warning('请先登录')
    return
  }
  
  loading.value = true
  try {
    const [movieResults, showResults, seasonResults] = await Promise.all([
      invoke<WatchlistItem[]>('get_watchlist', {
        id: userInfo.value.username,
        selectType: 'movies'
      }),
      invoke<WatchlistItem[]>('get_watchlist', {
        id: userInfo.value.username,
        selectType: 'shows'
      }),
      invoke<WatchlistItem[]>('get_watchlist', {
        id: userInfo.value.username,
        selectType: 'seasons'
      })
    ])
    
    const items: ExtendedMedia[] = []
    
    for (const item of movieResults) {
      if (item.movie) {
        const extendedMovie = { ...item.movie, listed_at: item.listed_at, media_type: 'movie' } as ExtendedMedia
        items.push(extendedMovie)
      }
    }
    
    for (const item of showResults) {
      if (item.show) {
        const extendedShow = { ...item.show, listed_at: item.listed_at, media_type: 'show' } as ExtendedMedia
        items.push(extendedShow)
      }
    }

    for (const item of seasonResults) {
      if (item.season && item.show) {
        const extendedSeason = { 
          ...item.show, 
          listed_at: item.listed_at, 
          media_type: 'season',
          season_number: item.season.number,
          // 保持 title 为 show title, MediaCard 会处理显示 "第 X 季"
        } as ExtendedMedia
        items.push(extendedSeason)
      }
    }
    
    watchlistItems.value = items
  } catch (error) {
    console.error('加载观看清单失败:', error)
    Message.error('加载观看清单失败，请稍后重试')
  } finally {
    loading.value = false
  }
}

const handleFilterChange = () => {
  saveWatchlistState()
}

const handleSortChange = () => {
  saveWatchlistState()
}

const clearAll = () => {
  watchlistItems.value = []
  Message.info('清单已清空')
}

// 生命周期
onMounted(() => {
  restoreWatchlistState()
  loadWatchlist()
})

// 页面卸载前保存状态
onBeforeUnmount(() => {
  saveWatchlistState()
})
</script>

<style scoped>
.watchlist-view {
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

/* 响应式设计 */
@media (max-width: 768px) {
  .toolbar {
    flex-direction: column;
    gap: 16px;
    align-items: stretch;
  }
  
  .toolbar > .a-space:first-child {
    justify-content: center;
  }
  
  .toolbar > .a-space:last-child {
    justify-content: space-between;
  }
  
  .page-title {
    font-size: 24px;
  }
}
</style>
