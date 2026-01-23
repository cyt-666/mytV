<template>
  <div class="movie-detail-view">
    <!-- 加载状态 -->
    <div v-if="loading" class="loading-container">
      <a-spin :size="40" />
      <p>加载中...</p>
    </div>

    <!-- 错误状态 -->
    <div v-else-if="error" class="error-container">
      <div class="error-icon">
        <icon-exclamation-circle :size="48" />
      </div>
      <h3>加载失败</h3>
      <p>{{ error }}</p>
      <a-button @click="fetchMovieDetails" type="primary">
        重试
      </a-button>
    </div>

    <!-- 电影详情内容 -->
    <div v-else-if="movieDetails" class="movie-detail-content">
      <!-- 背景模糊图 -->
      <div class="backdrop" v-if="backdropUrl" :style="{ backgroundImage: `url(${backdropUrl})` }"></div>
      
      <div class="detail-container">
        <!-- 主要信息区域 -->
        <div class="main-info">
          <!-- 海报 -->
          <div class="poster-section">
            <img 
              v-if="posterUrl" 
              :src="posterUrl" 
              :alt="movieDetails.title"
              class="movie-poster"
              @error="handleImageError"
            />
            <div v-else class="poster-placeholder">
              <icon-image :size="60" />
            </div>
            
            <!-- 操作按钮 -->
            <div class="action-buttons">
              <a-button 
                type="primary" 
                size="large" 
                class="action-btn"
                @click="handleMarkAsWatched"
                :loading="actionLoading.watched"
              >
                <icon-play-arrow />
                标记已观看
              </a-button>
              <a-button 
                size="large" 
                :class="['action-btn', { 'is-active': isInCollection }]"
                :type="isInCollection ? 'primary' : 'secondary'"
                :status="isInCollection ? 'success' : undefined"
                @click="handleToggleCollection"
                :loading="actionLoading.collection"
              >
                <icon-heart />
                {{ isInCollection ? '已入库' : '入库' }}
              </a-button>
              <a-button 
                size="large" 
                :class="['action-btn', { 'is-active': isInWatchlist }]"
                :type="isInWatchlist ? 'primary' : 'secondary'"
                :status="isInWatchlist ? 'warning' : undefined"
                @click="handleToggleWatchlist"
                :loading="actionLoading.watchlist"
              >
                <icon-bookmark />
                {{ isInWatchlist ? '已想看' : '想看' }}
              </a-button>
            </div>
          </div>

          <!-- 详细信息 -->
          <div class="info-section">
            <div class="title-area">
              <h1 class="movie-title">
                {{ chineseTranslation?.title || movieDetails.title }}
                <span v-if="translationLoading" class="translation-loading">
                  <a-spin :size="14" />
                </span>
              </h1>
              <p v-if="movieDetails.original_title && movieDetails.original_title !== movieDetails.title" 
                 class="original-title">
                {{ movieDetails.original_title }}
              </p>
              <p v-if="chineseTranslation?.tagline || movieDetails.tagline" class="tagline">
                {{ chineseTranslation?.tagline || movieDetails.tagline }}
              </p>
            </div>

            <!-- 评分和基本信息 -->
            <div class="rating-info">
              <div v-if="movieDetails.rating" class="rating-score">
                <icon-star-fill />
                <span class="score">{{ movieDetails.rating.toFixed(1) }}</span>
                <span class="votes">({{ movieDetails.votes }} 评价)</span>
              </div>
              
              <div class="basic-info">
                <span class="info-item">{{ movieDetails.year }}</span>
                <span v-if="movieDetails.runtime" class="info-item">{{ formatRuntime(movieDetails.runtime) }}</span>
                <span v-if="movieDetails.certification" class="info-item certification">{{ movieDetails.certification }}</span>
              </div>
            </div>

            <!-- 类型标签 -->
            <div v-if="movieDetails.genres?.length" class="genres">
              <a-tag v-for="genre in movieDetails.genres" :key="genre" class="genre-tag">
                {{ genre }}
              </a-tag>
            </div>

            <!-- 剧情简介 -->
            <div v-if="chineseTranslation?.overview || movieDetails.overview" class="overview">
              <h3>剧情简介</h3>
              <p>{{ chineseTranslation?.overview || movieDetails.overview }}</p>
            </div>

            <!-- 详细信息 -->
            <div class="detailed-info">
              <div class="info-grid">
                <div v-if="movieDetails.released" class="info-row">
                  <span class="label">上映日期:</span>
                  <span class="value">{{ formatDate(movieDetails.released) }}</span>
                </div>
                <div v-if="movieDetails.country" class="info-row">
                  <span class="label">制片国家:</span>
                  <span class="value">{{ movieDetails.country.toUpperCase() }}</span>
                </div>
                <div v-if="movieDetails.languages?.length" class="info-row">
                  <span class="label">语言:</span>
                  <span class="value">{{ movieDetails.languages.join(', ') }}</span>
                </div>
                <div v-if="movieDetails.homepage" class="info-row">
                  <span class="label">官方网站:</span>
                  <a :href="movieDetails.homepage" target="_blank" class="homepage-link">
                    访问官网
                    <icon-link />
                  </a>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { 
  IconImage, IconStarFill, IconPlayArrow, IconHeart,
  IconBookmark, IconExclamationCircle, IconLink
} from '@arco-design/web-vue/es/icon'
import { Message } from '@arco-design/web-vue'
import { invoke } from "@tauri-apps/api/core"
import type { MovieDetails, MovieImages } from '../types/api'
import { getMovieChineseTranslation, type TranslationResult } from '../utils/translation'
import { useMediaUpdate } from '../composables/useEvent'

