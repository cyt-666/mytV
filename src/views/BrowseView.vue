<template>
  <div class="browse-view page-container">
    <div class="page-header">
      <h1 class="page-title">发现内容</h1>
      
      <a-radio-group 
        v-model="activeTab" 
        type="button" 
        size="large"
        class="custom-segmented-control"
        @change="handleTabChange"
      >
        <a-radio value="genres">
           <template #default>
             <IconApps style="margin-right: 6px"/> 按类型
           </template>
        </a-radio>
        <a-radio value="countries">
           <template #default>
             <IconPublic style="margin-right: 6px"/> 按地区
           </template>
        </a-radio>
      </a-radio-group>
    </div>

    <!-- 按类型浏览 -->
    <div v-show="activeTab === 'genres'" class="view-section">
      <div class="filter-toolbar">
        <a-space size="medium" wrap>
          <a-radio-group v-model="genreMediaType" type="button" @change="resetAndLoad('genres')">
            <a-radio value="movies">电影</a-radio>
            <a-radio value="shows">剧集</a-radio>
          </a-radio-group>
          
          <a-select 
            v-model="genreDataSource" 
            placeholder="数据源" 
            style="width: 160px;"
            @change="resetAndLoad('genres')"
          >
            <a-option value="watched-weekly">本周观看榜</a-option>
            <a-option value="watched-monthly">本月观看榜</a-option>
            <a-option value="collected-monthly">本月收藏榜</a-option>
            <a-option value="popular">历史热门</a-option>
            <a-option value="trending">实时趋势</a-option>
          </a-select>
          
          <a-select 
            v-model="selectedGenre" 
            placeholder="选择类型" 
            style="width: 180px;"
            @change="resetAndLoad('genres')"
          >
            <a-option value="all">全部类型</a-option>
            <a-option v-for="(name, slug) in genreMap" :key="slug" :value="slug">
              {{ name }}
            </a-option>
          </a-select>

          <a-tag v-if="genreItems.length > 0" color="arcoblue" class="result-tag">
            <template #icon><IconCheckCircle /></template>
            {{ genreItems.length }} 个结果
          </a-tag>
        </a-space>
      </div>

      <MediaGrid
        :items="genreItems"
        :loading="loading && genreItems.length === 0"
        :loading-more="loadingMore"
        :has-more="hasMoreGenre"
        @load-more="loadMoreGenre"
        :media-type="genreMediaType === 'movies' ? 'movie' : 'show'"
        :empty-message="getEmptyMessage('genre')"
      />
    </div>

    <!-- 按地区浏览 -->
    <div v-show="activeTab === 'countries'" class="view-section">
      <div class="filter-toolbar">
        <a-space size="medium" wrap>
          <a-radio-group v-model="countryMediaType" type="button" @change="resetAndLoad('countries')">
            <a-radio value="movies">电影</a-radio>
            <a-radio value="shows">剧集</a-radio>
          </a-radio-group>
          
          <a-select 
            v-model="countryDataSource" 
            placeholder="数据源" 
            style="width: 160px;"
            @change="resetAndLoad('countries')"
          >
            <a-option value="watched-weekly">本周观看榜</a-option>
            <a-option value="watched-monthly">本月观看榜</a-option>
            <a-option value="collected-monthly">本月收藏榜</a-option>
            <a-option value="popular">历史热门</a-option>
            <a-option value="trending">实时趋势</a-option>
          </a-select>
          
          <a-select 
            v-model="selectedCountry" 
            placeholder="选择地区" 
            style="width: 180px;"
            @change="resetAndLoad('countries')"
          >
            <a-option value="all">全部地区</a-option>
            <a-option v-for="(name, code) in countryMap" :key="code" :value="code">
              {{ name }}
            </a-option>
          </a-select>

          <a-tag v-if="countryItems.length > 0" color="arcoblue" class="result-tag">
            <template #icon><IconCheckCircle /></template>
            {{ countryItems.length }} 个结果
          </a-tag>
        </a-space>
      </div>

      <MediaGrid
        :items="countryItems"
        :loading="loading && countryItems.length === 0"
        :loading-more="loadingMore"
        :has-more="hasMoreCountry"
        @load-more="loadMoreCountry"
        :media-type="countryMediaType === 'movies' ? 'movie' : 'show'"
        :empty-message="getEmptyMessage('country')"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { IconCheckCircle, IconApps, IconPublic } from '@arco-design/web-vue/es/icon'
