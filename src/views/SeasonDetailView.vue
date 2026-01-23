<template>
  <div class="season-detail-view">
    <div class="page-container">
      <div v-if="loading" class="loading-container">
        <a-spin :size="40" />
        <p>加载中...</p>
      </div>
      
      <div v-else-if="error" class="error-container">
        <icon-exclamation-circle :size="48" style="color: #f53f3f; margin-bottom: 16px;" />
        <h3>{{ error }}</h3>
        <a-button @click="loadSeason" type="primary">重试</a-button>
      </div>
      
      <div v-else-if="episodes.length > 0" class="season-content">
        <div class="header-section">
          <div class="poster-wrapper" v-if="seasonPoster">
            <img :src="seasonPoster" class="season-poster" />
          </div>
          <div class="info-wrapper">
            <h1 class="season-title">第 {{ seasonNumber }} 季</h1>
            <div class="meta-row">
              <span class="episode-count">{{ episodes.length }} 集</span>
              <span v-if="firstAired" class="air-date">首播: {{ formatDate(firstAired) }}</span>
            </div>
            
            <div class="actions">
              <a-button 
                @click="markSeasonWatched" 
                type="primary" 
                status="success"
                :loading="actionLoading.watched"
              >
                <icon-check /> 标记已看
              </a-button>
              <a-button 
                @click="handleToggleCollection" 
                :type="isInCollection ? 'primary' : 'secondary'"
                :status="isInCollection ? 'success' : undefined"
                :loading="actionLoading.collection"
              >
                <icon-heart /> {{ isInCollection ? '已入库' : '入库' }}
              </a-button>
              <a-button 
                @click="handleToggleWatchlist" 
                :type="isInWatchlist ? 'primary' : 'secondary'"
                :status="isInWatchlist ? 'warning' : undefined"
                :loading="actionLoading.watchlist"
              >
                <icon-bookmark /> {{ isInWatchlist ? '已想看' : '想看' }}
              </a-button>
            </div>
          </div>
        </div>

        <div class="episodes-list">
          <h3>剧集列表</h3>
          <div 
            v-for="episode in episodes" 
            :key="episode.ids.trakt"
            class="episode-card"
            @click="goToEpisodeDetail(episode.number)"
          >
            <div class="episode-screenshot" v-if="episode.images?.screenshot?.length">
               <img :src="getScreenshot(episode)" />
            </div>
            <div class="episode-screenshot-placeholder" v-else>
               <icon-image />
            </div>
            
            <div class="episode-info">
              <div class="episode-header">
                <span class="episode-idx">{{ episode.number }}</span>
                <span class="episode-title">{{ episode.title }}</span>
              </div>
              <div class="episode-meta">
                <span v-if="episode.first_aired">{{ formatDate(episode.first_aired) }}</span>
                <span v-if="episode.rating"><icon-star-fill style="color: #faad14" /> {{ episode.rating.toFixed(1) }}</span>
              </div>
              <p class="episode-overview">{{ episode.overview || '暂无简介' }}</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { Message } from '@arco-design/web-vue'
import { 
  IconImage, IconStarFill, IconCheck, 
  IconExclamationCircle, IconHeart, IconBookmark
} from '@arco-design/web-vue/es/icon'
import type { Episode, Seasons } from '../types/api'
import { useMediaUpdate } from '../composables/useEvent'

const route = useRoute()
const router = useRouter()

const loading = ref(false)
const error = ref('')
const episodes = ref<Episode[]>([])
const seasonNumber = ref(0)
const traktId = ref<number | null>(null)

// 监听后台数据更新
useMediaUpdate((payload) => {
  if (
    payload.type === 'season' &&
    payload.id === Number(route.params.id) &&
    payload.season === Number(route.params.season)
  ) {
    console.log('收到季度详情后台更新')
    episodes.value = payload.data as Episode[]
    Message.info({
      content: '列表已自动刷新',
      position: 'bottom',
      duration: 2000
    })
  }
})
const isInCollection = ref(false)
const isInWatchlist = ref(false)
const actionLoading = ref({
  collection: false,
  watchlist: false,
  watched: false
})

const firstAired = computed(() => episodes.value[0]?.first_aired)

// 尝试从sessionStorage获取剧集海报（因为API可能不返回季度海报，通常用剧集海报）
const seasonPoster = computed(() => {
  const showId = route.params.id
  if (!showId) return null
  const cacheKey = `media_images_${showId}`
  const cached = sessionStorage.getItem(cacheKey)
  if (cached) {
    const images = JSON.parse(cached)
    // 如果有专门的季度海报逻辑可以在这里加，暂用剧集海报
    return images.poster?.[0] ? (images.poster[0].startsWith('http') ? images.poster[0] : `https://${images.poster[0]}`) : null
  }
  return null
})

const formatDate = (dateString: string) => {
  return new Date(dateString).toLocaleDateString('zh-CN')
}

const getScreenshot = (episode: Episode) => {
  if (episode.images?.screenshot?.length) {
    const path = episode.images.screenshot[0]
    return path.startsWith('http') ? path : `https://${path}`
  }
  return ''
}

