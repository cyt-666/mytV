<template>
  <div 
    ref="cardRef"
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

      <!-- 观看次数 -->
      <div v-if="watchCount > 1" class="count-badge">
        {{ type === 'movie' ? `刷${watchCount}遍` : `${watchCount}集` }}
      </div>
    </div>
    
    <div class="media-info">
      <h3 class="media-title">
        {{ title }}
        <span v-if="translationLoading" class="translation-indicator">
          <div class="translation-dot"></div>
        </span>
      </h3>
      <p class="media-year" v-if="displayYear">{{ displayYear }}</p>
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
import { computed, ref, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import {
  IconImage, IconStarFill, IconCheckCircle, 
  IconPlayCircle, IconClockCircle
} from '@arco-design/web-vue/es/icon'
import type { Movie, Show } from '../types/api'
import { 
  loadMovieTranslationAsync, 
  loadShowTranslationAsync, 
  loadSeasonTranslationAsync,
  loadEpisodeTranslationAsync,
  type TranslationResult 
} from '../utils/translation'

interface Props {
  media: Movie | Show
  type: 'movie' | 'show' | 'season'
  watchedStatus?: 'watched' | 'watching' | 'planned'
  showMeta?: boolean
  watchCount?: number
}

const props = withDefaults(defineProps<Props>(), {
  showMeta: false,
  watchCount: 0
})

const router = useRouter()
const cardRef = ref<HTMLElement | null>(null)
let observer: IntersectionObserver | null = null

// 翻译状态
const translation = ref<TranslationResult | null>(null)
const episodeTranslation = ref<TranslationResult | null>(null)
const translationLoading = ref(false)

// 计算属性
const title = computed(() => {
  // 优先显示翻译标题，没有翻译时显示原标题
  return translation.value?.title || props.media.title
})

const rating = computed(() => props.media.rating)
const genres = computed(() => props.media.genres)
const runtime = computed(() => props.media.runtime)

const displayYear = computed(() => {
  if (props.type === 'show') {
    const show = props.media as Show
    if ((show as any).episode) {
      const ep = (show as any).episode
      const epStr = `S${ep.season.toString().padStart(2, '0')}E${ep.number.toString().padStart(2, '0')}`
      const epTitle = episodeTranslation.value?.title || ep.title
      return epTitle ? `${epStr} · ${epTitle}` : epStr
    }
        if (show.latestSeason && show.latestSeason > 1) {
      return `第${show.latestSeason}季`
    }
  } else if (props.type === 'season') {
    const seasonNumber = (props.media as any).season_number
    if (seasonNumber !== undefined) {
      return `第 ${seasonNumber} 季`
    }
  }
  return props.media.year
})

const posterUrl = computed(() => {
  const images = props.media.images
  if (images?.poster?.length) {
    return `https://${images.poster[0]}`
  }
  return null
})

// 方法
const handleClick = () => {
  let routeName = ''
  let params: any = {}
  const id = props.media.ids?.trakt || props.media.ids?.slug

  if (!id) {
    console.error('无法跳转：缺少有效的ID', props.media)
    return
  }

  if (props.type === 'movie') {
    routeName = 'movie-detail'
    params = { id }
  } else if (props.type === 'show') {
    routeName = 'show-detail'
    params = { id }
  } else if (props.type === 'season') {
    routeName = 'season-detail'
    const seasonNumber = (props.media as any).season_number
    if (seasonNumber === undefined) {
      console.error('无法跳转：缺少season_number', props.media)
      return
    }
    params = { id, season: seasonNumber }
  }
  
  console.log('MediaCard clicked:', { 
    type: props.type, 
    title: props.media.title, 
    ids: props.media.ids,
    id,
    params
  })
  
  // 将图片信息存储到sessionStorage供详情页使用
  if (props.media.images) {
    const cacheKey = `media_images_${id}`
    sessionStorage.setItem(cacheKey, JSON.stringify(props.media.images))
  }
  
  router.push({
    name: routeName,
    params
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

const loadTranslation = async () => {
  try {
    // 检查是否已经在加载或已加载
    if (translation.value || translationLoading.value) return
    
    translationLoading.value = true
    
    // 添加小延迟，避免IntersectionObserver同时也触发大量并发
    await new Promise(resolve => setTimeout(resolve, Math.random() * 500))
    
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
      
      // 如果有单集信息，额外加载单集翻译
      const ep = (props.media as any).episode
      if (ep && props.media.ids?.trakt) {
        loadEpisodeTranslationAsync(
          props.media.ids.trakt, 
          ep.season, 
          ep.number, 
          (trans: TranslationResult | null) => {
            episodeTranslation.value = trans
          }
        )
      }
    } else if (props.type === 'season') {
      const showId = props.media.ids?.trakt
      const seasonNumber = (props.media as any).season_number
      if (showId && seasonNumber !== undefined) {
        loadSeasonTranslationAsync(showId, seasonNumber, (translationData: TranslationResult | null) => {
          translation.value = translationData
          translationLoading.value = false
        })
      } else {
        translationLoading.value = false
      }
    } else {
      // 对于其他类型，直接关闭加载状态
      translationLoading.value = false
    }
    
    // 安全网：如果8秒后还在加载，就停止加载状态
    setTimeout(() => {
      if (translationLoading.value) {
        // console.warn('翻译加载超时，停止加载状态:', props.media.title)
        translationLoading.value = false
      }
    }, 8000)
    
  } catch (error) {
    console.warn('翻译加载失败:', error)
    translationLoading.value = false
  }
}

// 生命周期
onMounted(() => {
  if (cardRef.value) {
    observer = new IntersectionObserver((entries) => {
      const [entry] = entries
      if (entry.isIntersecting) {
        loadTranslation()
        if (observer) {
          observer.disconnect()
          observer = null
        }
      }
    }, {
      rootMargin: '100px' // 提前100px加载
    })
    
    observer.observe(cardRef.value)
  } else {
    // 降级处理：如果没有ref，直接加载
    loadTranslation()
  }
})

onUnmounted(() => {
  if (observer) {
    observer.disconnect()
    observer = null
  }
})
</script>

<style scoped>
.media-card {
  position: relative;
  background: transparent; /* 卡片本身透明，由图片撑起 */
  border-radius: var(--macos-radius-lg, 12px);
  cursor: pointer;
  transition: all 0.4s cubic-bezier(0.25, 0.46, 0.45, 0.94);
  /* 解决 Safari 圆角溢出 */
  transform: translateZ(0); 
}

.media-card:hover {
  transform: scale(1.05) translateY(-4px); /* 放大并上浮 */
  z-index: 10;
}

/* MacOS Hover Refinement */
:global(.platform-macos) .media-card:hover {
  transform: scale(1.03) translateY(-4px); /* More subtle lift on macOS */
}

.poster-container {
  position: relative;
  width: 100%;
  aspect-ratio: 2/3;
  overflow: hidden;
  border-radius: var(--macos-radius-lg, 12px);
  box-shadow: 0 4px 12px rgba(0,0,0,0.1); /* 默认柔和阴影 */
  transition: box-shadow 0.4s ease, transform 0.4s ease;
  background: var(--color-fill-2);
}

/* MacOS Shadow & Border Integration */
:global(.platform-macos) .poster-container {
  background: var(--glass-bg, rgba(255, 255, 255, 0.65));
  box-shadow: 0 4px 10px rgba(0,0,0,0.08), 0 0 0 1px var(--macos-glass-border, rgba(255,255,255,0.1));
  backdrop-filter: blur(10px);
}

.media-card:hover .poster-container {
  box-shadow: 0 16px 32px rgba(0,0,0,0.25); /* 悬停深邃阴影 */
}

:global(.platform-macos) .media-card:hover .poster-container {
  box-shadow: 0 16px 32px rgba(0,0,0,0.15), 0 0 0 1px var(--macos-glass-border, rgba(255,255,255,0.4));
}

.media-poster {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: transform 0.4s ease;
}

/* 悬停时图片不放大，而是整体放大，保持清晰 */
/* .media-card:hover .media-poster { transform: scale(1.05); } */

.poster-placeholder {
  width: 100%;
  height: 100%;
  background: var(--color-fill-2);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--color-text-3);
}

/* 标签样式优化 */
.rating-badge {
  position: absolute;
  top: 8px;
  right: 8px;
  background: rgba(0, 0, 0, 0.7);
  backdrop-filter: blur(4px);
  color: #ffc107;
  padding: 4px 8px;
  border-radius: 6px;
  font-size: 11px;
  font-weight: 700;
  display: flex;
  align-items: center;
  gap: 3px;
  opacity: 0; /* 默认隐藏 */
  transform: translateY(-4px);
  transition: all 0.3s ease;
}

.media-card:hover .rating-badge {
  opacity: 1; /* 悬停显示 */
  transform: translateY(0);
}

.status-badge {
  position: absolute;
  top: 8px;
  left: 8px;
  background: rgba(22, 93, 255, 0.9);
  color: white;
  padding: 4px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  box-shadow: 0 2px 8px rgba(22, 93, 255, 0.4);
}

.count-badge {
  position: absolute;
  bottom: 8px;
  right: 8px;
  background: rgba(0,0,0,0.7);
  color: white;
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 10px;
  font-weight: 600;
}

.media-info {
  padding: 12px 4px;
}

.media-title {
  font-size: 15px;
  font-weight: 600;
  margin: 0 0 4px 0;
  line-height: 1.4;
  color: var(--glass-text, #1d1d1f);
  display: -webkit-box;
  -webkit-line-clamp: 1; /* 单行显示，整洁 */
  -webkit-box-orient: vertical;
  overflow: hidden;
  text-overflow: ellipsis;
  position: relative;
  transition: color 0.2s;
}

.media-card:hover .media-title {
  color: rgb(var(--primary-6)); /* 悬停变色 */
}

.translation-dot {
  display: inline-block;
  width: 6px;
  height: 6px;
  background: rgb(var(--primary-6));
  border-radius: 50%;
  margin-left: 4px;
  vertical-align: middle;
}

.media-year {
  font-size: 13px;
  color: var(--glass-text-secondary, #86909c);
  margin: 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.media-meta {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-top: 6px;
}

.genre-tag {
  background: var(--color-fill-2);
  color: var(--color-text-2);
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 10px;
  font-weight: 500;
}

.runtime {
  font-size: 11px;
  color: var(--glass-text-secondary, #86909c);
}

/* 响应式 */
@media (max-width: 768px) {
  .media-title { font-size: 13px; }
  .media-info { padding: 8px 0; }
  .rating-badge { opacity: 1; } /* 移动端常驻 */
}
</style>
