<template>
  <div class="search-view">
    <div class="page-container">
      <!-- 搜索区域 -->
      <section class="search-section">
        <div class="search-header">
          <h1 class="search-title">搜索影视作品</h1>
          <p class="search-subtitle">发现你想要观看的电影和电视剧</p>
        </div>
        
        <div class="search-form">
          <a-input-search
            v-model="searchQuery"
            class="search-input"
            placeholder="输入电影或电视剧名称..."
            size="large"
            @search="handleSearch"
            @press-enter="handleSearch"
            :loading="searching"
          >
            <template #prefix>
              <icon-search />
            </template>
            <template #suffix>
              <a-button 
                v-if="searchQuery"
                type="text" 
                size="mini"
                @click="clearSearch"
              >
                <icon-close />
              </a-button>
            </template>
          </a-input-search>
        </div>

        <!-- 搜索筛选 -->
        <div class="search-filters" v-if="searchQuery">
          <a-space :size="16">
            <a-select
              v-model="filters.type"
              placeholder="类型"
              :style="{ width: '120px' }"
              @change="handleFilterChange"
            >
              <a-option value="">全部</a-option>
              <a-option value="movie">电影</a-option>
              <a-option value="show">电视剧</a-option>
            </a-select>
            
            <a-select
              v-model="filters.year"
              placeholder="年份"
              :style="{ width: '120px' }"
              @change="handleFilterChange"
            >
              <a-option value="">全部年份</a-option>
              <a-option 
                v-for="year in yearOptions" 
                :key="year" 
                :value="year"
              >
                {{ year }}
              </a-option>
            </a-select>
            
            <a-select
              v-model="filters.genre"
              placeholder="类型"
              :style="{ width: '140px' }"
              @change="handleFilterChange"
            >
              <a-option value="">全部类型</a-option>
              <a-option 
                v-for="genre in genreOptions" 
                :key="genre" 
                :value="genre"
              >
                {{ genre }}
              </a-option>
            </a-select>
            
            <a-button 
              v-if="hasActiveFilters"
              type="outline"
              @click="clearFilters"
            >
              <icon-refresh />
              清除筛选
            </a-button>
          </a-space>
        </div>
      </section>

      <!-- 搜索结果 -->
      <section class="results-section" v-if="searchQuery">
        <!-- 结果统计 -->
        <div class="results-header" v-if="!searching">
          <h2 class="results-title">
            "{{ searchQuery }}" 的搜索结果
            <span v-if="searchResults.length > 0" class="results-count">
              ({{ searchResults.length }} 个结果)
            </span>
          </h2>
        </div>

        <!-- 搜索结果网格 -->
        <MediaGrid
          :items="filteredResults"
          :loading="searching"
          :empty-message="getEmptyMessage()"
          :show-meta="true"
          @load-more="loadMoreResults"
        />
      </section>

      <!-- 搜索历史与推荐 -->
      <section class="trending-section" v-else>
        <div class="history-section" v-if="searchHistory.length > 0">
          <div class="section-header">
            <h2 class="section-title">
              <icon-history /> 搜索记录
            </h2>
            <a-button type="text" size="small" @click="clearHistory" class="clear-btn">
              <icon-delete /> 清空
            </a-button>
          </div>
          <div class="history-tags">
            <a-tag
              v-for="tag in searchHistory"
              :key="tag"
              class="history-tag"
              closable
              @click="searchTrending(tag)"
              @close="removeHistoryItem(tag)"
            >
              {{ tag }}
            </a-tag>
          </div>
        </div>

        <h2 class="section-title">
          <icon-trophy /> 热门期待榜
        </h2>
        <div class="top-search-list">
          <div v-if="loadingTopSearch" class="loading-state">
            <a-spin /> 加载榜单中...
          </div>
          <div 
            v-else
            v-for="(item, index) in topSearchItems" 
            :key="item.ids?.trakt"
            class="top-search-item"
            @click="searchTrending(item.title)"
          >
            <div class="rank-number" :class="{'top-3': index < 3}">{{ index + 1 }}</div>
            <div class="item-info">
              <div class="item-title">{{ item.title }}</div>
              <div class="item-meta">
                <span v-if="item.year">{{ item.year }}</span>
                <span class="dot" v-if="item.year && item.rating">•</span>
                <span v-if="item.rating" class="rating">
                  <icon-star-fill /> {{ item.rating.toFixed(1) }}
                </span>
              </div>
            </div>
            <div class="trend-icon">
              <icon-fire style="color: #f53f3f" v-if="index < 3" />
            </div>
          </div>
        </div>
      </section>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, nextTick, onBeforeUnmount } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { Message } from '@arco-design/web-vue'
