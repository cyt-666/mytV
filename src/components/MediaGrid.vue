<template>
  <div class="media-grid-container">
    <!-- 加载状态 -->
    <div v-if="loading && items.length === 0" class="loading-grid">
      <div 
        v-for="i in 20" 
        :key="i" 
        class="loading-card"
      >
        <div class="loading-poster loading-skeleton"></div>
        <div class="loading-info">
          <div class="loading-title loading-skeleton"></div>
          <div class="loading-year loading-skeleton"></div>
        </div>
      </div>
    </div>

    <!-- 内容网格 -->
    <div v-else-if="items.length > 0" class="media-grid">
      <MediaCard
        v-for="item in items"
        :key="getItemKey(item)"
        :media="item"
        :type="getItemType(item)"
        :watched-status="getWatchedStatus(item)"
        :show-meta="showMeta"
        :watch-count="(item as any).watch_count"
      />
    </div>

    <!-- 空状态 -->
    <div v-else class="empty-state">
      <div class="empty-icon">
        <icon-empty :size="80" />
      </div>
      <h3 class="empty-title">暂无内容</h3>
      <p class="empty-description">{{ emptyMessage }}</p>
    </div>

    <!-- 加载更多按钮 -->
    <div v-if="items.length > 0 && hasMore" class="load-more-container">
      <a-button 
        v-if="!loadingMore"
        @click="handleLoadMore"
        size="large"
        type="outline"
      >
        <icon-down />
        加载更多
      </a-button>
      <a-spin v-else :size="24" />
    </div>

    <!-- 无限滚动加载 -->
    <div 
      v-if="infiniteScroll && hasMore" 
      ref="loadTrigger"
      class="load-trigger"
    ></div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { IconEmpty, IconDown } from '@arco-design/web-vue/es/icon'
import MediaCard from './MediaCard.vue'
import type { Movie, Show } from '../types/api'

interface Props {
  items: (Movie | Show)[]
  loading?: boolean
  loadingMore?: boolean
  hasMore?: boolean
  showMeta?: boolean
  emptyMessage?: string
  infiniteScroll?: boolean
  mediaType?: 'movie' | 'show' | 'auto'
}

interface Emits {
  (event: 'load-more'): void
}

const props = withDefaults(defineProps<Props>(), {
  loading: false,
  loadingMore: false,
  hasMore: true,
  showMeta: false,
  emptyMessage: '没有找到相关内容',
  infiniteScroll: true,
  mediaType: 'auto'
})

const emit = defineEmits<Emits>()

const loadTrigger = ref<HTMLElement>()
let observer: IntersectionObserver | null = null

// 计算属性
const getItemKey = (item: Movie | Show) => {
  return item.ids?.trakt || item.ids?.slug || item.title
}

const getItemType = (item: Movie | Show): 'movie' | 'show' => {
  // 如果明确指定了类型，直接使用
  if (props.mediaType !== 'auto') {
    return props.mediaType
  }
  
  // 自动推断逻辑
  // 0. 检查显式的 media_type 属性
  if ('media_type' in item && item.media_type) {
    return item.media_type
  }

  // 1. 检查是否有电影特有的属性
  if ('tagline' in item && item.tagline !== undefined) {
    return 'movie'
  }
  // 检查是否有电视剧特有的属性
  if ('network' in item || 'seasons' in item || 'aired_episodes' in item) {
    return 'show'
  }
  // 检查从MovieTrending或ShowTrending包装中获取的数据
  if ('movie' in item) {
    return 'movie'
  }
  if ('show' in item) {
    return 'show'
  }
  // 默认根据released属性判断（电影通常有released，电视剧没有）
  if ('released' in item && item.released) {
    return 'movie'
  }
  // 默认返回show
  return 'show'
}

const getWatchedStatus = (_item: Movie | Show) => {
  // 这里可以根据用户的观看记录返回状态
  // 从localStorage或API获取用户的观看状态
  return undefined
}

// 方法
const handleLoadMore = () => {
  if (!props.loadingMore && props.hasMore) {
    emit('load-more')
  }
}

const setupInfiniteScroll = () => {
  if (!props.infiniteScroll || !loadTrigger.value) return

  observer = new IntersectionObserver(
    (entries) => {
      const [entry] = entries
      if (entry.isIntersecting && props.hasMore && !props.loadingMore) {
        handleLoadMore()
      }
    },
    {
      rootMargin: '100px'
    }
  )

  observer.observe(loadTrigger.value)
}

const cleanupInfiniteScroll = () => {
  if (observer && loadTrigger.value) {
    observer.unobserve(loadTrigger.value)
    observer.disconnect()
    observer = null
  }
}

// 生命周期
onMounted(() => {
  if (props.infiniteScroll) {
    setupInfiniteScroll()
  }
})

onUnmounted(() => {
  cleanupInfiniteScroll()
})
</script>

<style scoped>
.media-grid-container {
  width: 100%;
}

.media-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
  gap: 20px;
  margin: 20px 0;
}

.loading-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
  gap: 20px;
  margin: 20px 0;
}

.loading-card {
  background: white;
  border-radius: 12px;
  overflow: hidden;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.08);
}

.loading-poster {
  width: 100%;
  aspect-ratio: 2/3;
  border-radius: 12px 12px 0 0;
}

.loading-info {
  padding: 12px;
}

.loading-title {
  height: 16px;
  border-radius: 4px;
  margin-bottom: 8px;
}

.loading-year {
  height: 12px;
  width: 60%;
  border-radius: 4px;
}

.loading-skeleton {
  background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%);
  background-size: 200% 100%;
  animation: loading 1.5s infinite;
}

@keyframes loading {
  0% {
    background-position: 200% 0;
  }
  100% {
    background-position: -200% 0;
  }
}

.empty-state {
  text-align: center;
  padding: 60px 20px;
  color: #8e8e93;
}

.empty-icon {
  margin-bottom: 16px;
  opacity: 0.6;
}

.empty-title {
  font-size: 18px;
  font-weight: 600;
  margin: 0 0 8px 0;
  color: #1d1d1f;
}

.empty-description {
  font-size: 14px;
  margin: 0;
  opacity: 0.8;
}

.load-more-container {
  display: flex;
  justify-content: center;
  padding: 40px 20px;
}

.load-trigger {
  height: 1px;
  opacity: 0;
}

/* 响应式设计 */
@media (max-width: 1200px) {
  .media-grid,
  .loading-grid {
    grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
  }
}

@media (max-width: 768px) {
  .media-grid,
  .loading-grid {
    grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
    gap: 16px;
  }
}

@media (max-width: 480px) {
  .media-grid,
  .loading-grid {
    grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
    gap: 12px;
  }
  
  .empty-state {
    padding: 40px 20px;
  }
  
  .empty-title {
    font-size: 16px;
  }
  
  .empty-description {
    font-size: 13px;
  }
}
</style> 