const route = useRoute()

// 响应式数据
const movieDetails = ref<MovieDetails | null>(null)
const loading = ref(false)
const error = ref('')
const movieImages = ref<MovieImages | null>(null)
const chineseTranslation = ref<TranslationResult | null>(null)
const translationLoading = ref(false)

const isInCollection = ref(false)
const isInWatchlist = ref(false)
const actionLoading = ref({
  collection: false,
  watchlist: false,
  watched: false
})

// 监听后台更新事件
useMediaUpdate((payload) => {
  if (payload.type === 'movie' && payload.id === Number(route.params.id)) {
    console.log('接收到后台更新数据:', payload.data)
    movieDetails.value = payload.data
    // 重新加载翻译
    loadTranslationAsync(payload.id)
    Message.info({
      content: '数据已自动更新',
      position: 'bottom',
      duration: 2000
    })
  }
})

// 计算属性
const posterUrl = computed(() => {
  if (movieImages.value?.poster?.length) {
    const posterPath = movieImages.value.poster[0]
    // 如果已经是完整URL，直接使用；否则添加https://
    if (posterPath.startsWith('http')) {
      return posterPath
    }
    return `https://${posterPath}`
  }
  return null
})

const backdropUrl = computed(() => {
  if (movieImages.value?.fanart?.length) {
    return `https://${movieImages.value.fanart[0]}`
  }
  return null
})

// 方法
const fetchMovieDetails = async () => {
  const movieId = route.params.id
  if (!movieId) {
    error.value = '无效的电影ID'
    return
  }

  // 从sessionStorage加载图片信息
  const cacheKey = `media_images_${movieId}`
  const cachedImages = sessionStorage.getItem(cacheKey)
  if (cachedImages) {
    try {
      movieImages.value = JSON.parse(cachedImages)
    } catch (e) {
      console.warn('解析缓存的图片信息失败:', e)
    }
  }

  // 确保ID是数字类型
  const numericId = typeof movieId === 'string' ? parseInt(movieId, 10) : Number(movieId)
  if (isNaN(numericId)) {
    error.value = '无效的电影ID格式'
    return
  }

  loading.value = true
  error.value = ''

  try {
    // 立即获取电影详情，不等待翻译
    // 后端实现了 SWR 策略，如果缓存存在会立即返回，旧数据会触发后台更新事件
    const details = await invoke<MovieDetails>("movie_details", { id: numericId })
    movieDetails.value = details
    
    console.log('电影详情:', details)
    
    // 异步加载翻译，不阻塞页面显示
    loadTranslationAsync(numericId)
  } catch (err) {
    console.error('获取电影详情失败:', err)
    error.value = '获取电影详情失败，请稍后重试'
    Message.error('获取电影详情失败')
  } finally {
    loading.value = false
  }
}