import { IconSearch, IconClose, IconRefresh, IconHistory, IconDelete, IconFire, IconTrophy, IconStarFill } from '@arco-design/web-vue/es/icon'
import MediaGrid from '../components/MediaGrid.vue'
import type { Movie, Show, MovieAnticipated, ShowAnticipated } from '../types/api'
import { usePageState } from '../composables/usePageState'
import { getMovieChineseTranslation, getShowChineseTranslation } from '../utils/translation'

interface SearchResultItem {
  type: string
  score: number
  movie?: Movie
  show?: Show
}

const route = useRoute()
const router = useRouter()

// 使用状态管理
const { saveState, restoreState } = usePageState('search')

// 定义组件名称用于keep-alive
defineOptions({
  name: 'SearchView'
})

// 响应式数据
const searchQuery = ref('')
const searching = ref(false)
const searchResults = ref<(Movie | Show)[]>([])
const topSearchItems = ref<(Movie | Show)[]>([])
const loadingTopSearch = ref(false)
const searchHistory = ref<string[]>([])

const filters = ref({
  type: '',
  year: '',
  genre: ''
})

// 历史记录管理
const loadSearchHistory = () => {
  try {
    const history = localStorage.getItem('search_history')
    if (history) {
      searchHistory.value = JSON.parse(history)
    }
  } catch (e) {
    console.error('加载搜索记录失败', e)
  }
}

const saveToHistory = (query: string) => {
  if (!query.trim()) return
  const history = searchHistory.value.filter(item => item !== query)
  history.unshift(query)
  if (history.length > 20) history.pop()
  searchHistory.value = history
  localStorage.setItem('search_history', JSON.stringify(history))
}

const removeHistoryItem = (item: string) => {
  searchHistory.value = searchHistory.value.filter(i => i !== item)
  localStorage.setItem('search_history', JSON.stringify(searchHistory.value))
}

const clearHistory = () => {
  searchHistory.value = []
  localStorage.removeItem('search_history')
  Message.success('搜索记录已清空')
}

// 选项数据
const yearOptions = computed(() => {
  const currentYear = new Date().getFullYear()
  const years = []
  for (let year = currentYear; year >= 1950; year--) {
    years.push(year)
  }
  return years
})

const genreOptions = ref([
  '动作', '冒险', '喜剧', '犯罪', '剧情', '科幻',
  '惊悚', '恐怖', '爱情', '动画', '纪录片', '悬疑'
])

// 计算属性
const hasActiveFilters = computed(() => {
  return Object.values(filters.value).some(value => value !== '')
})

const filteredResults = computed(() => {
  let results = searchResults.value

  if (filters.value.type) {
    results = results.filter(item => {
      // 优先使用显式的 media_type
      if (item.media_type) {
        return item.media_type === filters.value.type
      }
      // 后备判断逻辑
      const isMovie = 'tagline' in item || 'released' in item
      return filters.value.type === 'movie' ? isMovie : !isMovie
    })
  }

  if (filters.value.year) {
    results = results.filter(item => 
      item.year === Number(filters.value.year)
    )
  }

  if (filters.value.genre) {
    results = results.filter(item =>
      item.genres?.includes(filters.value.genre)
    )
  }

  return results
})

// 保存搜索状态
const saveSearchState = () => {
  const state = {
    searchQuery: searchQuery.value,
    filters: { ...filters.value },
    searchResults: searchResults.value,
    scrollPosition: window.scrollY,
    timestamp: Date.now()
  }
  saveState(state)
}

// 恢复搜索状态
const restoreSearchState = () => {
  const state = restoreState()
  if (state && state.timestamp) {
    // 2分钟内的状态才恢复
    const twoMinutes = 2 * 60 * 1000
    if (Date.now() - state.timestamp < twoMinutes) {
      searchQuery.value = state.searchQuery || ''
      filters.value = state.filters || { type: '', year: '', genre: '' }
      searchResults.value = state.searchResults || []
      
      // 恢复滚动位置
      if (state.scrollPosition > 0) {
        nextTick(() => {
          window.scrollTo({ top: state.scrollPosition, behavior: 'smooth' })
        })
      }
      
      return true
    }
  }
  return false
}

// 方法
const handleSearch = () => {
  if (!searchQuery.value.trim()) return
  
  router.push({
    name: 'search',
    query: { q: searchQuery.value }
  })
  
  performSearch()
  saveSearchState()
}

