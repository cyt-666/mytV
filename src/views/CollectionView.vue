<template>
  <div class="collection-view">
    <div class="page-container">
      <div class="page-header">
        <h1 class="page-title">我的收藏</h1>
        <p class="page-subtitle">已收藏的电影和电视剧</p>
      </div>

      <!-- 筛选和排序 -->
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
          
          <a-select
            v-model="sortBy"
            placeholder="排序方式"
            :style="{ width: '140px' }"
          >
            <a-option value="collected_desc">最新收藏</a-option>
            <a-option value="collected_asc">最早收藏</a-option>
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
        empty-message="还没有收藏任何内容，快去收藏一些喜欢的影视作品吧！"
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

interface CollectionItem {
  collected_at: string
  updated_at: string
  movie?: Movie
  show?: Show
}

interface ExtendedMedia extends Movie, Show {
  collected_at?: string
}

const { userInfo } = useAuth()

const loading = ref(false)
const collectionItems = ref<ExtendedMedia[]>([])
const filterType = ref('')
const sortBy = ref('collected_desc')

const filteredAndSortedItems = computed(() => {
  let items = [...collectionItems.value]
  
  if (filterType.value) {
    items = items.filter(item => {
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
        const extendedMovie = { ...item.movie, collected_at: item.collected_at } as ExtendedMedia
        items.push(extendedMovie)
      }
    }
    
    for (const item of showResults) {
      if (item.show) {
        const extendedShow = { ...item.show, collected_at: item.collected_at } as ExtendedMedia
        items.push(extendedShow)
      }
    }
    
    collectionItems.value = items
  } catch (error) {
    console.error('加载收藏失败:', error)
    Message.error('加载收藏失败，请稍后重试')
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  loadCollection()
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