import MediaGrid from '../components/MediaGrid.vue'
import { invoke } from "@tauri-apps/api/core"
import { preloadMovieTranslations } from '../utils/translation'
import type { Movie, Show } from '../types/api'

defineOptions({ name: 'BrowseView' })

const route = useRoute()
const router = useRouter()

const genreMap: Record<string, string> = {
  'action': '动作', 'adventure': '冒险', 'animation': '动画',
  'comedy': '喜剧', 'crime': '犯罪', 'documentary': '纪录片',
  'drama': '剧情', 'family': '家庭', 'fantasy': '奇幻',
  'history': '历史', 'horror': '恐怖', 'music': '音乐',
  'mystery': '悬疑', 'romance': '爱情', 'science-fiction': '科幻',
  'thriller': '惊悚', 'war': '战争'
}

const countryMap: Record<string, string> = {
  'us': '🇺🇸 美国', 'gb': '🇬🇧 英国', 'jp': '🇯🇵 日本', 'kr': '🇰🇷 韩国',
  'cn': '🇨🇳 中国', 'fr': '🇫🇷 法国', 'de': '🇩🇪 德国', 'ca': '🇨🇦 加拿大',
  'au': '🇦🇺 澳大利亚', 'es': '🇪🇸 西班牙', 'it': '🇮🇹 意大利', 'in': '🇮🇳 印度',
  'hk': '🇭🇰 香港', 'tw': '🇹🇼 台湾'
}

const activeTab = ref('genres')
const loading = ref(false)
const loadingMore = ref(false)

const genreMediaType = ref('movies')
const genreDataSource = ref('watched-weekly')
const selectedGenre = ref('all')
const genreItems = ref<(Movie | Show)[]>([])
const hasMoreGenre = ref(true)
const genrePage = ref(1)

const countryMediaType = ref('movies')
const countryDataSource = ref('watched-weekly')
const selectedCountry = ref('all')
const countryItems = ref<(Movie | Show)[]>([])
const hasMoreCountry = ref(true)
const countryPage = ref(1)

const loadData = async (isLoadMore: boolean) => {
  const isGenre = activeTab.value === 'genres'
  const mediaType = isGenre ? genreMediaType.value : countryMediaType.value
  const dataSource = isGenre ? genreDataSource.value : countryDataSource.value
  
  const genres = isGenre && selectedGenre.value !== 'all' ? selectedGenre.value : null
  const countries = !isGenre && selectedCountry.value !== 'all' ? selectedCountry.value : null
  
  const itemsRef = isGenre ? genreItems : countryItems
  const pageRef = isGenre ? genrePage : countryPage
  const hasMoreRef = isGenre ? hasMoreGenre : hasMoreCountry
  
  if (isLoadMore) {
    if (!hasMoreRef.value) return
    loadingMore.value = true
  } else {
    loading.value = true
    itemsRef.value = []
    pageRef.value = 1
    hasMoreRef.value = true
  }

  try {
    const limit = 40
    let newItems: (Movie | Show)[] = []
    
    let command = ''
    let args: any = { 
      page: pageRef.value, 
      limit, 
      genres, 
      countries 
    }

    if (dataSource === 'popular') {
      command = mediaType === 'movies' ? 'movie_popular_page' : 'show_popular_page'
    } else if (dataSource === 'trending') {
      command = mediaType === 'movies' ? 'movie_trending_page' : 'show_trending_page'
    } else {
      const [source, period] = dataSource.split('-')
      args.period = period
      if (source === 'watched') {
        command = mediaType === 'movies' ? 'movie_watched_period' : 'show_watched_period'
      } else {
        command = mediaType === 'movies' ? 'movie_collected_period' : 'show_collected_period'
      }
    }

    const res = await invoke<any>(command, args)
    
    if (dataSource === 'popular') {
      newItems = res
    } else if (dataSource === 'trending') {
      newItems = res.map((item: any) => {
        const media = mediaType === 'movies' ? item.movie : item.show
        return { ...media, watchers: item.watchers }
      })
    } else if (dataSource.startsWith('watched')) {
      newItems = res.map((item: any) => {
        const media = mediaType === 'movies' ? item.movie : item.show
        return { ...media, watcher_count: item.watcher_count }
      })
    } else { 
      newItems = res.map((item: any) => {
        const media = mediaType === 'movies' ? item.movie : item.show
        return { ...media, collected_count: item.collected_count }
      })
    }

    if (newItems.length < limit) {
      hasMoreRef.value = false
    }

    const existingIds = new Set(itemsRef.value.map(i => i.ids?.trakt))
    const uniqueItems = newItems.filter(i => i.ids?.trakt && !existingIds.has(i.ids.trakt)) as (Movie | Show)[]
    itemsRef.value.push(...uniqueItems)
    
    pageRef.value++

    if (mediaType === 'movies' && uniqueItems.length > 0) {
      preloadMovieTranslations(uniqueItems as Movie[], () => {})
    }

  } catch (error) {
    console.error('Load data failed:', error)
  } finally {
    loading.value = false
    loadingMore.value = false
  }
}