const performSearch = async () => {
  if (!searchQuery.value.trim()) return

  saveToHistory(searchQuery.value.trim())
  searching.value = true
  try {
    const results = await invoke<SearchResultItem[]>('search_media', { 
      query: searchQuery.value.trim() 
    })
    
    const items: (Movie | Show)[] = []
    for (const result of results) {
      if (result.movie) {
        result.movie.media_type = 'movie'
        items.push(result.movie)
      } else if (result.show) {
        result.show.media_type = 'show'
        items.push(result.show)
      }
    }
    
    searchResults.value = items
  } catch (error) {
    console.error('搜索失败:', error)
    Message.error('搜索失败，请稍后重试')
    searchResults.value = []
  } finally {
    searching.value = false
  }
}

const clearSearch = () => {
  searchQuery.value = ''
  searchResults.value = []
  clearFilters()
  router.push({ name: 'search' })
  saveSearchState()
}

const handleFilterChange = () => {
  // 筛选器变化时保存状态
  saveSearchState()
}

const clearFilters = () => {
  filters.value = {
    type: '',
    year: '',
    genre: ''
  }
  saveSearchState()
}

const searchTrending = (tag: string) => {
  searchQuery.value = tag
  handleSearch()
}

const loadMoreResults = () => {
  console.log('加载更多搜索结果')
  // 实现分页加载
}

const getEmptyMessage = () => {
  if (hasActiveFilters.value) {
    return '没有找到符合筛选条件的结果，试试调整筛选条件'
  }
  return `没有找到 "${searchQuery.value}" 相关的结果`
}

const loadTopSearchData = async () => {
  loadingTopSearch.value = true
  try {
    // 并行获取最受期待电影和电视剧 (Anticipated)
    const [anticipatedMovies, anticipatedShows] = await Promise.all([
      invoke<MovieAnticipated[]>('movie_anticipated', { page: 1, limit: 10 }),
      invoke<ShowAnticipated[]>('show_anticipated', { page: 1, limit: 10 })
    ])

    const items: (Movie | Show)[] = []
    
    if (anticipatedMovies) {
      anticipatedMovies.forEach(item => {
        if (item.movie) {
          item.movie.media_type = 'movie'
          items.push(item.movie)
        }
      })
    }
    
    if (anticipatedShows) {
      anticipatedShows.forEach(item => {
        if (item.show) {
          item.show.media_type = 'show'
          items.push(item.show)
        }
      })
    }
    
    // 随机打乱顺序并取前10个作为 Top Search
    topSearchItems.value = items.sort(() => Math.random() - 0.5).slice(0, 10)
    
    // 尝试加载中文标题
    const fetchPromises: Promise<void>[] = []
    topSearchItems.value.forEach(item => {
      const id = item.ids?.trakt
      if (id) {
        const promise = (async () => {
          try {
            const trans = item.media_type === 'movie' 
              ? await getMovieChineseTranslation(id)
              : await getShowChineseTranslation(id)
            if (trans?.title) {
              item.title = trans.title
            }
          } catch (e) { /* ignore */ }
        })()
        fetchPromises.push(promise)
      }
    })
    await Promise.all(fetchPromises)
    
  } catch (error) {
    console.error('加载热门搜索内容失败:', error)
  } finally {
    loadingTopSearch.value = false
  }
}

// 监听路由变化
watch(
  () => route.query.q,
  (newQuery) => {
    if (newQuery && typeof newQuery === 'string') {
      searchQuery.value = newQuery
      performSearch()
    } else {
      searchQuery.value = ''
      searchResults.value = []
    }
  },
  { immediate: true }
)

// 生命周期
onMounted(() => {
  // 尝试恢复状态
  const restored = restoreSearchState()
  
  if (!restored) {
    // 没有恢复状态时，处理URL参数
    const query = route.query.q
    if (query && typeof query === 'string') {
      searchQuery.value = query
      performSearch()
    }
  }
  
  loadTopSearchData()
  loadSearchHistory()
})

// 页面卸载前保存状态
onBeforeUnmount(() => {
  if (searchQuery.value || searchResults.value.length > 0) {
    saveSearchState()
  }
})
</script>

<style scoped>
.search-view {
  min-height: 100vh;
}

.search-section {
  margin-bottom: 40px;
  text-align: center;
}

.search-header {
  margin-bottom: 32px;
}

.search-form {
  width: 100%;
  max-width: 600px;
  margin: 0 auto 24px;
}

.search-input {
  width: 100%;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
}