// 异步加载翻译
const loadTranslationAsync = async (movieId: number) => {
  translationLoading.value = true
  try {
    // 在下一个事件循环中加载翻译，确保页面先渲染
    await new Promise(resolve => setTimeout(resolve, 0))
    
    const translation = await getMovieChineseTranslation(movieId)
    chineseTranslation.value = translation
    
    console.log('翻译加载完成:', translation)
  } catch (error) {
    console.warn('翻译加载失败:', error)
    // 翻译失败不影响页面功能
  } finally {
    translationLoading.value = false
  }
}

const formatRuntime = (minutes: number) => {
  const hours = Math.floor(minutes / 60)
  const mins = minutes % 60
  if (hours > 0) {
    return `${hours}小时${mins}分钟`
  }
  return `${mins}分钟`
}

const formatDate = (dateString: string) => {
  const date = new Date(dateString)
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'long',
    day: 'numeric'
  })
}

const handleImageError = (event: Event) => {
  const target = event.target as HTMLImageElement
  target.style.display = 'none'
}

const handleToggleCollection = async () => {
  if (!movieDetails.value?.ids?.trakt) {
    Message.error('电影信息不完整')
    return
  }
  
  actionLoading.value.collection = true
  try {
    if (isInCollection.value) {
      await invoke('remove_from_collection', {
        mediaType: 'movie',
        traktId: movieDetails.value.ids.trakt
      })
      isInCollection.value = false
      Message.success('已移出收藏')
    } else {
      await invoke('add_to_collection', {
        mediaType: 'movie',
        traktId: movieDetails.value.ids.trakt
      })
      isInCollection.value = true
      Message.success('已添加到收藏')
    }
  } catch (error) {
    console.error('收藏操作失败:', error)
    Message.error('操作失败，请稍后重试')
  } finally {
    actionLoading.value.collection = false
  }
}

const handleToggleWatchlist = async () => {
  if (!movieDetails.value?.ids?.trakt) {
    Message.error('电影信息不完整')
    return
  }
  
  actionLoading.value.watchlist = true
  try {
    if (isInWatchlist.value) {
      await invoke('remove_from_watchlist', {
        mediaType: 'movie',
        traktId: movieDetails.value.ids.trakt
      })
      isInWatchlist.value = false
      Message.success('已移出观看清单')
    } else {
      await invoke('add_to_watchlist', {
        mediaType: 'movie',
        traktId: movieDetails.value.ids.trakt
      })
      isInWatchlist.value = true
      Message.success('已添加到观看清单')
    }
  } catch (error) {
    console.error('观看清单操作失败:', error)
    Message.error('操作失败，请稍后重试')
  } finally {
    actionLoading.value.watchlist = false
  }
}

const handleMarkAsWatched = async () => {
  if (!movieDetails.value?.ids?.trakt) {
    Message.error('电影信息不完整')
    return
  }
  
  actionLoading.value.watched = true
  try {
    await invoke('mark_as_watched', {
      mediaType: 'movie',
      traktId: movieDetails.value.ids.trakt
    })
    Message.success('已标记为观看')
  } catch (error) {
    console.error('标记观看失败:', error)
    Message.error('操作失败，请稍后重试')
  } finally {
    actionLoading.value.watched = false
  }
}

// 生命周期
onMounted(() => {
  fetchMovieDetails()
  checkUserStatus()
})

// 检查用户的收藏和观看清单状态
const checkUserStatus = async () => {
  try {
    const movieId = route.params.id
    if (!movieId) return
    
    const numericId = typeof movieId === 'string' ? parseInt(movieId, 10) : Number(movieId)
    if (isNaN(numericId)) return
    
    // 并行获取用户的 watchlist 和 collection
    const [watchlistResult, collectionResult] = await Promise.allSettled([
      invoke<any[]>('get_watchlist', { id: 'me', selectType: 'movies' }),
      invoke<any[]>('get_collection', { id: 'me', selectType: 'movies' })
    ])
    
    // 检查是否在 watchlist 中
    if (watchlistResult.status === 'fulfilled' && watchlistResult.value) {
      isInWatchlist.value = watchlistResult.value.some(
        (item: any) => item.movie?.ids?.trakt === numericId
      )
    }
    
    // 检查是否在 collection 中
    if (collectionResult.status === 'fulfilled' && collectionResult.value) {
      isInCollection.value = collectionResult.value.some(
        (item: any) => item.movie?.ids?.trakt === numericId
      )
    }
    
    console.log('用户状态检查完成:', { isInWatchlist: isInWatchlist.value, isInCollection: isInCollection.value })
  } catch (error) {
    console.error('检查用户状态失败:', error)
  }
}
</script>

