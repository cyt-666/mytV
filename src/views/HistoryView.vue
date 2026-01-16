<template>
  <div class="history-view">
    <div class="page-container">
      <div class="page-header">
        <h1 class="page-title">观看历史</h1>
        <p class="page-subtitle">查看你的观看记录</p>
      </div>

      <!-- 统计卡片 -->
      <div class="stats-cards">
        <div class="stat-card">
          <div class="stat-number">{{ stats.totalMovies }}</div>
          <div class="stat-label">观看电影</div>
        </div>
        <div class="stat-card">
          <div class="stat-number">{{ stats.totalShows }}</div>
          <div class="stat-label">观看剧集</div>
        </div>
        <div class="stat-card">
          <div class="stat-number">{{ stats.totalHours }}</div>
          <div class="stat-label">观看小时</div>
        </div>
        <div class="stat-card">
          <div class="stat-number">{{ stats.thisMonth }}</div>
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
        :show-meta="true"
        empty-message="还没有观看记录"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { Message } from '@arco-design/web-vue'
import MediaGrid from '../components/MediaGrid.vue'
import type { Movie, Show } from '../types/api'
import { useAuth } from '../composables/useAuth'

interface HistoryItem {
  id: number
  watched_at: string
  type: string
  movie?: Movie
  show?: Show
}

const { userInfo } = useAuth()

const loading = ref(false)
const historyItems = ref<(Movie | Show)[]>([])
const filterType = ref('')
const dateRange = ref([])

const stats = ref({
  totalMovies: 0,
  totalShows: 0,
  totalHours: 0,
  thisMonth: 0
})

const filteredItems = computed(() => {
  let items = [...historyItems.value]
  
  if (filterType.value) {
    items = items.filter(item => {
      const isMovie = 'tagline' in item
      return filterType.value === 'movie' ? isMovie : !isMovie
    })
  }
  
  return items
})

const loadHistory = async () => {
  if (!userInfo.value?.username) {
    Message.warning('请先登录')
    return
  }
  
  loading.value = true
  try {
    const results = await invoke<HistoryItem[]>('get_history', {
      id: userInfo.value.username
    })
    
    const items: (Movie | Show)[] = []
    let movieCount = 0
    let showCount = 0
    let totalMinutes = 0
    
    for (const item of results) {
      if (item.movie) {
        items.push(item.movie)
        movieCount++
        if (item.movie.runtime) {
          totalMinutes += item.movie.runtime
        }
      } else if (item.show) {
        items.push(item.show)
        showCount++
        if (item.show.runtime) {
          totalMinutes += item.show.runtime
        }
      }
    }
    
    historyItems.value = items
    
    const now = new Date()
    const thisMonth = new Date(now.getFullYear(), now.getMonth(), 1)
    const thisMonthCount = results.filter(item => {
      const watchedDate = new Date(item.watched_at)
      return watchedDate >= thisMonth
    }).length
    
    stats.value = {
      totalMovies: movieCount,
      totalShows: showCount,
      totalHours: Math.round(totalMinutes / 60),
      thisMonth: thisMonthCount
    }
  } catch (error) {
    console.error('加载观看历史失败:', error)
    Message.error('加载观看历史失败，请稍后重试')
  } finally {
    loading.value = false
  }
}

const handleDateChange = () => {
  console.log('日期范围变化:', dateRange.value)
}

onMounted(() => {
  loadHistory()
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