.search-header {
  text-align: center;
  margin-bottom: 32px;
  display: flex;
  flex-direction: column;
  align-items: center;
  width: 100%; /* 确保占满宽度 */
}

.search-form {
  width: 100%;
  max-width: 600px;
  margin-bottom: 24px;
}

.search-header {
  text-align: center;
  margin-bottom: 32px;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.search-title {
  font-size: 32px;
  font-weight: 700;
  margin: 0 0 8px 0;
  color: var(--glass-text);
}

.search-subtitle {
  font-size: 16px;
  color: var(--glass-text-secondary);
  margin: 0;
}

.search-form {
  max-width: 600px;
  margin: 0 auto 24px;
}

.search-input {
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
}

.search-filters {
  display: flex;
  justify-content: center;
  margin-top: 20px;
}

.results-section {
  margin-top: 40px;
}

.results-header {
  margin-bottom: 24px;
}

.results-title {
  font-size: 24px;
  font-weight: 600;
  margin: 0;
  color: var(--glass-text);
}

.results-count {
  font-size: 16px;
  color: var(--glass-text-secondary);
  font-weight: 400;
}

.history-section {
  margin-bottom: 48px; /* 增加底部间距 */
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.section-title {
  font-size: 20px;
  font-weight: 600;
  margin: 0 0 24px 0; /* 增加标题底部间距 20px -> 24px */
  color: var(--glass-text);
  display: flex;
  align-items: center;
  gap: 8px;
}

.clear-btn {
  color: var(--glass-text-secondary);
}

.clear-btn:hover {
  color: #f53f3f;
  background-color: transparent;
}

.history-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
}

.history-tag {
  cursor: pointer;
  padding: 6px 16px;
  height: 36px;
  line-height: 24px;
  font-size: 14px;
  background: var(--glass-bg);
  backdrop-filter: var(--glass-blur);
  -webkit-backdrop-filter: var(--glass-blur);
  border: 1px solid var(--glass-border);
  border-radius: 18px;
  color: var(--glass-text-secondary);
  transition: all 0.2s ease;
  box-shadow: var(--glass-shadow);
}

.history-tag:hover {
  background: var(--glass-overlay-bg);
  color: var(--glass-text);
}

.trending-section {
  margin-top: 48px; /* 增加顶部间距 */
}

.top-search-list {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 16px;
}

.top-search-item {
  display: flex;
  align-items: center;
  padding: 12px 16px;
  background: var(--glass-bg);
  backdrop-filter: var(--glass-blur);
  -webkit-backdrop-filter: var(--glass-blur);
  border: 1px solid var(--glass-border);
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.2s;
  box-shadow: var(--glass-shadow);
}

.top-search-item:hover {
  background: var(--glass-overlay-bg);
  transform: translateY(-2px);
  box-shadow: 0 8px 24px rgba(0,0,0,0.1);
}

.rank-number {
  font-size: 18px;
  font-weight: 700;
  width: 32px;
  color: var(--glass-text-secondary);
  font-style: italic;
}

.rank-number.top-3 {
  color: #f53f3f;
  font-size: 20px;
}

.item-info {
  flex: 1;
  overflow: hidden;
}

.item-title {
  font-size: 15px;
  font-weight: 600;
  color: var(--glass-text);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  margin-bottom: 2px;
}

.item-meta {
  font-size: 12px;
  color: var(--glass-text-secondary);
  display: flex;
  align-items: center;
  gap: 4px;
}

.rating {
  color: #ffb400;
  display: flex;
  align-items: center;
  gap: 2px;
}

.trend-icon {
  margin-left: 8px;
}

.loading-state {
  grid-column: span 2;
  text-align: center;
  padding: 40px;
  color: var(--glass-text-secondary);
}

@media (max-width: 640px) {
  .top-search-list {
    grid-template-columns: 1fr;
  }
}

@media (max-width: 768px) {
  .search-title {
    font-size: 24px;
  }
  
  .search-subtitle {
    font-size: 14px;
  }
  
  .search-filters {
    justify-content: flex-start;
  }
  
  .search-filters .a-space {
    flex-wrap: wrap;
  }
  
  .results-title {
    font-size: 20px;
  }
  
  .section-title {
    font-size: 18px;
  }
}

@media (max-width: 480px) {
  .search-header {
    margin-bottom: 24px;
  }
  
  .search-title {
    font-size: 20px;
  }
  
  .trending-tags {
    margin-bottom: 24px;
  }
  
  .trending-tag {
    padding: 6px 12px;
    font-size: 13px;
  }
}
</style> 