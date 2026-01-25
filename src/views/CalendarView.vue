<template>
  <div class="calendar-view">
    <div class="page-container">
      <div class="page-header">
        <h1 class="page-title">剧集日历</h1>
        <p class="page-subtitle">即将播出的剧集</p>
      </div>

      <div class="toolbar">
        <a-space :size="16">
          <a-range-picker
            v-model="dateRange"
            :style="{ width: '280px' }"
            @change="handleDateRangeChange"
          />
          <span class="item-count" v-if="calendarShows.length > 0">
            {{ calendarShows.length }} 个剧集
          </span>
        </a-space>
      </div>

      <div v-if="loading" class="loading-container">
        <a-spin :size="40" />
      </div>

      <div v-else-if="timelineGroups.length > 0" class="timeline-container">
        <div 
          v-for="group in timelineGroups" 
          :key="group.date" 
          class="timeline-group"
        >
          <div class="timeline-header">
            <div class="timeline-dot"></div>
            <h3 class="timeline-label">{{ group.label }}</h3>
            <span class="timeline-date-raw">{{ group.date }}</span>
          </div>
          <div class="timeline-content">
            <div class="episode-grid">
              <a-card
                v-for="item in group.items"
                :key="`${item.show.ids?.trakt ?? 0}-${item.episode?.ids.trakt ?? 0}`"
                class="episode-card"
                hoverable
                :bordered="false"
                @click="navigateToEpisode(item)"
              >
                <div class="episode-card-body">
                  <div class="poster-wrapper">
                    <img 
                      v-if="item.show.images?.poster?.[0]"
                      :src="getPosterUrl(item)" 
                      :alt="getShowTitle(item)"
                      class="show-poster"
                    />
                    <div v-else class="poster-placeholder"></div>
                  </div>
                  <div class="episode-info">
                    <div class="show-title">{{ getShowTitle(item) }}</div>
                    <div class="episode-title">
                      <a-tag color="arcoblue" size="small" class="episode-tag">
                        S{{ item.episode?.season }}E{{ item.episode?.number }}
                      </a-tag>
                      <span class="episode-name">{{ item.episode?.title }}</span>
                    </div>
                    <div class="air-time" v-if="item.first_aired">
                      {{ formatTime(item.first_aired) }}
                    </div>
                  </div>
                </div>
              </a-card>
            </div>
          </div>
        </div>
      </div>

      <div v-else class="empty-state">
        <a-empty description="这段时间没有剧集播出" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { Message } from '@arco-design/web-vue'
import type { CalendarShow } from '../types/api'
import { getShowChineseTranslation, type TranslationResult } from '../utils/translation'
import { useUserDataUpdate } from '../composables/useEvent'

defineOptions({ name: 'CalendarView' })

const router = useRouter()
const loading = ref(false)
const calendarShows = ref<CalendarShow[]>([])
const showTranslations = ref<Map<number, TranslationResult>>(new Map())

const today = new Date()
const nextWeek = new Date()
nextWeek.setDate(today.getDate() + 7)

const dateRange = ref<[string, string]>([
  today.toISOString().split('T')[0],
  nextWeek.toISOString().split('T')[0]
])

// 监听日历数据更新
useUserDataUpdate((payload) => {
  if (!dateRange.value || !dateRange.value[0] || !dateRange.value[1]) return
  
  const start = new Date(dateRange.value[0])
  const end = new Date(dateRange.value[1])
  const diffTime = Math.abs(end.getTime() - start.getTime())
  const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24)) + 1
  
  // Key format: calendar_my_shows_{start_date}_{days}
  const expectedKey = `calendar_my_shows_${start.toISOString().split('T')[0]}_${diffDays}`
  
  if (payload.key === expectedKey) {
    console.log('收到日历数据更新')
    const newData = payload.data as CalendarShow[]
    calendarShows.value = newData
    loadTranslations(newData)
    
    Message.info({
      content: '日历已自动刷新',
      position: 'bottom',
      duration: 2000
    })
  }
})

const fetchCalendar = async () => {
  if (!dateRange.value || !dateRange.value[0] || !dateRange.value[1]) return

  loading.value = true
  try {
    const start = new Date(dateRange.value[0])
    const end = new Date(dateRange.value[1])
    const diffTime = Math.abs(end.getTime() - start.getTime())
    const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24)) + 1

    // SWR
    const result = await invoke<CalendarShow[]>('get_my_calendar_shows', {
      startDate: start.toISOString().split('T')[0],
      days: diffDays
    })
    calendarShows.value = result
    loadTranslations(result)
  } catch (error) {
    console.error('Failed to fetch calendar:', error)
  } finally {
    loading.value = false
  }
}

const loadTranslations = async (shows: CalendarShow[]) => {
  const uniqueShowIds = [...new Set(shows.map(s => s.show.ids?.trakt).filter(Boolean))] as number[]
  for (const showId of uniqueShowIds) {
    if (!showTranslations.value.has(showId)) {
      const translation = await getShowChineseTranslation(showId)
      if (translation) {
        showTranslations.value.set(showId, translation)
      }
    }
  }
}

const getShowTitle = (item: CalendarShow) => {
  const showId = item.show.ids?.trakt
  if (showId && showTranslations.value.has(showId)) {
    return showTranslations.value.get(showId)?.title || item.show.title
  }
  return item.show.title
}

const getPosterUrl = (item: CalendarShow) => {
  const poster = item.show.images?.poster?.[0]
  if (!poster) return ''
  return poster.startsWith('http') ? poster : `https://${poster}`
}

const handleDateRangeChange = () => {
  fetchCalendar()
}

