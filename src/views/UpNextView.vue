<template>
  <div class="up-next-view">
    <div class="page-container">
      <div class="page-header">
        <h1 class="page-title">待看</h1>
        <p class="page-subtitle">继续观看你正在追的剧集</p>
      </div>

      <!-- 加载状态 -->
      <div v-if="loading" class="loading-container">
        <a-spin :size="40" tip="获取追剧进度..." />
      </div>

      <!-- 空状态 -->
      <div v-else-if="upNextItems.length === 0" class="empty-state">
        <a-empty description="还没有正在观看的剧集" />
      </div>

      <!-- 内容网格 -->
      <div v-else class="up-next-grid">
        <div 
          v-for="item in upNextItems" 
          :key="item.show.ids?.trakt ?? 0" 
          class="up-next-card-wrapper"
          @click="navigateToEpisode(item)"
        >
          <a-card class="up-next-card" :bordered="false" hoverable>
            <template #cover>
              <div class="poster-wrapper">
                <img
                  v-if="item.show.images?.poster?.[0]"
                  :src="`https://${item.show.images.poster[0]}`"
                  :alt="item.show.title"
                  class="poster-img"
                />
                <div v-else class="poster-placeholder">
                  <icon-image :size="40" />
                </div>
                <div class="progress-overlay">
                  <a-progress
                    :percent="(item.progress.completed / item.progress.aired) * 100"
                    :show-text="false"
                    status="success"
                    :stroke-width="4"
                    class="card-progress"
                  />
                </div>
              </div>
            </template>
            <div class="card-content">
              <div class="show-title">{{ getShowTitle(item) }}</div>
              <div class="episode-info">
                S{{ item.next_episode.season }}E{{ item.next_episode.number }} - {{ item.next_episode.title }}
              </div>
              <div class="meta-info">
                <span class="progress-text">已看 {{ item.progress.completed }}/{{ item.progress.aired }}</span>
                <span class="last-watched" v-if="item.progress.last_watched_at">
                  {{ formatRelativeTime(item.progress.last_watched_at) }}
                </span>
              </div>
            </div>
          </a-card>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { IconImage } from '@arco-design/web-vue/es/icon'
import { useAuth } from '../composables/useAuth'
import type { UpNextItem } from '../types/api'
import { getShowChineseTranslation, type TranslationResult } from '../utils/translation'

defineOptions({ name: 'UpNextView' })

const router = useRouter()
const { userInfo } = useAuth()

const loading = ref(false)
const upNextItems = ref<UpNextItem[]>([])
const showTranslations = ref<Map<number, TranslationResult>>(new Map())

const formatRelativeTime = (time: string | undefined) => {
  if (!time) return ''
  const date = new Date(time)
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  
  const seconds = Math.floor(diff / 1000)
  const minutes = Math.floor(seconds / 60)
  const hours = Math.floor(minutes / 60)
  const days = Math.floor(hours / 24)
  
  if (days > 30) return date.toLocaleDateString()
  if (days > 0) return `${days}天前`
  if (hours > 0) return `${hours}小时前`
  if (minutes > 0) return `${minutes}分钟前`
  return '刚刚'
}

const loadUpNext = async () => {
  if (!userInfo.value?.username) return
  
  loading.value = true
  try {
    const result = await invoke<UpNextItem[]>('get_up_next', {
      username: userInfo.value.username
    })
    upNextItems.value = result
    loadTranslations(result)
  } catch (error) {
    console.error('加载待看剧集失败:', error)
  } finally {
    loading.value = false
  }
}

const loadTranslations = async (items: UpNextItem[]) => {
  const uniqueShowIds = [...new Set(items.map(i => i.show.ids?.trakt).filter(Boolean))] as number[]
  for (const showId of uniqueShowIds) {
    if (!showTranslations.value.has(showId)) {
      const translation = await getShowChineseTranslation(showId)
      if (translation) {
        showTranslations.value.set(showId, translation)
      }
    }
  }
}

const getShowTitle = (item: UpNextItem) => {
  const showId = item.show.ids?.trakt
  if (showId && showTranslations.value.has(showId)) {
    return showTranslations.value.get(showId)?.title || item.show.title
  }
  return item.show.title
}

const navigateToEpisode = (item: UpNextItem) => {
  if (!item.show.ids?.trakt) return
  router.push({
    name: 'episode-detail',
    params: {
      id: String(item.show.ids.trakt),
      season: String(item.next_episode.season),
      episode: String(item.next_episode.number)
    }
  })
}

onMounted(() => {
  loadUpNext()
})
</script>

<style scoped>
.up-next-view {
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
  letter-spacing: -0.02em;
}

.page-subtitle {
  font-size: 16px;
  color: #8e8e93;
  margin: 0;
}

.loading-container {
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 100px 0;
}

.empty-state {
  padding: 100px 0;
}

.up-next-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
  gap: 24px;
}

.up-next-card {
  border-radius: 12px;
  overflow: hidden;
  background: #ffffff;
  transition: all 0.3s cubic-bezier(0.25, 0.46, 0.45, 0.94);
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.05);
  height: 100%;
}

.up-next-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 12px 24px rgba(0, 0, 0, 0.1);
}

.poster-wrapper {
  position: relative;
  width: 100%;
  aspect-ratio: 2/3;
  overflow: hidden;
  background: #f2f2f7;
}

.poster-img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.poster-placeholder {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #c7c7cc;
}

.progress-overlay {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  padding: 0;
  background: rgba(0, 0, 0, 0.3);
}

.card-progress {
  display: block;
}

.card-content {
  padding: 16px;
}

.show-title {
  font-size: 16px;
  font-weight: 600;
  color: #1d1d1f;
  margin-bottom: 4px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.episode-info {
  font-size: 13px;
  color: #165dff;
  font-weight: 500;
  margin-bottom: 12px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.meta-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 12px;
  color: #8e8e93;
}

.progress-text {
  background: #f2f2f7;
  padding: 2px 6px;
  border-radius: 4px;
}

.last-watched {
  font-style: italic;
}

@media (max-width: 768px) {
  .up-next-grid {
    grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
    gap: 16px;
  }
  
  .page-title {
    font-size: 24px;
  }
  
  .card-content {
    padding: 12px;
  }
  
  .show-title {
    font-size: 14px;
  }
  
  .episode-info {
    font-size: 12px;
  }
}
</style>
