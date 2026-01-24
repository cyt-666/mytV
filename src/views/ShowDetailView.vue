<template>
  <div class="show-detail-view">
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
      <a-button @click="fetchShowDetails" type="primary">
        重试
      </a-button>
    </div>

    <!-- 电视剧详情内容 -->
    <div v-else-if="showDetails" class="show-detail-content">
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
              :alt="showDetails.title"
              class="show-poster"
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
                :class="['action-btn', { 'is-active': isWatched }]"
                
                @click="handleMarkAsWatched"
                :loading="actionLoading.watched"
                :status="isWatched ? 'success' : undefined"
              >
                <icon-check v-if="isWatched" />
                <icon-play-arrow v-else />
                {{ isWatched ? '已观看' : '标记已观看' }}
              </a-button>
              <a-button 
                size="large" 
                :class="['action-btn', 'collection-btn', { 'is-active': isInCollection }]"
                :type="isInCollection ? 'primary' : 'secondary'"
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
              <h1 class="show-title">
                {{ chineseTranslation?.title || showDetails.title }}
                <span v-if="translationLoading" class="translation-loading">
                  <a-spin :size="14" />
                </span>
              </h1>
              <p v-if="showDetails.original_title && showDetails.original_title !== showDetails.title" 
                 class="original-title">
                {{ showDetails.original_title }}
              </p>
              <p v-if="chineseTranslation?.tagline || showDetails.tagline" class="tagline">
                {{ chineseTranslation?.tagline || showDetails.tagline }}
              </p>
            </div>

            <!-- 评分和基本信息 -->
            <div class="rating-info">
              <div v-if="showDetails.rating" class="rating-score">
                <icon-star-fill />
                <span class="score">{{ showDetails.rating.toFixed(1) }}</span>
                <span class="votes">({{ showDetails.votes }} 评价)</span>
              </div>
              
              <div class="basic-info">
                <span class="info-item">{{ showDetails.year }}</span>
                <span v-if="showDetails.runtime" class="info-item">{{ formatRuntime(showDetails.runtime) }}</span>
                <span v-if="showDetails.certification" class="info-item certification">{{ showDetails.certification }}</span>
                <span v-if="showDetails.network" class="info-item network">{{ showDetails.network }}</span>
                <span v-if="showDetails.status" class="info-item status">{{ getStatusText(showDetails.status) }}</span>
              </div>
            </div>

            <!-- 类型标签 -->
            <div v-if="showDetails.genres?.length" class="genres">
              <a-tag v-for="genre in showDetails.genres" :key="genre" class="genre-tag">
                {{ genre }}
              </a-tag>
            </div>

            <!-- 播出信息 -->
            <div v-if="showDetails.airs || showDetails.first_aired || showDetails.aired_episodes" class="air-info">
              <h3>播出信息</h3>
              <div class="air-details">
                <div v-if="showDetails.first_aired" class="air-item">
                  <span class="label">首播时间:</span>
                  <span class="value">{{ formatDate(showDetails.first_aired) }}</span>
                </div>
                <div v-if="showDetails.airs?.day && showDetails.airs?.time" class="air-item">
                  <span class="label">播出时间:</span>
                  <span class="value">{{ getAirSchedule() }}</span>
                </div>
                <div v-if="showDetails.aired_episodes" class="air-item">
                  <span class="label">已播集数:</span>
                  <span class="value">{{ showDetails.aired_episodes }} 集</span>
                </div>
              </div>
            </div>

            <!-- 剧情简介 -->
            <div v-if="chineseTranslation?.overview || showDetails.overview" class="overview">
              <h3>剧情简介</h3>
              <p>{{ chineseTranslation?.overview || showDetails.overview }}</p>
            </div>

            <!-- 季度信息 -->
            <div v-if="seasons.length > 0 || seasonsLoading" class="seasons-info">
              <h3>季度信息</h3>
              
              <!-- 季度加载状态 -->
              <div v-if="seasonsLoading" class="seasons-loading">
                <a-spin :size="16" />
                <span>加载季度信息中...</span>
              </div>
              
              <!-- 季度列表 -->
              <div v-else class="seasons-grid">
                <div 
                  v-for="season in seasons" 
                  :key="season.ids.trakt" 
                  class="season-card"
                  @click="goToSeasonDetail(season)"
                >
                  <div class="season-header">
                    <div class="season-header-left">
                      <h4 class="season-title">
                        {{ getSeasonTitle(season) }}
                        <span v-if="!seasonTranslations.has(season.ids.trakt)" class="season-translation-loading">
                          <div class="translation-dot"></div>
                        </span>
                      </h4>
                      
                      <div class="season-actions">
                        <a-button 
                          type="text" 
                          shape="circle"
                          size="small" 
                          @click="(e: Event) => handleSeasonCollection(season, e)"
                          title="添加到收藏"
                        >
                          <icon-heart />
                        </a-button>
                        <a-button 
                          type="text" 
                          shape="circle"
                          size="small" 
                          @click="(e: Event) => handleSeasonWatchlist(season, e)"
                          title="添加到想看"
                        >
                          <icon-bookmark />
                        </a-button>
                      </div>
                    </div>

                    <div class="season-meta">
                      <span v-if="season.first_aired" class="season-year">
                        {{ new Date(season.first_aired).getFullYear() }}
                      </span>
                      <span v-if="season.rating" class="season-rating">
                        <icon-star-fill />
                        {{ season.rating.toFixed(1) }}
                      </span>
                    </div>
                  </div>
                  
                  <div class="season-info">
                    <div class="season-stats">
                      <span v-if="season.episode_count" class="stat-item">
                        {{ season.episode_count }} 集
                      </span>
                      <span v-if="season.aired_episodes && season.aired_episodes !== season.episode_count" class="stat-item">
                        已播 {{ season.aired_episodes }} 集
                      </span>
                      <span v-if="season.network" class="stat-item network">
                        {{ season.network }}
                      </span>
                    </div>
                    
                    <p v-if="getSeasonOverview(season)" class="season-overview">
                      {{ getSeasonOverview(season) }}
                    </p>
                  </div>
                </div>
              </div>
            </div>

            <!-- 详细信息 -->
            <div class="detailed-info">
              <div class="info-grid">
                <div v-if="showDetails.country" class="info-row">
                  <span class="label">制片国家:</span>
                  <span class="value">{{ showDetails.country.toUpperCase() }}</span>
                </div>
                <div v-if="showDetails.languages?.length" class="info-row">
                  <span class="label">语言:</span>
                  <span class="value">{{ showDetails.languages.join(', ') }}</span>
                </div>
                <div v-if="showDetails.homepage" class="info-row">
                  <span class="label">官方网站:</span>
                  <a :href="showDetails.homepage" target="_blank" class="homepage-link">
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
import { useRoute, useRouter } from 'vue-router'
import { 
  IconImage, IconStarFill, IconPlayArrow, IconHeart,
  IconBookmark, IconExclamationCircle, IconLink, IconCheck
} from '@arco-design/web-vue/es/icon'
import { Message } from '@arco-design/web-vue'
import { invoke } from "@tauri-apps/api/core"
import type { ShowDetails, MovieImages, Seasons } from '../types/api'
import { getShowChineseTranslation, getSeasonChineseTranslation, type TranslationResult } from '../utils/translation'