const timelineGroups = computed(() => {
  const groups: Record<string, CalendarShow[]> = {}
  
  calendarShows.value.forEach(item => {
    if (!item.first_aired) return
    const date = item.first_aired.split('T')[0]
    if (!groups[date]) {
      groups[date] = []
    }
    groups[date].push(item)
  })

  const sortedDates = Object.keys(groups).sort()
  const todayStr = new Date().toISOString().split('T')[0]
  const tomorrowDate = new Date()
  tomorrowDate.setDate(tomorrowDate.getDate() + 1)
  const tomorrowStr = tomorrowDate.toISOString().split('T')[0]

  return sortedDates.map(date => {
    let label = ''
    if (date === todayStr) {
      label = '今天'
    } else if (date === tomorrowStr) {
      label = '明天'
    } else {
      const d = new Date(date)
      label = `${d.getMonth() + 1}月${d.getDate()}日`
    }

    return {
      date,
      label,
      items: groups[date].sort((a, b) => {
        if (!a.first_aired || !b.first_aired) return 0
        return a.first_aired.localeCompare(b.first_aired)
      })
    }
  })
})

const formatTime = (isoString: string) => {
  const date = new Date(isoString)
  return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
}

const navigateToEpisode = (item: CalendarShow) => {
  if (item.show.ids?.trakt && item.episode) {
    router.push({
      path: `/show/${item.show.ids.trakt}/season/${item.episode.season}/episode/${item.episode.number}`,
      state: {
        posterUrl: getPosterUrl(item),
        // 尝试获取 fanart (backdrop)，如果 CalendarShow 结构里有的话
        backdropUrl: item.show.images?.fanart?.[0] ? 
          (item.show.images.fanart[0].startsWith('http') ? item.show.images.fanart[0] : `https://${item.show.images.fanart[0]}`) 
          : undefined,
        showTitle: getShowTitle(item) // 把剧名也传过去，防止详情页不知道是哪个剧
      }
    })
  }
}

onMounted(() => {
  fetchCalendar()
})
</script>

<style scoped>
.calendar-view {
  min-height: 100vh;
}

.page-header {
  margin-bottom: 32px;
}

.page-title {
  font-size: 32px;
  font-weight: 700;
  margin: 0 0 8px 0;
  color: #1d1d1f;
  letter-spacing: -0.5px;
}

.page-subtitle {
  font-size: 16px;
  color: #86868b;
  margin: 0;
}

.toolbar {
  margin-bottom: 32px;
  padding: 16px 20px;
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border-radius: 16px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.04);
  border: 1px solid rgba(0,0,0,0.05);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.item-count {
  font-size: 14px;
  color: #86868b;
  font-weight: 500;
}

.loading-container {
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 100px 0;
}

/* Timeline Styling */
.timeline-container {
  position: relative;
  padding-left: 32px;
}

.timeline-container::before {
  content: '';
  position: absolute;
  left: 7px;
  top: 12px;
  bottom: 0;
  width: 2px;
  background: linear-gradient(180deg, #165dff 0%, rgba(22, 93, 255, 0.1) 100%);
  border-radius: 1px;
}

.timeline-group {
  position: relative;
  margin-bottom: 48px;
}

.timeline-header {
  display: flex;
  align-items: baseline;
  gap: 12px;
  margin-bottom: 20px;
  position: relative;
}

.timeline-dot {
  position: absolute;
  left: -32px;
  top: 6px;
  width: 16px;
  height: 16px;
  background: #165dff;
  border-radius: 50%;
  border: 3px solid #fff;
  box-shadow: 0 0 0 1px rgba(22, 93, 255, 0.1), 0 4px 12px rgba(22, 93, 255, 0.2);
  z-index: 1;
}

.timeline-label {
  font-size: 24px;
  font-weight: 700;
  color: #1d1d1f;
  margin: 0;
  letter-spacing: -0.5px;
}

.timeline-date-raw {
  font-size: 14px;
  color: #86868b;
  font-weight: 500;
}

.episode-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: 20px;
}

.episode-card {
  border-radius: 16px;
  border: none;
  background: #ffffff;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.04);
  transition: all 0.4s cubic-bezier(0.25, 0.46, 0.45, 0.94);
  overflow: hidden;
}

.episode-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 16px 32px rgba(0, 0, 0, 0.08);
}

:deep(.arco-card-body) {
  padding: 16px;
}

.episode-card-body {
  display: flex;
  gap: 16px;
}

.poster-wrapper {
  width: 80px;
  height: 120px;
  flex-shrink: 0;
  border-radius: 12px;
  overflow: hidden;
  background: #f5f5f7;
  box-shadow: 0 2px 8px rgba(0,0,0,0.06);
}

.show-poster {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: transform 0.5s ease;
}

.poster-placeholder {
  width: 100%;
  height: 100%;
  background: #e5e5e7;
}

.episode-card:hover .show-poster {
  transform: scale(1.05);
}

.episode-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: center;
  min-width: 0;
}

.show-title {
  font-size: 16px;
  font-weight: 700;
  color: #1d1d1f;
  margin-bottom: 4px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.episode-title {
  font-size: 14px;
  color: #424245;
  margin-bottom: 8px;
  display: flex;
  align-items: center;
  gap: 8px;
  line-height: 1.4;
}

.episode-name {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.episode-tag {
  font-weight: 600;
  flex-shrink: 0;
}

.air-time {
  font-size: 13px;
  color: #86868b;
  font-weight: 500;
  display: flex;
  align-items: center;
}

.air-time::before {
  content: '•';
  margin-right: 6px;
  color: #165dff;
}

.empty-state {
  padding: 80px 0;
  text-align: center;
}

@media (max-width: 640px) {
  .episode-grid {
    grid-template-columns: 1fr;
  }
}
</style>
