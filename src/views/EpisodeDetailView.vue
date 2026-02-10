<template>
  <div class="episode-detail-view">
    <div class="page-container">
      <div v-if="loading" class="loading-container">
        <a-spin :size="40" />
        <p>加载中...</p>
      </div>
      
      <div v-else-if="error" class="error-container">
        <icon-exclamation-circle :size="48" style="color: #f53f3f; margin-bottom: 16px;" />
        <h3>{{ error }}</h3>
        <a-button @click="loadEpisode" type="primary">重试</a-button>
      </div>
      
      <div v-else-if="episode" class="episode-content">
        <div class="main-section">
          <div class="screenshot-wrapper">
            <img v-if="screenshotUrl" :src="screenshotUrl" :alt="episode.title" class="screenshot" />
            <div v-else class="screenshot-placeholder">
              <icon-image :size="60" />
            </div>
          </div>
          
          <div class="info-wrapper">
            <h1 class="episode-title">
              <div v-if="fallbackData.showTitle" class="show-name">{{ fallbackData.showTitle }}</div>
              <span class="episode-number">{{ episode.number }}.</span>
              {{ episode.title }}
            </h1>
            
            <div class="meta-row">
              <span class="season-tag">第 {{ episode.season }} 季</span>
              <span v-if="episode.first_aired" class="air-date">{{ formatDate(episode.first_aired) }}</span>
              <span v-if="episode.runtime" class="runtime">{{ formatRuntime(episode.runtime) }}</span>
              <span v-if="episode.rating" class="rating">
                <icon-star-fill style="color: #faad14" /> {{ episode.rating.toFixed(1) }}
              </span>
            </div>
            
            <p class="overview" v-if="episode.overview">{{ episode.overview }}</p>
            <p class="overview" v-else>暂无简介</p>
            
            <div class="actions">
              <a-button type="primary" status="success" @click="markWatched">
                <template #icon><icon-check /></template>
                标记为已观看
              </a-button>
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
import { invoke } from '@tauri-apps/api/core'
import { Message } from '@arco-design/web-vue'
import { IconImage, IconStarFill, IconCheck, IconExclamationCircle } from '@arco-design/web-vue/es/icon'
import type { Episode } from '../types/api'
import { useMediaUpdate } from '../composables/useEvent'

const route = useRoute()

const loading = ref(false)
const error = ref('')
const episode = ref<Episode | null>(null)

// 监听后台数据更新
useMediaUpdate((payload) => {
  if (
    payload.type === 'episode' &&
    payload.id === Number(route.params.id) &&
    payload.season === Number(route.params.season) &&
    payload.episode === Number(route.params.episode)
  ) {
    console.log('收到单集详情后台更新')
    episode.value = payload.data as Episode
    Message.info({
      content: '详情已自动刷新',
      position: 'bottom',
      duration: 2000
    })
  }
})

// 从路由状态获取的后备数据
const fallbackData = ref({
  showTitle: '',
  posterUrl: '',
  backdropUrl: ''
})

const screenshotUrl = computed(() => {
  // 1. 优先使用剧集截图
  if (episode.value?.images?.screenshot?.length) {
    const path = episode.value.images.screenshot[0]
    return path.startsWith('http') ? path : `https://${path}`
  }
  // 2. 后备：使用传递过来的背景图或海报
  return fallbackData.value.backdropUrl || fallbackData.value.posterUrl || null
})

const formatDate = (dateString: string) => {
  return new Date(dateString).toLocaleDateString('zh-CN')
}

const formatRuntime = (minutes: number) => {
  return `${minutes}分钟`
}

const loadEpisode = async () => {
  const { id, season, episode: epNum } = route.params
  if (!id || !season || !epNum) {
    error.value = '参数错误'
    return
  }
  
  loading.value = true
  error.value = ''
  
  try {
    const result = await invoke<Episode>('get_episode_details', {
      id: Number(id),
      season: Number(season),
      episode: Number(epNum)
    })
    episode.value = result
  } catch (err) {
    console.error('加载剧集详情失败:', err)
    error.value = '加载失败，请稍后重试'
  } finally {
    loading.value = false
  }
}

const markWatched = async () => {
  if (!episode.value?.ids?.trakt) return
  
  try {
    await invoke('mark_as_watched', {
      mediaType: 'episode',
      traktId: episode.value.ids.trakt
    })
    Message.success('已标记为观看')
  } catch (err) {
    console.error('标记观看失败:', err)
    Message.error('操作失败')
  }
}

onMounted(() => {
  // 读取路由传递的状态
  const state = history.state as Record<string, any>
  if (state) {
    if (state.showTitle) fallbackData.value.showTitle = state.showTitle
    if (state.posterUrl) fallbackData.value.posterUrl = state.posterUrl
    if (state.backdropUrl) fallbackData.value.backdropUrl = state.backdropUrl
  }
  
  loadEpisode()
})
</script>

<style scoped>
.episode-detail-view {
  min-height: 100vh;
  /* background: #f7f8fa; removed for glass transparency */
  padding-bottom: 40px;
}

.page-container {
  max-width: 1000px;
  margin: 0 auto;
  padding: 20px;
}

.loading-container, .error-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 60vh;
  color: var(--glass-text-secondary);
}

.navigation-bar {
  margin-bottom: 20px;
}

.main-section {
  background: var(--glass-bg);
  backdrop-filter: var(--glass-blur);
  -webkit-backdrop-filter: var(--glass-blur);
  border: 1px solid var(--glass-border);
  border-radius: 16px;
  overflow: hidden;
  box-shadow: var(--glass-shadow);
}

.screenshot-wrapper {
  width: 100%;
  aspect-ratio: 16/9;
  background: #000;
  position: relative;
}

.screenshot {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.screenshot-placeholder {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--glass-overlay-bg);
  color: var(--glass-text-secondary);
}

.info-wrapper {
  padding: 32px;
}

.episode-title {
  font-size: 28px;
  font-weight: 700;
  margin: 0 0 12px 0;
  color: var(--glass-text);
}

.show-name {
  font-size: 16px;
  color: var(--glass-text-secondary);
  font-weight: 500;
  margin-bottom: 4px;
}

.episode-number {
  color: var(--glass-text-secondary);
  margin-right: 8px;
}

.meta-row {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-bottom: 24px;
  font-size: 14px;
  color: var(--glass-text-secondary);
}

.season-tag {
  background: rgba(22, 93, 255, 0.1);
  color: #165dff;
  padding: 2px 8px;
  border-radius: 4px;
  font-weight: 600;
}

.overview {
  font-size: 16px;
  line-height: 1.8;
  color: var(--glass-text);
  margin-bottom: 32px;
}

.actions {
  border-top: 1px solid var(--glass-border);
  padding-top: 24px;
}
</style>