const resetAndLoad = (tab: string) => {
  if (tab !== activeTab.value) return
  loadData(false)
}

const handleTabChange = (key: string | number | boolean) => {
  const k = String(key)
  activeTab.value = k
  router.replace({ query: { ...route.query, tab: k } })
  
  if (k === 'genres' && genreItems.value.length === 0) {
    loadData(false)
  } else if (k === 'countries' && countryItems.value.length === 0) {
    loadData(false)
  }
}

const loadMoreGenre = () => loadData(true)
const loadMoreCountry = () => loadData(true)

// 恢复参数以匹配模板调用，使用 _ 前缀忽略未使用检查
const getEmptyMessage = (_type: string) => {
  return '暂无数据，请尝试切换数据源或筛选条件'
}

onMounted(() => {
  const tab = route.query.tab as string
  if (tab && ['genres', 'countries'].includes(tab)) {
    activeTab.value = tab
  }
  loadData(false)
})
</script>

<style scoped>
.page-header { 
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}
.page-title { font-size: 32px; font-weight: 800; color: #1d1d1f; margin: 0; }

/* Segmented Control Styles */
.custom-segmented-control {
  background-color: rgba(118, 118, 128, 0.12);
  padding: 4px;
  border-radius: 9px;
  border: none;
  display: inline-flex;
}

.custom-segmented-control :deep(.arco-radio-button) {
  background: transparent;
  border: none;
  border-radius: 7px;
  color: #555;
  font-weight: 500;
  padding: 0 16px;
  height: 32px;
  line-height: 32px;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  display: flex;
  align-items: center;
}

.custom-segmented-control :deep(.arco-radio-button.arco-radio-checked) {
  background: #ffffff;
  box-shadow: 0 3px 8px rgba(0, 0, 0, 0.12), 0 1px 1px rgba(0, 0, 0, 0.04);
  color: #000;
  font-weight: 600;
}

.custom-segmented-control :deep(.arco-radio-button:hover:not(.arco-radio-checked)) {
  background: rgba(255, 255, 255, 0.5);
}

/* Filter Toolbar - Glassmorphism */
.filter-toolbar {
  margin: 0 0 32px 0;
  padding: 20px 24px;
  background: rgba(255, 255, 255, 0.65);
  backdrop-filter: blur(20px) saturate(180%);
  -webkit-backdrop-filter: blur(20px) saturate(180%);
  border-radius: 16px;
  border: 1px solid rgba(255, 255, 255, 0.5);
  box-shadow: 0 4px 24px rgba(0, 0, 0, 0.02);
  display: flex;
  align-items: center;
}

/* Rounded inputs inside toolbar */
.filter-toolbar :deep(.arco-select-view),
.filter-toolbar :deep(.arco-input-wrapper) {
  border-radius: 8px;
  background-color: rgba(255, 255, 255, 0.8);
  border: 1px solid rgba(0,0,0,0.05);
  box-shadow: 0 2px 4px rgba(0,0,0,0.01);
  transition: all 0.2s ease;
}
.filter-toolbar :deep(.arco-select-view:hover),
.filter-toolbar :deep(.arco-input-wrapper:hover) {
  background-color: #fff;
  border-color: rgba(0,0,0,0.1);
}

/* Radio buttons inside toolbar */
.filter-toolbar :deep(.arco-radio-group-button) {
  background-color: rgba(118, 118, 128, 0.12);
  padding: 2px;
  border-radius: 8px;
  border: none;
}
.filter-toolbar :deep(.arco-radio-button) {
  background: transparent;
  border: none;
  border-radius: 6px;
  margin: 0;
  color: #666;
}
.filter-toolbar :deep(.arco-radio-button.arco-radio-checked) {
  background: #fff;
  color: #1d1d1f;
  box-shadow: 0 2px 5px rgba(0,0,0,0.1);
  font-weight: 500;
}

.result-tag {
  border-radius: 6px;
  font-weight: 500;
}

@media (max-width: 768px) {
  .page-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 16px;
  }
  .page-title { font-size: 24px; }
  .filter-toolbar { padding: 16px; }
}
</style>