const route = useRoute()
const router = useRouter()

// 响应式数据
const showDetails = ref<ShowDetails | null>(null)
const loading = ref(false)
const error = ref('')
const showImages = ref<MovieImages | null>(null)
const chineseTranslation = ref<TranslationResult | null>(null)
const translationLoading = ref(false)
const seasons = ref<Seasons>([])
const seasonsLoading = ref(false)
const seasonTranslations = ref<Map<number, TranslationResult>>(new Map())

const isInCollection = ref(false)
const isInWatchlist = ref(false)
const isWatched = ref(false)
const actionLoading = ref({
  collection: false,
  watchlist: false,
  watched: false
})

// 计算属性
const posterUrl = computed(() => {
  if (showImages.value?.poster?.length) {
    const posterPath = showImages.value.poster[0]
    // 如果已经是完整URL，直接使用；否则添加https://
    if (posterPath.startsWith('http')) {
      return posterPath
    }
    return `https://${posterPath}`
  }
  return null
})

const backdropUrl = computed(() => {
  if (showImages.value?.fanart?.length) {
    // 优先使用fanart作为背景图
    const fanartPath = showImages.value.fanart[0]
    if (fanartPath.startsWith('http')) {
      return fanartPath
    }
    return `https://${fanartPath}`
  } else if (showImages.value?.banner?.length) {
    // 备选使用banner
    const bannerPath = showImages.value.banner[0]
    if (bannerPath.startsWith('http')) {
      return bannerPath
    }
    return `https://${bannerPath}`
  } else if (showImages.value?.poster?.length) {
    // 最后备选使用poster作为背景
    const posterPath = showImages.value.poster[0]
    if (posterPath.startsWith('http')) {
      return posterPath
    }
    return `https://${posterPath}`
  }
  return null
})

