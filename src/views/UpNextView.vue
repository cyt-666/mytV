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

      <!-- 内容列表 -->
      <div v-else class="up-next-list">
        <div 
          v-for="item in upNextItems" 
          :key="item.show.ids?.trakt ?? 0" 
          class="up-next-item"
          @click="navigateToEpisode(item)"
        >
          <!-- 海报 -->
          <div class="item-poster">
            <img
              v-if="item.show.images?.poster?.[0]"
              :src="`https://${item.show.images.poster[0]}`"
              :alt="item.show.title"
              class="poster-img"
            />
            <div v-else class="poster-placeholder">
              <icon-image :size="32" />
            </div>
          </div>

          <!-- 内容信息 -->
          <div class="item-content">
            <div class="item-header">
              <h3 class="show-title">{{ getShowTitle(item) }}</h3>
              <span class="last-watched" v-if="item.progress.last_watched_at">
                {{ formatRelativeTime(item.progress.last_watched_at) }}
              </span>
            </div>
            
            <div class="episode-info">
              <span class="season-ep">S{{ item.next_episode.season }}E{{ item.next_episode.number }}</span>
              <span class="episode-title">{{ item.next_episode.title }}</span>
            </div>

            <!-- 进度条区域 -->
            <div class="progress-section">
              <div class="progress-info">
                <span>总进度 {{ Math.round((item.progress.completed / item.progress.aired) * 100) }}%</span>
                <span class="progress-text">{{ item.progress.completed }} / {{ item.progress.aired }} 集</span>
              </div>
              <a-progress
                :percent="item.progress.completed / item.progress.aired"
                :show-text="false"
                :color="{
                  '0%': '#165dff',
                  '100%': '#722ed1'
                }"
                :stroke-width="8"
                class="list-progress"
              />
            </div>
          </div>
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

.up-next-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
  max-width: 800px; /* 限制列表最大宽度，优化阅读体验 */
  margin: 0 auto;
}

.up-next-item {
  display: flex;
  background: #ffffff;
  border-radius: 12px;
  padding: 16px;
  gap: 20px;
  cursor: pointer;
  transition: all 0.2s cubic-bezier(0.25, 0.46, 0.45, 0.94);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.04);
  border: 1px solid rgba(0,0,0,0.02);
}

.up-next-item:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.08);
}

.item-poster {
  width: 80px;
  height: 120px;
  border-radius: 8px;
  overflow: hidden;
  flex-shrink: 0;
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
}

.poster-img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.poster-placeholder {
  width: 100%;
  height: 100%;
  background: #f2f3f5;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #c9ccd4;
}

.item-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: center;
  min-width: 0; /* 防止文本溢出 */
}

.item-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 8px;
}

.show-title {
  font-size: 18px;
  font-weight: 700;
  color: #1d1d1f;
  margin: 0;
  line-height: 1.3;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  margin-right: 12px;
}

.last-watched {
  font-size: 12px;
  color: #86909c;
  white-space: nowrap;
  padding-top: 4px;
}

.episode-info {
  margin-bottom: 16px;
  font-size: 14px;
  color: #4e5969;
  display: flex;
  align-items: center;
}

.season-ep {
  font-weight: 600;
  color: #165dff;
  margin-right: 8px;
  background: rgba(22, 93, 255, 0.1);
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 12px;
}

.episode-title {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.progress-section {
  margin-top: auto;
}

.progress-info {
  display: flex;
  justify-content: space-between;
  font-size: 12px;
  color: #86909c;
  margin-bottom: 6px;
}

.progress-text {
  font-weight: 500;
}

.list-progress :deep(.arco-progress-line-bar) {
  border-radius: 4px;
}

@media (max-width: 640px) {
  .up-next-item {
    padding: 12px;
    gap: 12px;
  }
  
  .item-poster {
    width: 60px;
    height: 90px;
  }
  
  .show-title {
    font-size: 16px;
  }
  
  .episode-info {
    font-size: 13px;
    margin-bottom: 12px;
  }
}

</style>
