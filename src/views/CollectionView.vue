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
        :items="collectionItems"
        :loading="loading"
        :show-meta="true"
        empty-message="还没有收藏任何内容，快去收藏一些喜欢的影视作品吧！"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import MediaGrid from '../components/MediaGrid.vue'
import type { Movie, Show } from '../types/api'

const loading = ref(false)
const collectionItems = ref<(Movie | Show)[]>([])
const filterType = ref('')
const sortBy = ref('collected_desc')

const loadCollection = async () => {
  loading.value = true
  try {
    console.log('加载收藏')
    // const response = await invoke('get_collection')
    // collectionItems.value = response
    collectionItems.value = []
  } catch (error) {
    console.error('加载收藏失败:', error)
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