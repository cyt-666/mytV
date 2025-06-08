<template>
  <div 
    class="media-card"
    @click="handleClick"
  >
    <div class="poster-container">
      <img
        v-if="posterUrl"
        :src="posterUrl"
        :alt="title"
        class="media-poster"
        @error="handleImageError"
      />
      <div v-else class="poster-placeholder">
        <icon-image :size="40" />
      </div>
      
      <!-- 评分badge -->
      <div v-if="rating" class="rating-badge">
        <icon-star-fill />
        {{ rating.toFixed(1) }}
      </div>
      
      <!-- 状态指示器 -->
      <div v-if="watchedStatus" class="status-badge">
        <icon-check-circle v-if="watchedStatus === 'watched'" />
        <icon-play-circle v-else-if="watchedStatus === 'watching'" />
        <icon-clock-circle v-else />
      </div>
    </div>
    
    <div class="media-info">
      <h3 class="media-title">
        {{ title }}
        <span v-if="translationLoading" class="translation-indicator">
          <div class="translation-dot"></div>
        </span>
      </h3>
      <p class="media-year" v-if="year">{{ year }}</p>
      <div class="media-meta" v-if="showMeta">
        <span v-if="genres?.length" class="genre-tag">
          {{ genres[0] }}
        </span>
        <span v-if="runtime" class="runtime">
          {{ formatRuntime(runtime) }}
        </span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import {
  IconImage, IconStarFill, IconCheckCircle, 
  IconPlayCircle, IconClockCircle
} from '@arco-design/web-vue/es/icon'
import type { Movie, Show } from '../types/api'
import { loadMovieTranslationAsync, loadShowTranslationAsync, type TranslationResult } from '../utils/translation'

interface Props {
  media: Movie | Show
  type: 'movie' | 'show'
  watchedStatus?: 'watched' | 'watching' | 'planned'
  showMeta?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  showMeta: false
})

const router = useRouter()

// 翻译状态
const translation = ref<TranslationResult | null>(null)
const translationLoading = ref(false)

// 计算属性
const title = computed(() => {
  // 优先显示翻译标题，没有翻译时显示原标题
  return translation.value?.title || props.media.title
})

const year = computed(() => props.media.year)
const rating = computed(() => props.media.rating)
const genres = computed(() => props.media.genres)
const runtime = computed(() => props.media.runtime)

const posterUrl = computed(() => {
  const images = props.media.images
  if (images?.poster?.length) {
    return `https://${images.poster[0]}`
  }
  return null
})

// 方法
const handleClick = () => {
  const routeName = props.type === 'movie' ? 'movie-detail' : 'show-detail'
  // 优先使用 trakt ID（数字），因为后端API需要u32类型
  const id = props.media.ids?.trakt || props.media.ids?.slug
  
  // 将图片信息存储到sessionStorage供详情页使用
  if (props.media.images) {
    const cacheKey = `media_images_${id}`
    sessionStorage.setItem(cacheKey, JSON.stringify(props.media.images))
  }
  
  router.push({
    name: routeName,
    params: { id }
  })
}

const handleImageError = (event: Event) => {
  const target = event.target as HTMLImageElement
  target.style.display = 'none'
}

const formatRuntime = (minutes: number) => {
  const hours = Math.floor(minutes / 60)
  const mins = minutes % 60
  if (hours > 0) {
    return `${hours}h ${mins}m`
  }
  return `${mins}m`
}

// 生命周期
onMounted(() => {
  // 为电影和电视剧加载翻译
  // 添加随机延迟，避免同时发起太多请求
  const delay = Math.random() * 2000 // 0-2秒随机延迟，分散请求
  
  const loadTranslation = async () => {
    try {
      translationLoading.value = true
      
      if (props.type === 'movie') {
        loadMovieTranslationAsync(props.media as Movie, (translationData: TranslationResult | null) => {
          translation.value = translationData
          translationLoading.value = false
        })
      } else if (props.type === 'show') {
        loadShowTranslationAsync(props.media as Show, (translationData: TranslationResult | null) => {
          translation.value = translationData
          translationLoading.value = false
        })
      } else {
        // 对于其他类型，直接关闭加载状态
        translationLoading.value = false
      }
      
      // 安全网：如果5秒后还在加载，就停止加载状态
      setTimeout(() => {
        if (translationLoading.value) {
          console.warn('翻译加载超时，停止加载状态:', props.media.title)
          translationLoading.value = false
        }
      }, 5000)
      
    } catch (error) {
      console.warn('翻译加载失败:', error)
      translationLoading.value = false
    }
  }
  
  setTimeout(loadTranslation, delay)
})
</script>

<style scoped>
.media-card {
  background: white;
  border-radius: 12px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.08);
  overflow: hidden;
  transition: all 0.3s ease;
  cursor: pointer;
  position: relative;
}

.media-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 25px rgba(0, 0, 0, 0.15);
}

.poster-container {
  position: relative;
  width: 100%;
  aspect-ratio: 2/3;
  overflow: hidden;
}

.media-poster {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: transform 0.3s ease;
}

.media-card:hover .media-poster {
  transform: scale(1.05);
}

.poster-placeholder {
  width: 100%;
  height: 100%;
  background: #f5f5f5;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #bbb;
}

.rating-badge {
  position: absolute;
  top: 8px;
  right: 8px;
  background: rgba(0, 0, 0, 0.8);
  color: white;
  padding: 4px 8px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 600;
  display: flex;
  align-items: center;
  gap: 4px;
}

.status-badge {
  position: absolute;
  top: 8px;
  left: 8px;
  background: rgba(0, 0, 0, 0.8);
  color: white;
  padding: 4px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
}

.media-info {
  padding: 12px;
}

.media-title {
  font-size: 14px;
  font-weight: 600;
  margin: 0 0 4px 0;
  line-height: 1.4;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
  color: #1d1d1f;
  position: relative;
}

.translation-indicator {
  position: absolute;
  top: -2px;
  right: -6px;
}

.translation-dot {
  width: 4px;
  height: 4px;
  background: #165dff;
  border-radius: 50%;
  animation: pulse-dot 1.5s ease-in-out infinite;
}

@keyframes pulse-dot {
  0%, 100% { 
    opacity: 0.4;
    transform: scale(1);
  }
  50% { 
    opacity: 1;
    transform: scale(1.2);
  }
}

.media-year {
  font-size: 12px;
  color: #8e8e93;
  margin: 0 0 8px 0;
}

.media-meta {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
}

.genre-tag {
  background: #f0f0f0;
  color: #666;
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 10px;
  font-weight: 500;
}

.runtime {
  font-size: 11px;
  color: #999;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .media-title {
    font-size: 13px;
  }
  
  .media-info {
    padding: 10px;
  }
  
  .rating-badge,
  .status-badge {
    top: 6px;
  }
  
  .rating-badge {
    right: 6px;
    padding: 3px 6px;
    font-size: 11px;
  }
}
</style> 