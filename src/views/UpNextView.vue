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
        
        <!-- 加载更多提示 -->
        <div v-if="loadingMore" class="loading-more">
          <a-spin /> 正在加载更多...
        </div>
        <div v-if="!hasMore && upNextItems.length > 0" class="no-more">
          没有更多内容了
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, nextTick } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { IconImage } from '@arco-design/web-vue/es/icon'
import { useAuth } from '../composables/useAuth'
import { useUserDataUpdate } from '../composables/useEvent'
import type { UpNextItem } from '../types/api'
import { getShowChineseTranslation, type TranslationResult } from '../utils/translation'

defineOptions({ name: 'UpNextView' })

const router = useRouter()
const { userInfo } = useAuth()

const loading = ref(false)
const loadingMore = ref(false)
const hasMore = ref(true)
const page = ref(1)
const upNextItems = ref<UpNextItem[]>([])
const showTranslations = ref<Map<number, TranslationResult>>(new Map())

useUserDataUpdate((payload) => {
  if (!userInfo.value?.username) return
  
  const expectedKeyPrefix = `up_next_${userInfo.value.username}_p`
  if (payload.key.startsWith(expectedKeyPrefix)) {
    const pageNum = parseInt(payload.key.replace(expectedKeyPrefix, ''), 10)
    const newData = payload.data as UpNextItem[]
    
    if (pageNum === 1) {
      upNextItems.value = newData
    } else {
      const startIdx = (pageNum - 1) * 20
      upNextItems.value.splice(startIdx, newData.length, ...newData)
    }
    
    loadTranslations(newData)
    console.log(`Up Next 第 ${pageNum} 页已后台刷新，共 ${newData.length} 条`)
  }
})

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

const loadTranslations = async (items: UpNextItem[]) => {
  const uniqueShowIds = [...new Set(items.map(i => i.show.ids?.trakt).filter(Boolean))] as number[]
  
  // Filter out IDs we already have
  const idsToFetch = uniqueShowIds.filter(id => !showTranslations.value.has(id))
  
  // Parallel fetch
  await Promise.all(idsToFetch.map(async (showId) => {
    try {
      const translation = await getShowChineseTranslation(showId)
      if (translation) {
        showTranslations.value.set(showId, translation)
      }
    } catch (e) {
      console.warn(`Failed to load translation for show ${showId}`, e)
    }
  }))
}