const loadSeason = async () => {
  const { id, season } = route.params
  if (!id || !season) {
    error.value = '参数错误'
    return
  }
  
  seasonNumber.value = Number(season)
  
  // 尝试从query获取traktId
  if (route.query.traktId) {
    traktId.value = Number(route.query.traktId)
  }

  loading.value = true
  
  try {
    // 如果没有traktId，需要先获取季度列表找到它
    if (!traktId.value) {
      const seasons = await invoke<Seasons>('show_seasons', { id: Number(id) })
      const currentSeason = seasons.find(s => s.number === Number(season))
      if (currentSeason?.ids?.trakt) {
        traktId.value = currentSeason.ids.trakt
      }
    }

    const result = await invoke<Episode[]>('get_season_episodes', {
      id: Number(id),
      season: Number(season)
    })
    episodes.value = result
    
    // 加载完数据后检查用户状态
    if (traktId.value) {
      checkUserStatus()
    }
  } catch (err) {
    console.error('加载季度详情失败:', err)
    error.value = '加载失败，请稍后重试'
  } finally {
    loading.value = false
  }
}

const goToEpisodeDetail = (episodeNum: number) => {
  router.push({
    name: 'episode-detail',
    params: {
      id: route.params.id,
      season: route.params.season,
      episode: episodeNum
    }
  })
}

const handleToggleCollection = async () => {
  if (!traktId.value) {
    Message.warning('无法获取季度信息')
    return
  }
  
  actionLoading.value.collection = true
  try {
    if (isInCollection.value) {
      await invoke('remove_from_collection', {
        mediaType: 'season',
        traktId: traktId.value
      })
      isInCollection.value = false
      Message.success('已移出收藏')
    } else {
      await invoke('add_to_collection', {
        mediaType: 'season',
        traktId: traktId.value
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
  if (!traktId.value) {
    Message.warning('无法获取季度信息')
    return
  }
  
  actionLoading.value.watchlist = true
  try {
    if (isInWatchlist.value) {
      await invoke('remove_from_watchlist', {
        mediaType: 'season',
        traktId: traktId.value
      })
      isInWatchlist.value = false
      Message.success('已移出想看')
    } else {
      await invoke('add_to_watchlist', {
        mediaType: 'season',
        traktId: traktId.value
      })
      isInWatchlist.value = true
      Message.success('已添加到想看')
    }
  } catch (error) {
    console.error('想看操作失败:', error)
    Message.error('操作失败，请稍后重试')
  } finally {
    actionLoading.value.watchlist = false
  }
}

const markSeasonWatched = async () => {
  if (!traktId.value) {
    Message.warning('无法获取季度信息')
    return
  }
  
  actionLoading.value.watched = true
  try {
    await invoke('mark_as_watched', {
      mediaType: 'season',
      traktId: traktId.value
    })
    Message.success('已标记本季为已观看')
  } catch (err) {
    console.error('标记观看失败:', err)
    Message.error('操作失败')
  } finally {
    actionLoading.value.watched = false
  }
}

const checkUserStatus = async () => {
  if (!traktId.value) return
  
  try {
    const [watchlistResult, collectionResult] = await Promise.allSettled([
      invoke<any[]>('get_watchlist', { id: 'me', selectType: 'seasons' }),
      invoke<any[]>('get_collection', { id: 'me', selectType: 'seasons' })
    ])
    
    if (watchlistResult.status === 'fulfilled' && watchlistResult.value) {
      isInWatchlist.value = watchlistResult.value.some(
        (item: any) => item.season?.ids?.trakt === traktId.value
      )
    }
    
    if (collectionResult.status === 'fulfilled' && collectionResult.value) {
      isInCollection.value = collectionResult.value.some(
        (item: any) => item.season?.ids?.trakt === traktId.value
      )
    }
  } catch (error) {
    console.error('检查用户状态失败:', error)
  }
}

onMounted(() => {
  loadSeason()
})
</script>

<style scoped>
.season-detail-view {
  min-height: 100vh;
  background: #f7f8fa;
  padding: 20px;
}

.page-container {
  max-width: 1000px;
  margin: 0 auto;
}

.loading-container, .error-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 60vh;
  color: #86909c;
}

.header-section {
  display: flex;
  gap: 30px;
  margin-bottom: 40px;
  background: white;
  padding: 30px;
  border-radius: 16px;
  box-shadow: 0 4px 10px rgba(0,0,0,0.05);
}

.season-poster {
  width: 160px;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0,0,0,0.2);
}

.info-wrapper {
  flex: 1;
}

.season-title {
  margin: 0 0 10px 0;
  font-size: 32px;
  color: #1d1d1f;
}

.meta-row {
  margin-bottom: 20px;
  color: #86909c;
  font-size: 14px;
}

.actions {
  display: flex;
  gap: 12px;
}

.episodes-list h3 {
  margin-bottom: 20px;
  font-size: 20px;
  color: #1d1d1f;
}

.episode-card {
  display: flex;
  gap: 20px;
  background: white;
  padding: 20px;
  border-radius: 12px;
  margin-bottom: 16px;
  cursor: pointer;
  transition: transform 0.2s, box-shadow 0.2s;
}

.episode-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 20px rgba(0,0,0,0.08);
}

.episode-screenshot {
  width: 200px;
  height: 112px; /* 16:9 */
  flex-shrink: 0;
  border-radius: 8px;
  overflow: hidden;
}

.episode-screenshot img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.episode-screenshot-placeholder {
  width: 200px;
  height: 112px;
  background: #f0f0f0;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #ccc;
  border-radius: 8px;
}

.episode-info {
  flex: 1;
}

.episode-header {
  display: flex;
  gap: 12px;
  margin-bottom: 8px;
  align-items: baseline;
}

.episode-idx {
  font-size: 24px;
  font-weight: 700;
  color: #e5e6eb;
}

.episode-title {
  font-size: 18px;
  font-weight: 600;
  color: #1d1d1f;
}

.episode-meta {
  font-size: 13px;
  color: #86909c;
  margin-bottom: 12px;
}

.episode-overview {
  color: #4e5969;
  font-size: 14px;
  line-height: 1.6;
  margin: 0;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>