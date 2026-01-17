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

      <!-- 热门搜索 -->
      <section class="trending-section" v-else>
        <h2 class="section-title">🔥 热门搜索</h2>
        <div class="trending-tags">
          <a-tag
            v-for="tag in trendingSearches"
            :key="tag"
            class="trending-tag"
            @click="searchTrending(tag)"
          >
            {{ tag }}
          </a-tag>
        </div>

        <h2 class="section-title">💫 推荐发现</h2>
        <MediaGrid
          :items="discoverItems"
          :loading="loadingDiscover"
          :show-meta="true"
          empty-message="加载推荐内容中..."
        />
      </section>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, nextTick, onBeforeUnmount } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { Message } from '@arco-design/web-vue'
import {
  IconSearch, IconClose, IconRefresh
} from '@arco-design/web-vue/es/icon'
import MediaGrid from '../components/MediaGrid.vue'
import type { Movie, Show } from '../types/api'
import { usePageState } from '../composables/usePageState'

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
const loadingDiscover = ref(false)
const searchResults = ref<(Movie | Show)[]>([])
const discoverItems = ref<(Movie | Show)[]>([])

const filters = ref({
  type: '',
  year: '',
  genre: ''
})

const trendingSearches = ref([
  '阿凡达', '流浪地球', '复仇者联盟', '权力的游戏', '老友记',
  '肖申克的救赎', '盗梦空间', '星际穿越', '泰坦尼克号', '黑镜'
])

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

const loadDiscoverData = async () => {
  loadingDiscover.value = true
  try {
    // 加载推荐发现内容
    console.log('加载推荐发现内容')
    discoverItems.value = []
  } catch (error) {
    console.error('加载推荐内容失败:', error)
  } finally {
    loadingDiscover.value = false
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
  
  loadDiscoverData()
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
}

.search-header {
  text-align: center;
  margin-bottom: 32px;
}

.search-title {
  font-size: 32px;
  font-weight: 700;
  margin: 0 0 8px 0;
  color: #1d1d1f;
}

.search-subtitle {
  font-size: 16px;
  color: #8e8e93;
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
  color: #1d1d1f;
}

.results-count {
  font-size: 16px;
  color: #8e8e93;
  font-weight: 400;
}

.trending-section {
  margin-top: 40px;
}

.section-title {
  font-size: 20px;
  font-weight: 600;
  margin: 0 0 20px 0;
  color: #1d1d1f;
}

.trending-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-bottom: 40px;
}

.trending-tag {
  cursor: pointer;
  padding: 8px 16px;
  background: white;
  border: 1px solid #e8e8e8;
  border-radius: 20px;
  transition: all 0.2s ease;
}

.trending-tag:hover {
  background: #f5f5f7;
  border-color: #d1d1d1;
  transform: translateY(-1px);
}

/* 响应式设计 */
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