// 方法
const fetchShowDetails = async () => {
  const showId = route.params.id
  if (!showId) {
    error.value = '无效的电视剧ID'
    return
  }

  // 从sessionStorage加载图片信息
  const cacheKey = `media_images_${showId}`
  const cachedImages = sessionStorage.getItem(cacheKey)
  if (cachedImages) {
    try {
      showImages.value = JSON.parse(cachedImages)
    } catch (e) {
      console.warn('解析缓存的图片信息失败:', e)
    }
  }

  // 确保ID是数字类型
  const numericId = typeof showId === 'string' ? parseInt(showId, 10) : Number(showId)
  if (isNaN(numericId)) {
    error.value = '无效的电视剧ID格式'
    return
  }

  loading.value = true
  error.value = ''

  try {
    // 立即获取电视剧详情，不等待翻译
    const details = await invoke<ShowDetails>("show_details", { id: numericId })
    showDetails.value = details
    
    console.log('电视剧详情:', details)
    
    // 异步加载翻译，不阻塞页面显示
    loadTranslationAsync(numericId)
    
    // 异步加载季度信息
    loadSeasonsAsync(numericId)
  } catch (err) {
    console.error('获取电视剧详情失败:', err)
    error.value = '获取电视剧详情失败，请稍后重试'
    Message.error('获取电视剧详情失败')
  } finally {
    loading.value = false
  }
}

// 异步加载翻译
const loadTranslationAsync = async (showId: number) => {
  translationLoading.value = true
  try {
    // 在下一个事件循环中加载翻译，确保页面先渲染
    await new Promise(resolve => setTimeout(resolve, 0))
    
    console.log('开始加载电视剧翻译:', showId)
    const translation = await getShowChineseTranslation(showId)
    chineseTranslation.value = translation
    
    console.log('电视剧翻译加载完成:', translation)
  } catch (error) {
    console.error('电视剧翻译加载失败:', error)
    // 翻译失败不影响页面功能
  } finally {
    translationLoading.value = false
  }
}

// 异步加载季度信息
const loadSeasonsAsync = async (showId: number) => {
  seasonsLoading.value = true
  try {
    console.log('开始加载季度信息:', showId)
    const seasonsData = await invoke<Seasons>("show_seasons", { id: showId })
    seasons.value = seasonsData || []
    
    console.log('季度信息加载完成:', seasonsData)
    
    // 异步加载每个季度的翻译
    if (seasonsData && seasonsData.length > 0) {
      loadSeasonTranslationsAsync(showId, seasonsData)
    }
  } catch (error) {
    console.error('季度信息加载失败:', error)
    seasons.value = []
  } finally {
    seasonsLoading.value = false
  }
}