const loadUpNext = async (isLoadMore = false) => {
  if (!userInfo.value?.username) return
  if (isLoadMore && (!hasMore.value || loadingMore.value)) return
  
  if (isLoadMore) {
    loadingMore.value = true
  } else {
    loading.value = true
    page.value = 1
    upNextItems.value = []
  }

  try {
    const result = await invoke<UpNextItem[]>('get_up_next', {
      username: userInfo.value.username,
      page: page.value,
      limit: 20
    })
    
    if (result.length < 20) {
      hasMore.value = false
    } else {
      hasMore.value = true
      page.value++
    }

    if (isLoadMore) {
      upNextItems.value.push(...result)
    } else {
      upNextItems.value = result
    }
    
    loadTranslations(result)
  } catch (error) {
    console.error('加载待看剧集失败:', error)
  } finally {
    loading.value = false
    loadingMore.value = false
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
  
  const posterPath = item.show.images?.poster?.[0]
  const posterUrl = posterPath ? (posterPath.startsWith('http') ? posterPath : `https://${posterPath}`) : undefined
  
  const backdropPath = item.show.images?.fanart?.[0]
  const backdropUrl = backdropPath ? (backdropPath.startsWith('http') ? backdropPath : `https://${backdropPath}`) : undefined

  router.push({
    name: 'episode-detail',
    params: {
      id: String(item.show.ids.trakt),
      season: String(item.next_episode.season),
      episode: String(item.next_episode.number)
    },
    state: {
      posterUrl,
      backdropUrl,
      showTitle: getShowTitle(item)
    }
  })
}

const handleScroll = (e: Event) => {
  const target = e.target as HTMLElement
  if (!target) return
  
  // 增加调试日志
  console.log('Scroll:', target.scrollTop, target.clientHeight, target.scrollHeight)
  
  // 增加阈值到 300px，确保更容易触发
  if (target.scrollTop + target.clientHeight >= target.scrollHeight - 300) {
    loadUpNext(true)
  }
}

onMounted(async () => {
  loadUpNext()
  await nextTick()
  
  // 尝试在更大的范围内查找滚动容器
  // AppLayout 的 .app-content 是 overflow-y: auto 的容器
  const content = document.querySelector('.app-content')
  
  if (content) {
    console.log('UpNextView: Scroll listener attached to .app-content')
    content.addEventListener('scroll', handleScroll)
  } else {
    console.error('UpNextView: Could not find scroll container (.app-content)')
    // 尝试直接在 window 上绑定（作为后备，虽然通常不起作用）
    window.addEventListener('scroll', handleScroll, true)
  }
})

onBeforeUnmount(() => {
  const content = document.querySelector('.app-content')
  if (content) {
    content.removeEventListener('scroll', handleScroll)
  }
  window.removeEventListener('scroll', handleScroll, true)
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
  font-family: -apple-system, BlinkMacSystemFont, "SF Pro Display", "Helvetica Neue", sans-serif;
  font-size: 30px;
  font-weight: 700;
  margin: 0 0 8px 0;
  color: var(--glass-text, #1d1d1f);
  letter-spacing: -0.01em;
}

.page-subtitle {
  font-family: -apple-system, BlinkMacSystemFont, "SF Pro Text", "Helvetica Neue", sans-serif;
  font-size: 16px;
  color: var(--glass-text-secondary, #86909c);
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
  background: var(--glass-bg, rgba(255, 255, 255, 0.5));
  backdrop-filter: var(--glass-blur, blur(20px));
  border-radius: 12px;
  padding: 16px;
  gap: 20px;
  cursor: pointer;
  transition: all 0.25s cubic-bezier(0.2, 0, 0, 1);
  box-shadow: none; /* Flat by default */
  border: 1px solid var(--glass-border, rgba(255, 255, 255, 0.2)); /* Subtle border definition */
  position: relative; /* For z-index context */
}

.up-next-item:hover {
  transform: scale(1.015); /* Subtle scale */
  box-shadow: 0 8px 20px rgba(0, 0, 0, 0.08); /* Soft shadow on hover */
  background: var(--glass-overlay-bg, rgba(255, 255, 255, 0.7));
  border-color: var(--glass-border, rgba(255, 255, 255, 0.4));
  z-index: 10;
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
  background: var(--glass-overlay-bg, #f2f3f5);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--glass-text-secondary, #c9ccd4);
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
  font-family: -apple-system, BlinkMacSystemFont, "SF Pro Text", "Helvetica Neue", sans-serif;
  font-size: 17px;
  font-weight: 600;
  color: var(--glass-text, #1d1d1f);
  margin: 0;
  line-height: 1.3;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  margin-right: 12px;
}

.last-watched {
  font-family: -apple-system, BlinkMacSystemFont, "SF Pro Text", "Helvetica Neue", sans-serif;
  font-size: 12px;
  color: var(--glass-text-secondary, #86909c);
  white-space: nowrap;
  padding-top: 2px;
}

.episode-info {
  margin-bottom: 16px;
  font-family: -apple-system, BlinkMacSystemFont, "SF Pro Text", "Helvetica Neue", sans-serif;
  font-size: 14px;
  color: var(--glass-text-secondary, #4e5969);
  display: flex;
  align-items: center;
}

.season-ep {
  font-family: "SF Mono", SFMono-Regular, ui-monospace, monospace;
  font-weight: 600;
  color: #0066cc; /* macOS blue */
  margin-right: 8px;
  background: rgba(0, 102, 204, 0.08);
  padding: 2px 6px;
  border-radius: 6px;
  font-size: 12px;
  letter-spacing: -0.02em;
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

.loading-more, .no-more {
  text-align: center;
  padding: 20px;
  color: #86909c;
  font-size: 14px;
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