<template>
  <div class="collection-view">
    <div class="page-container">
      <div class="page-header">
        <h1 class="page-title">我的片库</h1>
        <p class="page-subtitle">已入库的电影和电视剧</p>
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
          </a-select>
          
          <a-select
            v-model="sortBy"
            placeholder="排序方式"
            :style="{ width: '140px' }"
            @change="handleSortChange"
          >
            <a-option value="collected_desc">最新入库</a-option>
            <a-option value="collected_asc">最早入库</a-option>
            <a-option value="title_asc">标题A-Z</a-option>
            <a-option value="title_desc">标题Z-A</a-option>
            <a-option value="rating_desc">评分高-低</a-option>
            <a-option value="rating_asc">评分低-高</a-option>
          </a-select>
        </a-space>
        
        <span class="item-count">{{ collectionItems.length }} 个项目</span>
      </div>

      <!-- 内容展示 -->
      <MediaGrid
        :items="filteredAndSortedItems"
        :loading="loading"
        :show-meta="true"
        empty-message="还没有入库任何内容，快去添加一些喜欢的影视作品吧！"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { Message } from '@arco-design/web-vue'
import MediaGrid from '../components/MediaGrid.vue'
import type { Movie, Show } from '../types/api'
import { useAuth } from '../composables/useAuth'
import { usePageState } from '../composables/usePageState'

interface CollectionItem {
  collected_at: string
  updated_at: string
  movie?: Movie
  show?: Show
}

interface ExtendedMedia extends Movie, Show {
  collected_at?: string
  media_type?: 'movie' | 'show'
}

const { userInfo } = useAuth()
const { saveState, restoreState } = usePageState('collection')

defineOptions({
  name: 'CollectionView'
})

const loading = ref(false)
const collectionItems = ref<ExtendedMedia[]>([])
const filterType = ref('')
const sortBy = ref('collected_desc')

const filteredAndSortedItems = computed(() => {
  let items = [...collectionItems.value]
  
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
  
  items.sort((a, b) => {
    switch (sortBy.value) {
      case 'collected_desc':
        if (a.collected_at && b.collected_at) {
          return new Date(b.collected_at).getTime() - new Date(a.collected_at).getTime()
        }
        return 0
      case 'collected_asc':
        if (a.collected_at && b.collected_at) {
          return new Date(a.collected_at).getTime() - new Date(b.collected_at).getTime()
        }
        return 0
      case 'title_asc':
        return a.title.localeCompare(b.title)
      case 'title_desc':
        return b.title.localeCompare(a.title)
      case 'rating_desc':
        return (b.rating || 0) - (a.rating || 0)
      case 'rating_asc':
        return (a.rating || 0) - (b.rating || 0)
      default:
        return 0
    }
  })
  
  return items
})

const saveCollectionState = () => {
  const state = {
    collectionItems: collectionItems.value,
    filterType: filterType.value,
    sortBy: sortBy.value,
    scrollPosition: window.scrollY,
    timestamp: Date.now()
  }
  saveState(state)
}

const handleFilterChange = () => {
  saveCollectionState()
}

const handleSortChange = () => {
  saveCollectionState()
}

const restoreCollectionState = () => {
  const state = restoreState()
  if (state && state.timestamp) {
    const tenMinutes = 10 * 60 * 1000
    if (Date.now() - state.timestamp < tenMinutes) {
      collectionItems.value = state.collectionItems || []
      filterType.value = state.filterType || ''
      sortBy.value = state.sortBy || 'collected_desc'
      
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

const loadCollection = async () => {
  if (!userInfo.value?.username) {
    Message.warning('请先登录')
    return
  }
  
  loading.value = true
  try {
    const movieResults = await invoke<CollectionItem[]>('get_collection', {
      id: userInfo.value.username,
      selectType: 'movies'
    })
    
    const showResults = await invoke<CollectionItem[]>('get_collection', {
      id: userInfo.value.username,
      selectType: 'shows'
    })
    
    const items: ExtendedMedia[] = []
    
    for (const item of movieResults) {
      if (item.movie) {
        const extendedMovie = { ...item.movie, collected_at: item.collected_at, media_type: 'movie' } as ExtendedMedia
        items.push(extendedMovie)
      }
    }
    
    for (const item of showResults) {
      if (item.show) {
        const extendedShow = { ...item.show, collected_at: item.collected_at, media_type: 'show' } as ExtendedMedia
        items.push(extendedShow)
      }
    }
    
    collectionItems.value = items
    saveCollectionState()
  } catch (error) {
    console.error('加载收藏失败:', error)
    Message.error('加载收藏失败，请稍后重试')
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  const restored = restoreCollectionState()
  if (!restored) {
    loadCollection()
  }
})

onBeforeUnmount(() => {
  saveCollectionState()
})
</script>

<style scoped>
.collection-view {
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
</style> 