// 异步加载所有季度的翻译
const loadSeasonTranslationsAsync = async (showId: number, seasonsList: Seasons) => {
  console.log('开始加载季度翻译:', seasonsList.length, '个季度')
  
  // 为每个季度异步加载翻译
  const translationPromises = seasonsList.map(async (season) => {
    try {
      const translation = await getSeasonChineseTranslation(showId, season.number)
      if (translation) {
        seasonTranslations.value.set(season.ids.trakt, translation)
        console.log(`季度 ${season.number} 翻译加载完成:`, translation)
      }
    } catch (error) {
      console.warn(`季度 ${season.number} 翻译加载失败:`, error)
    }
  })
  
  // 等待所有翻译加载完成
  await Promise.allSettled(translationPromises)
  console.log('所有季度翻译加载完成')
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

const getStatusText = (status: string) => {
  const statusMap: Record<string, string> = {
    'returning series': '连载中',
    'continuing': '连载中',
    'ended': '已完结',
    'canceled': '已取消',
    'planned': '计划中',
    'in production': '制作中',
    'pilot': '试播集'
  }
  return statusMap[status.toLowerCase()] || status
}

const getAirSchedule = () => {
  if (!showDetails.value?.airs) return ''
  
  const { day, time, timezone } = showDetails.value.airs
  const dayMap: Record<string, string> = {
    'monday': '周一',
    'tuesday': '周二', 
    'wednesday': '周三',
    'thursday': '周四',
    'friday': '周五',
    'saturday': '周六',
    'sunday': '周日'
  }
  
  const dayText = dayMap[day?.toLowerCase() || ''] || day
  const timeText = time ? ` ${time}` : ''
  const timezoneText = timezone ? ` (${timezone})` : ''
  
  return `${dayText}${timeText}${timezoneText}`
}

const handleImageError = (event: Event) => {
  const target = event.target as HTMLImageElement
  if (target) {
    target.style.display = 'none'
  }
}

const handleToggleCollection = async () => {
  if (!showDetails.value?.ids?.trakt) {
    Message.error('剧集信息不完整')
    return
  }
  
  actionLoading.value.collection = true
  try {
    if (isInCollection.value) {
      await invoke('remove_from_collection', {
        mediaType: 'show',
        traktId: showDetails.value.ids.trakt
      })
      isInCollection.value = false
      Message.success('已移出收藏')
    } else {
      await invoke('add_to_collection', {
        mediaType: 'show',
        traktId: showDetails.value.ids.trakt
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
  if (!showDetails.value?.ids?.trakt) {
    Message.error('剧集信息不完整')
    return
  }
  
  actionLoading.value.watchlist = true
  try {
    if (isInWatchlist.value) {
      await invoke('remove_from_watchlist', {
        mediaType: 'show',
        traktId: showDetails.value.ids.trakt
      })
      isInWatchlist.value = false
      Message.success('已移出观看清单')
    } else {
      await invoke('add_to_watchlist', {
        mediaType: 'show',
        traktId: showDetails.value.ids.trakt
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
  if (!showDetails.value?.ids?.trakt) {
    Message.error('剧集信息不完整')
    return
  }
  
  actionLoading.value.watched = true
  try {
    await invoke('mark_as_watched', {
      mediaType: 'show',
      traktId: showDetails.value.ids.trakt
    })
    isWatched.value = true
    Message.success('已标记为观看')
  } catch (error) {
    console.error('标记观看失败:', error)
    Message.error('操作失败，请稍后重试')
  } finally {
    actionLoading.value.watched = false
  }
}

const goToSeasonDetail = (season: any) => {
  if (!showDetails.value?.ids?.trakt) return
  
  router.push({
    name: 'season-detail',
    params: {
      id: showDetails.value.ids.trakt,
      season: season.number
    },
    query: {
      traktId: season.ids.trakt
    }
  })
}

const handleSeasonCollection = async (season: any, event: Event) => {
  event.stopPropagation()
  if (!season.ids?.trakt) return
  
  try {
    await invoke('add_to_collection', {
      mediaType: 'season',
      traktId: season.ids.trakt
    })
    Message.success(`第 ${season.number} 季已添加到收藏`)
  } catch (error) {
    console.error('收藏季度失败:', error)
    Message.error('操作失败')
  }
}

const handleSeasonWatchlist = async (season: any, event: Event) => {
  event.stopPropagation()
  if (!season.ids?.trakt) return
  
  try {
    await invoke('add_to_watchlist', {
      mediaType: 'season',
      traktId: season.ids.trakt
    })
    Message.success(`第 ${season.number} 季已添加到想看`)
  } catch (error) {
    console.error('添加到想看失败:', error)
    Message.error('操作失败')
  }
}

// 获取季度的翻译标题
const getSeasonTitle = (season: any) => {
  const translation = seasonTranslations.value.get(season.ids.trakt)
  return translation?.title || season.title || `第 ${season.number} 季`
}

// 获取季度的翻译简介
const getSeasonOverview = (season: any) => {
  const translation = seasonTranslations.value.get(season.ids.trakt)
  return translation?.overview || season.overview
}

// 生命周期
onMounted(() => {
  fetchShowDetails()
  checkUserStatus()
})

// 检查用户的收藏和观看清单状态
const checkUserStatus = async () => {
  try {
    const showId = route.params.id
    if (!showId) return
    
    const numericId = typeof showId === 'string' ? parseInt(showId, 10) : Number(showId)
    if (isNaN(numericId)) return
    
    // 并行获取用户的 watchlist, collection 和 watched history
    const [watchlistResult, collectionResult, watchedResult] = await Promise.allSettled([
      invoke<any[]>('get_watchlist', { id: 'me', selectType: 'shows' }),
      invoke<any[]>('get_collection', { id: 'me', selectType: 'shows' }),
      invoke<any[]>('get_watched', { id: 'me', selectType: 'shows', noSeason: true })
    ])
    
    // 检查是否在 watchlist 中
    if (watchlistResult.status === 'fulfilled' && watchlistResult.value) {
      isInWatchlist.value = watchlistResult.value.some(
        (item: any) => item.show?.ids?.trakt === numericId
      )
    }
    
    // 检查是否在 collection 中
    if (collectionResult.status === 'fulfilled' && collectionResult.value) {
      isInCollection.value = collectionResult.value.some(
        (item: any) => item.show?.ids?.trakt === numericId
      )
    }

    // 检查是否在 watched history 中
    if (watchedResult.status === 'fulfilled' && watchedResult.value) {
      isWatched.value = watchedResult.value.some(
        (item: any) => item.show?.ids?.trakt === numericId
      )
    }
    
    console.log('用户状态检查完成:', { 
      isInWatchlist: isInWatchlist.value, 
      isInCollection: isInCollection.value,
      isWatched: isWatched.value
    })
  } catch (error) {
    console.error('检查用户状态失败:', error)
  }
}
</script>

<style scoped>
.show-detail-view {
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

.show-poster {
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

/* 覆盖 Arco 默认样式，实现紫色收藏按钮 */
.action-btn.is-active.collection-btn {
  background-color: #722ed1 !important;
  border-color: #722ed1 !important;
  color: white !important;
}

.action-btn.is-active.collection-btn:hover {
  background-color: #5b25a8 !important;
  border-color: #5b25a8 !important;
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

.show-title {
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

.network {
  background: #165dff;
  color: white;
}

.status {
  background: #00b42a;
  color: white;
}

.genres {
  margin-bottom: 32px;
}

.genre-tag {
  margin-right: 8px;
  margin-bottom: 8px;
}

.air-info {
  margin-bottom: 32px;
}

.air-info h3 {
  font-size: 20px;
  font-weight: 600;
  margin: 0 0 12px 0;
  color: #1d1d1f;
}

.air-details {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.air-item {
  display: flex;
  align-items: center;
  gap: 12px;
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

.seasons-info {
  margin-bottom: 32px;
}

.seasons-info h3 {
  font-size: 20px;
  font-weight: 600;
  margin: 0 0 12px 0;
  color: #1d1d1f;
}

.seasons-loading {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 20px;
  color: #8e8e93;
}

.seasons-grid {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.season-card {
  background: rgba(255, 255, 255, 0.8);
  padding: 24px;
  border-radius: 16px;
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  transition: all 0.3s ease;
  cursor: pointer;
}

.season-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 25px rgba(0, 0, 0, 0.1);
}

.season-card.is-expanded {
  background: white;
  border-color: rgba(var(--primary-6), 0.3);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  transform: none;
}

.season-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  margin-bottom: 16px;
  gap: 16px;
}

.season-header-left {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 12px;
  flex-wrap: wrap;
}

.season-title {
  font-size: 20px;
  font-weight: 600;
  margin: 0;
  color: #1d1d1f;
  display: flex;
  align-items: center;
  gap: 8px;
}

.season-actions {
  display: flex;
  gap: 4px;
  opacity: 0;
  transition: opacity 0.2s;
}

.season-card:hover .season-actions,
.season-card.is-expanded .season-actions {
  opacity: 1;
}

.season-meta {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-shrink: 0;
}

.expand-icon {
  color: #8e8e93;
  margin-left: 4px;
}

/* ... existing styles ... */

.episodes-container {
  margin-top: 20px;
  padding-top: 20px;
  border-top: 1px solid rgba(0, 0, 0, 0.06);
  animation: slide-down 0.3s ease-out;
}

@keyframes slide-down {
  from { opacity: 0; transform: translateY(-10px); }
  to { opacity: 1; transform: translateY(0); }
}

.episodes-loading {
  padding: 24px;
  text-align: center;
  color: #8e8e93;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
}

.episodes-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.episode-item {
  padding: 16px;
  background: rgba(0, 0, 0, 0.02);
  border-radius: 12px;
  transition: background 0.2s;
  cursor: pointer;
}

.episode-item:hover {
  background: rgba(0, 0, 0, 0.04);
}

.episode-header {
  display: flex;
  gap: 16px;
  margin-bottom: 8px;
}

.episode-idx {
  font-size: 20px;
  font-weight: 700;
  color: #c9c9c9;
  min-width: 32px;
  text-align: center;
  line-height: 1.2;
}

.episode-main {
  flex: 1;
}

.episode-top {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 4px;
  flex-wrap: wrap;
}

.episode-title {
  font-size: 16px;
  font-weight: 600;
  color: #1d1d1f;
}

.episode-rating {
  font-size: 13px;
  color: #faad14;
  display: flex;
  align-items: center;
  gap: 4px;
  background: rgba(250, 173, 20, 0.1);
  padding: 2px 6px;
  border-radius: 4px;
}

.episode-meta {
  font-size: 13px;
  color: #8e8e93;
}

.episode-overview {
  margin: 0;
  font-size: 14px;
  color: #555;
  padding-left: 48px;
  line-height: 1.6;
}

.season-year {
  font-size: 14px;
  color: #8e8e93;
  background: rgba(0, 0, 0, 0.05);
  padding: 4px 8px;
  border-radius: 8px;
}

.season-rating {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 14px;
  font-weight: 600;
  color: #1d1d1f;
}

.season-rating .arco-icon {
  color: #faad14;
  font-size: 12px;
}

.season-info {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.season-stats {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
}

.stat-item {
  font-size: 14px;
  font-weight: 500;
  color: #1d1d1f;
  background: rgba(0, 0, 0, 0.05);
  padding: 4px 8px;
  border-radius: 8px;
}

.stat-item.network {
  background: #165dff;
  color: white;
}

.season-overview {
  font-size: 14px;
  line-height: 1.6;
  color: #6b7280;
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
  
  .show-title {
    font-size: 28px;
  }
  
  .seasons-grid {
    grid-template-columns: 1fr;
    gap: 16px;
  }
}

@media (max-width: 480px) {
  .show-title {
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

.season-translation-loading {
  position: relative;
}

.season-translation-loading .translation-dot {
  width: 4px;
  height: 4px;
  background: #165dff;
  border-radius: 50%;
  animation: pulse-dot 1.5s ease-in-out infinite;
}

@keyframes pulse-dot {
  0%, 100% { opacity: 0; }
  50% { opacity: 1; }
}

.show-detail-content {
  position: relative;
  min-height: 100vh;
  color: white;
}
</style>