<style scoped>
.movie-detail-view {
  min-height: 100vh;
  position: relative;
}

.loading-container,
.error-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 60vh;
  text-align: center;
}

.error-icon {
  color: #f53f3f;
  margin-bottom: 16px;
}

.backdrop {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 500px;
  background-size: cover;
  background-position: center;
  background-repeat: no-repeat;
  filter: blur(20px);
  opacity: 0.3;
  z-index: 0;
}

.detail-container {
  position: relative;
  z-index: 1;
  max-width: 1200px;
  margin: 0 auto;
  padding: 40px 24px;
}

.main-info {
  display: flex;
  gap: 40px;
  align-items: flex-start;
}

.poster-section {
  flex: 0 0 300px;
}

.movie-poster {
  width: 100%;
  border-radius: 16px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
  margin-bottom: 24px;
}

.poster-placeholder {
  width: 100%;
  aspect-ratio: 2/3;
  background: #f5f5f5;
  border-radius: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #bbb;
  margin-bottom: 24px;
}

.action-buttons {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.action-btn {
  width: 100%;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  font-weight: 600;
}

.action-btn.is-active {
  font-weight: 700;
}

.info-section {
  flex: 1;
  min-width: 0;
}

.title-area {
  margin-bottom: 24px;
}

.movie-title {
  font-size: 36px;
  font-weight: 700;
  margin: 0 0 8px 0;
  color: #1d1d1f;
  line-height: 1.2;
  display: flex;
  align-items: center;
  gap: 8px;
}

.original-title {
  font-size: 18px;
  color: #8e8e93;
  margin: 0 0 8px 0;
  font-style: italic;
}

.tagline {
  font-size: 16px;
  color: #6b7280;
  margin: 0;
  font-style: italic;
}

.rating-info {
  margin-bottom: 24px;
}

.rating-score {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 12px;
}

.rating-score .arco-icon {
  color: #faad14;
}

.score {
  font-size: 24px;
  font-weight: 700;
  color: #1d1d1f;
}

.votes {
  font-size: 14px;
  color: #8e8e93;
}

.basic-info {
  display: flex;
  gap: 16px;
  flex-wrap: wrap;
}

.info-item {
  padding: 4px 12px;
  background: rgba(0, 0, 0, 0.05);
  border-radius: 16px;
  font-size: 14px;
  font-weight: 500;
}

.certification {
  background: #f53f3f;
  color: white;
}

.genres {
  margin-bottom: 32px;
}

.genre-tag {
  margin-right: 8px;
  margin-bottom: 8px;
}

.overview {
  margin-bottom: 32px;
}

.overview h3 {
  font-size: 20px;
  font-weight: 600;
  margin: 0 0 12px 0;
  color: #1d1d1f;
}

.overview p {
  font-size: 16px;
  line-height: 1.6;
  color: #374151;
  margin: 0;
}

.detailed-info {
  background: rgba(255, 255, 255, 0.8);
  padding: 24px;
  border-radius: 16px;
  backdrop-filter: blur(10px);
}

.info-grid {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.info-row {
  display: flex;
  align-items: center;
  gap: 12px;
}

.label {
  font-weight: 600;
  color: #6b7280;
  min-width: 80px;
}

.value {
  color: #1d1d1f;
}

.homepage-link {
  color: #165dff;
  text-decoration: none;
  display: flex;
  align-items: center;
  gap: 4px;
}

.homepage-link:hover {
  text-decoration: underline;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .main-info {
    flex-direction: column;
    gap: 24px;
  }
  
  .poster-section {
    flex: none;
    max-width: 250px;
    margin: 0 auto;
  }
  
  .movie-title {
    font-size: 28px;
  }
  
  .action-buttons {
    flex-direction: row;
  }
  
  .action-btn {
    flex: 1;
    height: 40px;
  }
}

@media (max-width: 480px) {
  .movie-title {
    font-size: 24px;
  }
}

.translation-loading {
  opacity: 0.6;
  animation: pulse 1.5s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 0.6; }
  50% { opacity: 1; }
}

.movie-detail-content {
  position: relative;
  min-height: 100vh;
  color: white;
}
</style>
