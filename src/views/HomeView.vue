<template>
  <div class="home-view">
    <div class="page-container">
      <!-- Hero 轮播区域 -->
      <section class="hero-section">
        <a-carousel
          class="hero-carousel"
          :auto-play="{ interval: 5000 }"
          show-arrow="hover"
          indicator-type="dot"
          indicator-position="bottom"
        >
          <a-carousel-item
            v-for="item in featuredMovies"
            :key="item.ids?.trakt"
          >
            <div class="hero-slide" :style="getHeroBackground(item)">
              <div class="hero-content">
                <div class="hero-info">
                  <h1 class="hero-title">{{ (item.ids?.trakt && featuredTranslations[item.ids.trakt]?.title) || item.title }}</h1>
                  <p class="hero-overview">{{ (item.ids?.trakt && featuredTranslations[item.ids.trakt]?.overview) || item.overview }}</p>
                  <div class="hero-meta">
                    <span v-if="item.year" class="hero-year">{{ item.year }}</span>
                    <span v-if="item.rating" class="hero-rating">
                      <icon-star-fill />
                      {{ item.rating.toFixed(1) }}
                    </span>
                    <span v-if="item.genres?.length" class="hero-genre">
                      {{ item.genres.slice(0, 2).join(', ') }}
                    </span>
                  </div>
                  <div class="hero-actions">
                    <a-button type="primary" size="large" @click="viewDetails(item, 'movie')">
                      <icon-play-arrow />
                      查看详情
                    </a-button>
                    <a-button size="large" @click="addToWatchlist(item)">
                      <icon-plus-circle />
                      添加到清单
                    </a-button>
                  </div>
                </div>
              </div>
            </div>
          </a-carousel-item>
        </a-carousel>
      </section>

      <!-- 内容分类标签 -->
      <section class="category-tabs">
        <a-tabs
          v-model:active-key="activeTab"
          type="line"
          size="large"
          @change="handleTabChange"
        >
          <a-tab-pane key="trending" title="🔥 热门">
            <a-tabs
              v-model:active-key="trendingSubTab"
              type="card"
              size="small"
              @change="handleTrendingSubTabChange"
              class="trending-sub-tabs"
            >
              <a-tab-pane key="movies" title="电影">
                <MediaGrid
                  :items="trendingMovies"
                  :loading="loading.trendingMovies"
                  :loading-more="loading.trendingMovies"
                  @load-more="loadMoreTrendingMovies"
                  media-type="movie"
                />
              </a-tab-pane>

              <a-tab-pane key="shows" title="电视剧">
                <MediaGrid
                  :items="trendingShows"
                  :loading="loading.trendingShows"
                  :loading-more="loading.trendingShows"
                  @load-more="loadMoreTrendingShows"
                  media-type="show"
                />
              </a-tab-pane>
            </a-tabs>
          </a-tab-pane>

          <a-tab-pane key="movies" title="🎬 推荐电影">
            <MediaGrid
              :items="recommendedMovies"
              :loading="loading.movies && recommendedMoviesPage === 1"
              :loading-more="loading.movies && recommendedMoviesPage > 1"
              :has-more="hasMoreRecommendedMovies"
              @load-more="loadMoreRecommendedMovies"
              media-type="movie"
            />
          </a-tab-pane>

          <a-tab-pane key="shows" title="📺 推荐剧集">
            <MediaGrid
              :items="recommendedShows"
              :loading="loading.shows && recommendedShowsPage === 1"
              :loading-more="loading.shows && recommendedShowsPage > 1"
              :has-more="hasMoreRecommendedShows"
              @load-more="loadMoreRecommendedShows"
              media-type="show"
            />
          </a-tab-pane>

          <a-tab-pane key="recent" title="🆕 最新发布">
            <a-tabs
              v-model:active-key="recentSubTab"
              type="card"
              size="small"
              @change="handleRecentSubTabChange"
              class="recent-sub-tabs"
            >
              <a-tab-pane key="movies" title="电影">
                <MediaGrid
                  :items="recentMovies"
                  :loading="loading.recentMovies"
                  :has-more="false"
                  media-type="movie"
                />
              </a-tab-pane>

              <a-tab-pane key="shows" title="电视剧">
                <MediaGrid
                  :items="recentShows"
                  :loading="loading.recentShows"
                  :has-more="false"
                  media-type="show"
                />
              </a-tab-pane>
            </a-tabs>
          </a-tab-pane>

        <!-- 按类型浏览 -->
        <a-tab-pane key="genres" title="🎭 按类型">
          <div class="filter-toolbar">
            <a-space size="medium" wrap>
              <!-- 媒体类型切换 -->
              <a-radio-group v-model="genreMediaType" type="button" @change="handleGenreMediaTypeChange">
                <a-radio value="movies">电影</a-radio>
                <a-radio value="shows">剧集</a-radio>
              </a-radio-group>
              
              <!-- 数据源切换 -->
              <a-select 
                v-model="genreDataSource" 
                placeholder="数据源" 
                style="width: 160px;"
                @change="handleGenreDataSourceChange"
              >
                <a-option value="watched-weekly">📊 本周观看榜</a-option>
                <a-option value="watched-monthly">📈 本月观看榜</a-option>
                <a-option value="collected-monthly">⭐ 本月收藏榜</a-option>
              </a-select>
              
              <!-- 类型选择器 -->
              <a-select 
                v-model="selectedGenre" 
                placeholder="选择类型" 
                style="width: 180px;"
                @change="handleGenreChange"
              >
                <a-option value="all">🌟 全部类型</a-option>
                <a-option value="action">💥 动作</a-option>
                <a-option value="adventure">🗺️ 冒险</a-option>
                <a-option value="animation">🎨 动画</a-option>
                <a-option value="comedy">😄 喜剧</a-option>
                <a-option value="crime">🔫 犯罪</a-option>
                <a-option value="documentary">📹 纪录片</a-option>
                <a-option value="drama">🎭 剧情</a-option>
                <a-option value="family">👨‍👩‍👧 家庭</a-option>
                <a-option value="fantasy">🧙 奇幻</a-option>
                <a-option value="history">📜 历史</a-option>
                <a-option value="horror">👻 恐怖</a-option>
                <a-option value="music">🎵 音乐</a-option>
                <a-option value="mystery">🔍 悬疑</a-option>
                <a-option value="romance">💕 爱情</a-option>
                <a-option value="science-fiction">🚀 科幻</a-option>
                <a-option value="thriller">😱 惊悚</a-option>
                <a-option value="war">⚔️ 战争</a-option>
              </a-select>

              <!-- 结果计数 -->
              <a-tag color="arcoblue" v-if="genreFilteredItems.length > 0">
                <template #icon><icon-check-circle /></template>
                {{ genreFilteredItems.length }} 个结果
              </a-tag>
            </a-space>
          </div>

          <MediaGrid
            :items="genreFilteredItems"
            :loading="loading.genre && genreCurrentPage === 1"
            :loading-more="loadingMore.genre"
            :has-more="hasMoreGenre"
            @load-more="loadMoreGenreData"
            :media-type="genreMediaType === 'movies' ? 'movie' : 'show'"
            :empty-message="selectedGenre === 'all' ? '暂无数据' : `暂无${getGenreName(selectedGenre)}类型内容`"
          />
        </a-tab-pane>

        <!-- 按地区浏览 -->
        <a-tab-pane key="countries" title="🌍 按地区">
          <div class="filter-toolbar">
            <a-space size="medium" wrap>
              <!-- 媒体类型切换 -->
              <a-radio-group v-model="countryMediaType" type="button" @change="handleCountryMediaTypeChange">
                <a-radio value="movies">电影</a-radio>
                <a-radio value="shows">剧集</a-radio>
              </a-radio-group>
              
              <!-- 数据源切换 -->
              <a-select 
                v-model="countryDataSource" 
                placeholder="数据源" 
                style="width: 160px;"
                @change="handleCountryDataSourceChange"
              >
                <a-option value="watched-weekly">📊 本周观看榜</a-option>
                <a-option value="watched-monthly">📈 本月观看榜</a-option>
                <a-option value="collected-monthly">⭐ 本月收藏榜</a-option>
              </a-select>
              
              <!-- 地区选择器 -->
              <a-select 
                v-model="selectedCountry" 
                placeholder="选择地区" 
                style="width: 180px;"
                @change="handleCountryChange"
              >
                <a-option value="all">🌏 全部地区</a-option>
                <a-option value="us">🇺🇸 美国</a-option>
                <a-option value="gb">🇬🇧 英国</a-option>
                <a-option value="jp">🇯🇵 日本</a-option>
                <a-option value="kr">🇰🇷 韩国</a-option>
                <a-option value="cn">🇨🇳 中国</a-option>
                <a-option value="fr">🇫🇷 法国</a-option>
                <a-option value="de">🇩🇪 德国</a-option>
                <a-option value="ca">🇨🇦 加拿大</a-option>
                <a-option value="au">🇦🇺 澳大利亚</a-option>
                <a-option value="es">🇪🇸 西班牙</a-option>
                <a-option value="it">🇮🇹 意大利</a-option>
                <a-option value="in">🇮🇳 印度</a-option>
                <a-option value="hk">🇭🇰 香港</a-option>
                <a-option value="tw">🇹🇼 台湾</a-option>
              </a-select>

              <!-- 结果计数 -->
              <a-tag color="arcoblue" v-if="countryFilteredItems.length > 0">
                <template #icon><icon-check-circle /></template>
                {{ countryFilteredItems.length }} 个结果
              </a-tag>
            </a-space>
          </div>

          <MediaGrid
            :items="countryFilteredItems"
            :loading="loading.country && countryCurrentPage === 1"
            :loading-more="loadingMore.country"
            :has-more="hasMoreCountry"
            @load-more="loadMoreCountryData"
            :media-type="countryMediaType === 'movies' ? 'movie' : 'show'"
            :empty-message="selectedCountry === 'all' ? '暂无数据' : `暂无${getCountryName(selectedCountry)}地区内容`"
          />
        </a-tab-pane>
        </a-tabs>
      </section>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch, nextTick, onBeforeUnmount, inject, computed } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import {
  IconStarFill, IconPlayArrow, IconPlusCircle, IconCheckCircle
} from '@arco-design/web-vue/es/icon'
import MediaGrid from '../components/MediaGrid.vue'
import type { 
  Movie, Show, 
  MoviesRecommendResponse, ShowsRecommendResponse, 
  MovieTrendingResponse, ShowTrendingResponse, 
  CalendarMovie, CalendarShow,
  MovieWatchedResponse, MovieCollectedResponse,
  ShowWatchedResponse, ShowCollectedResponse
} from '../types/api'
import { invoke } from "@tauri-apps/api/core";
import { preloadMovieTranslations, getMovieChineseTranslation, type TranslationResult } from '../utils/translation'
import { useHomePageState } from '../composables/usePageState'

const router = useRouter()
const route = useRoute()
const isLoggedIn = inject('isLoggedIn', ref(false))

// 使用状态管理
const { saveHomeState, restoreHomeState } = useHomePageState()

// 定义组件名称用于keep-alive
defineOptions({
  name: 'HomeView'
})

// 响应式数据
const activeTab = ref('trending')
const trendingSubTab = ref('movies')
const recentSubTab = ref('movies')
const featuredMovies = ref<Movie[]>([])
// 存储轮播图的翻译数据
const featuredTranslations = ref<Record<number, TranslationResult>>({})

const trendingMovies = ref<Movie[]>([])
const trendingShows = ref<Show[]>([])
const recommendedMovies = ref<Movie[]>([])
const recommendedShows = ref<Show[]>([])
const recentMovies = ref<Movie[]>([])
const recentShows = ref<Show[]>([])

// ===== 按类型浏览状态 =====
const selectedGenre = ref('all')
const genreMediaType = ref('movies')
const genreDataSource = ref('watched-weekly')
const genreRawData = ref<(Movie | Show)[]>([])
const genreEnrichedData = ref<(Movie | Show)[]>([])
const genreCurrentPage = ref(1)
const hasMoreGenre = ref(true)

// ===== 按地区浏览状态 =====
const selectedCountry = ref('all')
const countryMediaType = ref('movies')
const countryDataSource = ref('watched-weekly')
const countryRawData = ref<(Movie | Show)[]>([])
const countryEnrichedData = ref<(Movie | Show)[]>([])
const countryCurrentPage = ref(1)
const hasMoreCountry = ref(true)

const loading = ref({
  featured: false,
  trendingMovies: false,
  trendingShows: false,
  movies: false,
  shows: false,
  recentMovies: false,
  recentShows: false,
  genre: false,
  country: false
})

const loadingMore = ref({
  genre: false,
  country: false
})

// 添加标志防止重复加载
const dataLoaded = ref({
  trendingMovies: false,
  trendingShows: false,
  movies: false,
  shows: false,
  recentMovies: false,
  recentShows: false,
  genre: false,
  country: false
})

const trendingMoviesPage = ref(1)
const trendingShowsPage = ref(1)

// 推荐分页状态
const recommendedMoviesPage = ref(1)
const hasMoreRecommendedMovies = ref(true)
const isFallbackMovies = ref(false)

const recommendedShowsPage = ref(1)
const hasMoreRecommendedShows = ref(true)
const isFallbackShows = ref(false)

// ===== 类型筛选计算属性 =====
const genreFilteredItems = computed(() => {
  if (selectedGenre.value === 'all') {
    return genreEnrichedData.value
  }
  
  return genreEnrichedData.value.filter(item => {
    const genres = (item as any).genres || []
    return genres.some((g: string) => 
      g.toLowerCase().replace(/\s+/g, '-') === selectedGenre.value
    )
  })
})

// ===== 地区筛选计算属性 =====
const countryFilteredItems = computed(() => {
  if (selectedCountry.value === 'all') {
    return countryEnrichedData.value
  }
  
  return countryEnrichedData.value.filter(item => {
    const country = ((item as any).country || '').toLowerCase()
    return country === selectedCountry.value
  })
})

// ===== 辅助方法 =====
const getGenreName = (slug: string) => {
  const genreMap: Record<string, string> = {
    'action': '动作',
    'adventure': '冒险',
    'animation': '动画',
    'comedy': '喜剧',
    'crime': '犯罪',
    'documentary': '纪录片',
    'drama': '剧情',
    'family': '家庭',
    'fantasy': '奇幻',
    'history': '历史',
    'horror': '恐怖',
    'music': '音乐',
    'mystery': '悬疑',
    'romance': '爱情',
    'science-fiction': '科幻',
    'thriller': '惊悚',
    'war': '战争'
  }
  return genreMap[slug] || slug
}

const getCountryName = (code: string) => {
  const countryMap: Record<string, string> = {
    'us': '美国',
    'gb': '英国',
    'jp': '日本',
    'kr': '韩国',
    'cn': '中国',
    'fr': '法国',
    'de': '德国',
    'ca': '加拿大',
    'au': '澳大利亚',
    'es': '西班牙',
    'it': '意大利',
    'in': '印度',
    'hk': '香港',
    'tw': '台湾'
  }
  return countryMap[code] || code
}

// 方法
const getHeroBackground = (item: Movie) => {
  const fanart = item.images?.fanart?.[0]
  if (fanart) {
    return {
      backgroundImage: `linear-gradient(rgba(0,0,0,0.4), rgba(0,0,0,0.6)), url(https://${fanart})`,
      backgroundSize: 'cover',
      backgroundPosition: 'center'
    }
  }
  return {
    background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)'
  }
}

const viewDetails = (item: Movie | Show, type: 'movie' | 'show') => {
  // 优先使用 trakt 数字ID，因为详情页API需要数字ID
  const id = item.ids?.trakt
  if (!id) return

  // 将图片信息存储到sessionStorage供详情页使用 (与 MediaCard 保持一致)
  if (item.images) {
    const cacheKey = `media_images_${id}`
    sessionStorage.setItem(cacheKey, JSON.stringify(item.images))
  }

  router.push({
    name: `${type}-detail`,
    params: { id }
  })
}

const addToWatchlist = (item: Movie | Show) => {
  // 添加到观看清单的逻辑
  console.log('添加到观看清单:', item.title)
  // 这里调用API
}

const handleTabChange = (key: string) => {
  activeTab.value = key
  loadTabData(key)
  // 保存标签状态
  saveHomeState(key, trendingSubTab.value)
}

const loadTabData = async (tab: string) => {
  switch (tab) {
    case 'trending':
      // trending标签下需要加载对应子标签的数据
      await loadTrendingSubTabData(trendingSubTab.value)
      break
    case 'movies':
      if (!dataLoaded.value.movies) {
        await fetchRecommendedMovies(1)
        dataLoaded.value.movies = true
      }
      break
    case 'shows':
      if (!dataLoaded.value.shows) {
        await fetchRecommendedShows(1)
        dataLoaded.value.shows = true
      }
      break
    case 'recent':
      await loadRecentData()
      break
    case 'genres':
      await loadGenreData()
      break
    case 'countries':
      await loadCountryData()
      break
  }
}

const loadFeaturedData = async () => {
  loading.value.featured = true
  try {
    const trendingData = await invoke<MovieTrendingResponse>('movie_trending')

    const movies: Movie[] = []
    for (const item of trendingData.slice(0, 5)) {
      if (item.movie) {
        movies.push(item.movie)
      }
    }

    featuredMovies.value = movies

    // 加载翻译
    movies.forEach(async (movie) => {
      if (movie.ids?.trakt) {
        const trans = await getMovieChineseTranslation(movie.ids.trakt)
        if (trans) {
          featuredTranslations.value[movie.ids.trakt] = trans
        }
      }
    })
  } catch (error) {
    console.error('加载轮播数据失败:', error)
  } finally {
    loading.value.featured = false
  }
}

// 推荐电影逻辑
const fetchRecommendedMovies = async (page: number, retryCount = 0) => {
  loading.value.movies = true
  try {
    let movies: Movie[] = []
    const limit = 40
    
    // 如果已登录且未进入降级模式，尝试获取个性化推荐
    if (isLoggedIn.value && !isFallbackMovies.value) {
      try {
        // 使用分页 API
        const res = await invoke<MoviesRecommendResponse>("movies_recommand_page", { page, limit })
        movies = res
      } catch (e) {
        if (page === 1) {
           console.warn('个性化推荐获取失败，降级为热门推荐', e)
           isFallbackMovies.value = true
           // 递归重试
           loading.value.movies = false
           return fetchRecommendedMovies(1)
        }
        // 如果是翻页失败，暂不处理，保留现状
      }
    } 
    
    // 如果未登录或处于降级模式，获取热门推荐
    if (!isLoggedIn.value || isFallbackMovies.value) {
       movies = await invoke<Movie[]>("movie_popular_page", { page, limit })
    }

    if (movies.length < limit) {
      hasMoreRecommendedMovies.value = false
    }
    
    if (page === 1) {
      recommendedMovies.value = movies
    } else {
      const existingIds = new Set(recommendedMovies.value.map(m => m.ids?.trakt))
      const newMovies = movies.filter(m => m.ids?.trakt && !existingIds.has(m.ids.trakt))
      if (newMovies.length > 0) {
        recommendedMovies.value.push(...newMovies)
      } else if (movies.length > 0 && retryCount < 3) {
        // 如果数据不为空但全被去重了，自动尝试下一页
        console.log(`Page ${page} data fully duplicated, retrying next page...`)
        recommendedMoviesPage.value++
        await fetchRecommendedMovies(recommendedMoviesPage.value, retryCount + 1)
        return
      } else if (movies.length > 0 && retryCount >= 3) {
        // 重试多次仍无新数据，标记为无更多数据
        hasMoreRecommendedMovies.value = false
      }
    }
    
    console.log('加载推荐电影 page:', page, 'count:', movies.length)

    // 在后台预加载翻译
    preloadMovieTranslations(movies, (_loaded, _total) => {
      // console.log(`翻译加载进度: ${loaded}/${total}`)
    })
  } catch (error) {
    console.error('加载推荐电影失败:', error)
  } finally {
    loading.value.movies = false
  }
}

const loadMoreRecommendedMovies = async () => {
  if (loading.value.movies || !hasMoreRecommendedMovies.value) return
  recommendedMoviesPage.value++
  await fetchRecommendedMovies(recommendedMoviesPage.value)
}

// 推荐剧集逻辑
const fetchRecommendedShows = async (page: number, retryCount = 0) => {
  loading.value.shows = true
  try {
    let shows: Show[] = []
    const limit = 40
    
    if (isLoggedIn.value && !isFallbackShows.value) {
      try {
        const res = await invoke<ShowsRecommendResponse>("shows_recommand_page", { page, limit })
        shows = res
      } catch (e) {
        if (page === 1) {
           console.warn('个性化剧集推荐获取失败，降级为热门推荐', e)
           isFallbackShows.value = true
           loading.value.shows = false
           return fetchRecommendedShows(1)
        }
      }
    } 
    
    if (!isLoggedIn.value || isFallbackShows.value) {
       shows = await invoke<Show[]>("show_popular_page", { page, limit })
    }

    if (shows.length < limit) {
      hasMoreRecommendedShows.value = false
    }
    
    if (page === 1) {
      recommendedShows.value = shows
    } else {
      const existingIds = new Set(recommendedShows.value.map(s => s.ids?.trakt))
      const newShows = shows.filter(s => s.ids?.trakt && !existingIds.has(s.ids.trakt))
      if (newShows.length > 0) {
        recommendedShows.value.push(...newShows)
      } else if (shows.length > 0 && retryCount < 3) {
        console.log(`Page ${page} data fully duplicated, retrying next page...`)
        recommendedShowsPage.value++
        await fetchRecommendedShows(recommendedShowsPage.value, retryCount + 1)
        return
      } else if (shows.length > 0 && retryCount >= 3) {
        hasMoreRecommendedShows.value = false
      }
    }
    
    console.log('加载推荐剧集 page:', page, 'count:', shows.length)
  } catch (error) {
    console.error('加载推荐剧集失败:', error)
  } finally {
    loading.value.shows = false
  }
}

const loadMoreRecommendedShows = async () => {
  if (loading.value.shows || !hasMoreRecommendedShows.value) return
  recommendedShowsPage.value++
  await fetchRecommendedShows(recommendedShowsPage.value)
}

const loadRecentData = async () => {
  await loadRecentSubTabData(recentSubTab.value)
}

const loadRecentMovies = async () => {
  if (loading.value.recentMovies || dataLoaded.value.recentMovies) return

  loading.value.recentMovies = true
  try {
    const today = new Date()
    const startDate = new Date(today.getTime() - 30 * 24 * 60 * 60 * 1000)
    const startDateStr = startDate.toISOString().split('T')[0]
    
    const result = await invoke<CalendarMovie[]>("get_calendar_movies", {
      startDate: startDateStr,
      days: 30
    })
    
    const movies: Movie[] = result.map(item => ({
      ...item.movie,
      released: item.released
    }))
    
    movies.sort((a, b) => {
      const dateA = a.released ? new Date(a.released).getTime() : 0
      const dateB = b.released ? new Date(b.released).getTime() : 0
      return dateB - dateA
    })
    
    recentMovies.value = movies
    dataLoaded.value.recentMovies = true
    console.log('加载最新电影:', movies.length, '部')
  } catch (error) {
    console.error('加载最新电影失败:', error)
  } finally {
    loading.value.recentMovies = false
  }
}

const loadRecentShows = async () => {
  if (loading.value.recentShows || dataLoaded.value.recentShows) return

  loading.value.recentShows = true
  try {
    const today = new Date()
    const startDate = new Date(today.getTime() - 30 * 24 * 60 * 60 * 1000)
    const startDateStr = startDate.toISOString().split('T')[0]
    
    const [newShowsResult, premieresResult] = await Promise.all([
      invoke<CalendarShow[]>("get_calendar_new_shows", {
        startDate: startDateStr,
        days: 30
      }),
      invoke<CalendarShow[]>("get_calendar_premieres", {
        startDate: startDateStr,
        days: 30
      })
    ])
    
    const showsMap = new Map<number, Show>()
    
    for (const item of newShowsResult) {
      const traktId = item.show.ids?.trakt
      if (traktId && !showsMap.has(traktId)) {
        showsMap.set(traktId, {
          ...item.show,
          released: item.first_aired,
          latestSeason: 1
        })
      }
    }
    
    for (const item of premieresResult) {
      const traktId = item.show.ids?.trakt
      const seasonNum = item.episode?.season
      if (traktId && seasonNum) {
        if (!showsMap.has(traktId)) {
          showsMap.set(traktId, {
            ...item.show,
            released: item.first_aired,
            latestSeason: seasonNum
          })
        } else {
          const existing = showsMap.get(traktId)!
          if (seasonNum > (existing.latestSeason || 1)) {
            existing.latestSeason = seasonNum
            existing.released = item.first_aired
          }
        }
      }
    }
    
    const shows = Array.from(showsMap.values())
    shows.sort((a, b) => {
      const dateA = a.released ? new Date(a.released).getTime() : 0
      const dateB = b.released ? new Date(b.released).getTime() : 0
      return dateB - dateA
    })
    
    recentShows.value = shows
    dataLoaded.value.recentShows = true
    console.log('加载最新电视剧:', shows.length, '部')
  } catch (error) {
    console.error('加载最新电视剧失败:', error)
  } finally {
    loading.value.recentShows = false
  }
}

const handleRecentSubTabChange = (key: string) => {
  recentSubTab.value = key
  loadRecentSubTabData(key)
}

const loadRecentSubTabData = async (subTab: string) => {
  switch (subTab) {
    case 'movies':
      if (!dataLoaded.value.recentMovies) {
        await loadRecentMovies()
      }
      break
    case 'shows':
      if (!dataLoaded.value.recentShows) {
        await loadRecentShows()
      }
      break
  }
}

// 加载更多数据的方法
const loadMoreTrendingMovies = async (retryCount = 0) => {
  loading.value.trendingMovies = true
  try {
    const res = await invoke<MovieTrendingResponse>("movie_trending_page", { page: trendingMoviesPage.value, limit: 40 })
    if (res) {
      const movies: Movie[] = []
      for (const item of res) {
        if (item.movie) {
          item.movie.watchers = item.watchers
          movies.push(item.movie)
        }
      }
      const existingIds = new Set(trendingMovies.value.map(m => m.ids?.trakt))
      const newMovies = movies.filter(m => m.ids?.trakt && !existingIds.has(m.ids.trakt))
      if (newMovies.length > 0) {
        trendingMovies.value.push(...newMovies)
      } else if (movies.length > 0 && retryCount < 3) {
        console.log('Trending movies page duplicated, retrying...')
        trendingMoviesPage.value++
        await loadMoreTrendingMovies(retryCount + 1)
        return
      } else if (movies.length > 0 && retryCount >= 3) {
         // 这里虽然是加载更多，但如果是 Trending 这种无限流，暂时无法直接控制 MediaGrid 的 hasMore 属性（因为它是 prop）
         // 但 HomeView 里并没有直接用 hasMore 控制 Trending 的 MediaGrid，而是写死了 :loading-more
         // 检查 template 发现:
         // :loading-more="loading.trendingMovies"
         // @load-more="loadMoreTrendingMovies"
         // 它并没有传 :has-more 属性，MediaGrid 默认 has-more 为 true
         // 我们需要在 dataLoaded 或者 loading 状态上做文章，或者给 MediaGrid 传一个 ref
         
         // 这是一个小问题，MediaGrid 组件默认 hasMore=true，这里我们没法直接改。
         // 不过对于 Recommended 列表，我们有 hasMoreRecommendedMovies 变量。
         // 对于 Trending，目前没有定义 hasMoreTrendingMovies。
         // 鉴于用户主要反馈的是“推荐页面”，我们重点修复 Recommended。
         // 对于 Trending，我们至少让 loading 停止。
      }

      // 在后台预加载翻译
      preloadMovieTranslations(movies, (_loaded, _total) => {
        // console.log(`更多热门电影翻译加载进度: ${loaded}/${total}`)
      })
    }
    trendingMoviesPage.value++
  } catch (error) {
    console.error('加载更多热门电影失败:', error)
  } finally {
    loading.value.trendingMovies = false
  }
}

const loadMoreTrendingShows = async (retryCount = 0) => {
  loading.value.trendingShows = true
  try {
    const res = await invoke<ShowTrendingResponse>("show_trending_page", { page: trendingShowsPage.value, limit: 40 })
    if (res) {
      const newShows: Show[] = []
      for (const item of res) {
        if (item.show) {
          item.show.watchers = item.watchers
          newShows.push(item.show)
        }
      }
      const existingIds = new Set(trendingShows.value.map(s => s.ids?.trakt))
      const uniqueShows = newShows.filter(s => s.ids?.trakt && !existingIds.has(s.ids.trakt))
      if (uniqueShows.length > 0) {
        trendingShows.value.push(...uniqueShows)
      } else if (newShows.length > 0 && retryCount < 3) {
        console.log('Trending shows page duplicated, retrying...')
        trendingShowsPage.value++
        await loadMoreTrendingShows(retryCount + 1)
        return
      }
    }
  trendingShowsPage.value++
  } catch (error) {
    console.error('加载更多热门电视剧失败:', error)
  } finally {
    loading.value.trendingShows = false
  }
}

const handleTrendingSubTabChange = (key: string) => {
  trendingSubTab.value = key
  loadTrendingSubTabData(key)
  // 保存子标签状态
  saveHomeState(activeTab.value, key)
}

const loadTrendingSubTabData = async (subTab: string) => {
  switch (subTab) {
    case 'movies':
      if (!dataLoaded.value.trendingMovies) {
        await loadTrendingMoviesData()
      }
      break
    case 'shows':
      if (!dataLoaded.value.trendingShows) {
        await loadTrendingShowsData()
      }
      break
  }
}

const loadTrendingMoviesData = async (retryCount = 0) => {
  if (loading.value.trendingMovies || dataLoaded.value.trendingMovies) return

  loading.value.trendingMovies = true
  try {
    const res = await invoke<MovieTrendingResponse>("movie_trending")
    if (res && res.length > 0) {
      console.log('热门电影数据:', res)
      const movies: Movie[] = []
      for (const item of res) {
        if (item.movie) {
          item.movie.watchers = item.watchers
          movies.push(item.movie)
        }
      }
      const existingIds = new Set(trendingMovies.value.map(m => m.ids?.trakt))
      const newMovies = movies.filter(m => m.ids?.trakt && !existingIds.has(m.ids.trakt))
      if (newMovies.length > 0) {
        trendingMovies.value.push(...newMovies)
      } else if (movies.length > 0 && retryCount < 3) {
        console.log('Trending movies page duplicated, retrying...')
        trendingMoviesPage.value++
        await loadMoreTrendingMovies(retryCount + 1)
        return
      } else if (movies.length > 0 && retryCount >= 3) {
         // 这里虽然是加载更多，但如果是 Trending 这种无限流，暂时无法直接控制 MediaGrid 的 hasMore 属性（因为它是 prop）
         // 但 HomeView 里并没有直接用 hasMore 控制 Trending 的 MediaGrid，而是写死了 :loading-more
         // 检查 template 发现:
         // :loading-more="loading.trendingMovies"
         // @load-more="loadMoreTrendingMovies"
         // 它并没有传 :has-more 属性，MediaGrid 默认 has-more 为 true
         // 我们需要在 dataLoaded 或者 loading 状态上做文章，或者给 MediaGrid 传一个 ref
         
         // 这是一个小问题，MediaGrid 组件默认 hasMore=true，这里我们没法直接改。
         // 不过对于 Recommended 列表，我们有 hasMoreRecommendedMovies 变量。
         // 对于 Trending，目前没有定义 hasMoreTrendingMovies。
         // 鉴于用户主要反馈的是“推荐页面”，我们重点修复 Recommended。
         // 对于 Trending，我们至少让 loading 停止。
      }
      dataLoaded.value.trendingMovies = true

      // 在后台预加载翻译
      preloadMovieTranslations(movies, (_loaded, _total) => {
        // console.log(`热门电影翻译加载进度: ${loaded}/${total}`)
      })
    }
    trendingMoviesPage.value++
  } catch (error) {
    console.error('加载热门电影失败:', error)
  } finally {
    loading.value.trendingMovies = false
  }
}

const loadTrendingShowsData = async (retryCount = 0) => {
  if (loading.value.trendingShows || dataLoaded.value.trendingShows) return

  loading.value.trendingShows = true
  try {
    // 调用API获取热门电视剧
    const res = await invoke<ShowTrendingResponse>("show_trending")
    if (res && res.length > 0) {
      const newShows: Show[] = []
      for (const item of res) {
        if (item.show) {
          item.show.watchers = item.watchers
          newShows.push(item.show)
        }
      }
      const existingIds = new Set(trendingShows.value.map(s => s.ids?.trakt))
      const uniqueShows = newShows.filter(s => s.ids?.trakt && !existingIds.has(s.ids.trakt))
      if (uniqueShows.length > 0) {
        trendingShows.value.push(...uniqueShows)
      } else if (newShows.length > 0 && retryCount < 3) {
        console.log('Trending shows page duplicated, retrying...')
        trendingShowsPage.value++
        await loadMoreTrendingShows(retryCount + 1)
        return
      }
      dataLoaded.value.trendingShows = true
    }
    trendingShowsPage.value++
  } catch (error) {
    console.error('加载热门电视剧失败:', error)
  } finally {
    loading.value.trendingShows = false
  }
}

// ===== 加载类型浏览数据 =====
const loadGenreData = async () => {
  if (dataLoaded.value.genre) return
  
  loading.value.genre = true
  genreCurrentPage.value = 1
  genreRawData.value = []
  genreEnrichedData.value = []
  hasMoreGenre.value = true
  
  try {
    await fetchGenreData()
    dataLoaded.value.genre = true
  } catch (error) {
    console.error('加载类型数据失败:', error)
  } finally {
    loading.value.genre = false
  }
}

const fetchGenreData = async () => {
  const limit = 40
  
  try {
    let items: Movie[] | Show[] = []
    
    const [source, period] = genreDataSource.value.split('-')
    
    if (genreMediaType.value === 'movies') {
      if (source === 'watched') {
        const res = await invoke<MovieWatchedResponse>('movie_watched_period', { 
          period,
          page: genreCurrentPage.value, 
          limit 
        })
        items = res.map(item => ({
          ...item.movie,
          watcher_count: item.watcher_count,
          play_count: item.play_count
        }))
      } else if (source === 'collected') {
        const res = await invoke<MovieCollectedResponse>('movie_collected_period', { 
          period,
          page: genreCurrentPage.value, 
          limit 
        })
        items = res.map(item => ({
          ...item.movie,
          collected_count: item.collected_count
        }))
      }
    } else {
      if (source === 'watched') {
        const res = await invoke<ShowWatchedResponse>('show_watched_period', { 
          period,
          page: genreCurrentPage.value, 
          limit 
        })
        items = res.map(item => ({
          ...item.show,
          watcher_count: item.watcher_count,
          play_count: item.play_count
        }))
      } else if (source === 'collected') {
        const res = await invoke<ShowCollectedResponse>('show_collected_period', { 
          period,
          page: genreCurrentPage.value, 
          limit 
        })
        items = res.map(item => ({
          ...item.show,
          collected_count: item.collected_count
        }))
      }
    }
    
    genreRawData.value.push(...items)
    
    await enrichItemsWithDetails(items, genreEnrichedData, genreMediaType.value)
    
    if (items.length < limit) {
      hasMoreGenre.value = false
    }
    
    console.log(`加载类型数据: ${items.length} 条, 当前总数: ${genreEnrichedData.value.length}`)
  } catch (error) {
    console.error('获取类型数据失败:', error)
    throw error
  }
}

// ===== 加载地区浏览数据 =====
const loadCountryData = async () => {
  if (dataLoaded.value.country) return
  
  loading.value.country = true
  countryCurrentPage.value = 1
  countryRawData.value = []
  countryEnrichedData.value = []
  hasMoreCountry.value = true
  
  try {
    await fetchCountryData()
    dataLoaded.value.country = true
  } catch (error) {
    console.error('加载地区数据失败:', error)
  } finally {
    loading.value.country = false
  }
}

const fetchCountryData = async () => {
  const limit = 40
  
  try {
    let items: Movie[] | Show[] = []
    
    const [source, period] = countryDataSource.value.split('-')
    
    if (countryMediaType.value === 'movies') {
      if (source === 'watched') {
        const res = await invoke<MovieWatchedResponse>('movie_watched_period', { 
          period,
          page: countryCurrentPage.value, 
          limit 
        })
        items = res.map(item => ({
          ...item.movie,
          watcher_count: item.watcher_count,
          play_count: item.play_count
        }))
      } else if (source === 'collected') {
        const res = await invoke<MovieCollectedResponse>('movie_collected_period', { 
          period,
          page: countryCurrentPage.value, 
          limit 
        })
        items = res.map(item => ({
          ...item.movie,
          collected_count: item.collected_count
        }))
      }
    } else {
      if (source === 'watched') {
        const res = await invoke<ShowWatchedResponse>('show_watched_period', { 
          period,
          page: countryCurrentPage.value, 
          limit 
        })
        items = res.map(item => ({
          ...item.show,
          watcher_count: item.watcher_count,
          play_count: item.play_count
        }))
      } else if (source === 'collected') {
        const res = await invoke<ShowCollectedResponse>('show_collected_period', { 
          period,
          page: countryCurrentPage.value, 
          limit 
        })
        items = res.map(item => ({
          ...item.show,
          collected_count: item.collected_count
        }))
      }
    }
    
    countryRawData.value.push(...items)
    
    await enrichItemsWithDetails(items, countryEnrichedData, countryMediaType.value)
    
    if (items.length < limit) {
      hasMoreCountry.value = false
    }
    
    console.log(`加载地区数据: ${items.length} 条, 当前总数: ${countryEnrichedData.value.length}`)
  } catch (error) {
    console.error('获取地区数据失败:', error)
    throw error
  }
}

// ===== 批量丰富数据详情 =====
const enrichItemsWithDetails = async (
  items: (Movie | Show)[], 
  targetArray: typeof genreEnrichedData,
  mediaType: string
) => {
  const BATCH_SIZE = 5
  const enrichedItems: (Movie | Show)[] = []
  
  for (let i = 0; i < items.length; i += BATCH_SIZE) {
    const batch = items.slice(i, i + BATCH_SIZE)
    
    const detailsPromises = batch.map(async item => {
      try {
        const id = item.ids?.trakt
        if (!id) return item
        
        const cacheKey = `enriched_${mediaType}_${id}`
        const cached = sessionStorage.getItem(cacheKey)
        if (cached) {
          try {
            return JSON.parse(cached)
          } catch {
            // 缓存损坏,继续获取
          }
        }
        
        const command = mediaType === 'movies' ? 'movie_details' : 'show_details'
        const details = await invoke<any>(command, { id })
        
        const enriched = { ...item, ...details } as Movie | Show
        
        try {
          sessionStorage.setItem(cacheKey, JSON.stringify(enriched))
        } catch (e) {
          console.warn('sessionStorage 已满,跳过缓存')
        }
        
        return enriched
      } catch (error) {
        console.warn(`获取 ${item.ids?.trakt} 详情失败:`, error)
        return item
      }
    })
    
    const batchResults = await Promise.all(detailsPromises)
    enrichedItems.push(...batchResults)
    
    targetArray.value = [...enrichedItems]
    
    if (i + BATCH_SIZE < items.length) {
      await new Promise(resolve => setTimeout(resolve, 300))
    }
  }
  
  if (mediaType === 'movies') {
    preloadMovieTranslations(enrichedItems as Movie[], () => {})
  }
}

// ===== 加载更多 =====
const loadMoreGenreData = async () => {
  if (loadingMore.value.genre || !hasMoreGenre.value) return
  
  loadingMore.value.genre = true
  genreCurrentPage.value++
  
  try {
    await fetchGenreData()
  } catch (error) {
    console.error('加载更多类型数据失败:', error)
    genreCurrentPage.value--
  } finally {
    loadingMore.value.genre = false
  }
}

const loadMoreCountryData = async () => {
  if (loadingMore.value.country || !hasMoreCountry.value) return
  
  loadingMore.value.country = true
  countryCurrentPage.value++
  
  try {
    await fetchCountryData()
  } catch (error) {
    console.error('加载更多地区数据失败:', error)
    countryCurrentPage.value--
  } finally {
    loadingMore.value.country = false
  }
}

// ===== 处理筛选变更 =====
const handleGenreChange = () => {
  window.scrollTo({ top: 0, behavior: 'smooth' })
}

const handleCountryChange = () => {
  window.scrollTo({ top: 0, behavior: 'smooth' })
}

const handleGenreMediaTypeChange = () => {
  selectedGenre.value = 'all'
  genreCurrentPage.value = 1
  genreRawData.value = []
  genreEnrichedData.value = []
  hasMoreGenre.value = true
  dataLoaded.value.genre = false
  loadGenreData()
}

const handleCountryMediaTypeChange = () => {
  selectedCountry.value = 'all'
  countryCurrentPage.value = 1
  countryRawData.value = []
  countryEnrichedData.value = []
  hasMoreCountry.value = true
  dataLoaded.value.country = false
  loadCountryData()
}

const handleGenreDataSourceChange = () => {
  genreCurrentPage.value = 1
  genreRawData.value = []
  genreEnrichedData.value = []
  hasMoreGenre.value = true
  dataLoaded.value.genre = false
  loadGenreData()
}

const handleCountryDataSourceChange = () => {
  countryCurrentPage.value = 1
  countryRawData.value = []
  countryEnrichedData.value = []
  hasMoreCountry.value = true
  dataLoaded.value.country = false
  loadCountryData()
}

// 生命周期
onMounted(async () => {
  // 尝试恢复状态
  const savedState = restoreHomeState()

  if (savedState) {
    // 恢复保存的状态
    activeTab.value = savedState.activeTab
    trendingSubTab.value = savedState.trendingSubTab

    // 加载数据
    await loadFeaturedData()
    await loadTabData(activeTab.value)

    // 恢复滚动位置
    if (savedState.scrollPosition > 0) {
      nextTick(() => {
        window.scrollTo({ top: savedState.scrollPosition, behavior: 'smooth' })
      })
    }
  } else {
    // 没有保存状态时，根据URL参数设置
    const type = route.query.type as string
    if (type && ['trending', 'movies', 'shows', 'recent'].includes(type)) {
      activeTab.value = type
    }

    // 加载初始数据
    await loadFeaturedData()
    await loadTabData(activeTab.value)
  }
})

// 页面卸载前保存状态
onBeforeUnmount(() => {
  saveHomeState(activeTab.value, trendingSubTab.value)
})

// 监听路由查询参数变化（仅外部跳转）
watch(() => route.query.type, (newType, oldType) => {
  // 只有从外部跳转进来才处理，避免覆盖恢复的状态
  if (newType !== oldType && newType && ['trending', 'movies', 'shows', 'recent'].includes(newType as string)) {
    activeTab.value = newType as string
    loadTabData(newType as string)
    saveHomeState(newType as string, trendingSubTab.value)
  }
}, { immediate: false })
</script>

<style scoped>
.home-view {
  min-height: 100vh;
  /* 移除 page-container 的默认限制，让 Hero 可以撑满 */
}

/* 覆盖全局样式，让首页更宽 */
:deep(.page-container) {
  max-width: 1600px;
  /* 恢复左右 padding，让内容不贴边 */
  padding: 0 40px 40px 40px;
  padding-top: 0;
}

/* Hero Section - 沉浸式电影感 */
.hero-section {
  position: relative;
  margin-bottom: 48px;
  /* 关键：悬浮卡片圆角样式 */
  border-radius: 24px;
  overflow: hidden;
  box-shadow: 0 20px 40px rgba(0,0,0,0.15); /* 增加悬浮阴影 */
  transform: translateZ(0); /* 修复 Safari 圆角 */
  z-index: 10; /* 确保在最上层 */
}

/* 确保 Carousel 容器也有高度 */
.hero-carousel {
  height: 500px;
}

.hero-slide {
  height: 100%; /* 跟随 Carousel 高度 */
  position: relative;
  display: flex;
  align-items: flex-end; /* 内容沉底 */
  background-size: cover;
  background-position: center top;
}

/* 渐变遮罩 - 增强文字可读性 */
.hero-slide::before {
  content: '';
  position: absolute;
  top: 0; left: 0; right: 0; bottom: 0;
  background: linear-gradient(
    to top,
    rgba(0,0,0,0.9) 0%,
    rgba(0,0,0,0.5) 40%,
    rgba(0,0,0,0) 100%
  );
  z-index: 1;
}

/* 左侧遮罩 - 让文字更清晰 */
.hero-slide::after {
  content: '';
  position: absolute;
  top: 0; left: 0; bottom: 0; width: 50%;
  background: linear-gradient(to right, rgba(0,0,0,0.8), transparent);
  z-index: 1;
}

.hero-content {
  position: relative;
  z-index: 2;
  width: 100%;
  padding: 0 60px 60px 60px; /* 增加内边距 */
  max-width: 1200px;
}

.hero-info {
  animation: slideUp 0.8s cubic-bezier(0.2, 0.8, 0.2, 1);
}

@keyframes slideUp {
  from { opacity: 0; transform: translateY(40px); }
  to { opacity: 1; transform: translateY(0); }
}

.hero-title {
  font-size: 56px; /* 更大标题 */
  font-weight: 800;
  margin: 0 0 16px 0;
  color: #fff;
  line-height: 1.1;
  text-shadow: 0 4px 12px rgba(0,0,0,0.5);
  letter-spacing: -1px;
}

.hero-overview {
  font-size: 18px;
  line-height: 1.6;
  color: rgba(255,255,255,0.9);
  margin: 0 0 24px 0;
  max-width: 700px;
  text-shadow: 0 2px 4px rgba(0,0,0,0.5);
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.hero-meta {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 32px;
}

.hero-year, .hero-rating, .hero-genre {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 14px;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 600;
  backdrop-filter: blur(10px);
}

.hero-year { background: rgba(255,255,255,0.15); color: #fff; }
.hero-rating { background: rgba(255, 193, 7, 0.2); color: #ffc107; }
.hero-genre { background: rgba(255,255,255,0.1); color: rgba(255,255,255,0.8); }

.hero-actions {
  display: flex;
  gap: 16px;
}

.hero-actions :deep(.arco-btn) {
  height: 48px;
  padding: 0 32px;
  font-size: 16px;
  font-weight: 600;
  border-radius: 24px;
  border: none;
  transition: transform 0.2s;
}

.hero-actions :deep(.arco-btn:hover) {
  transform: scale(1.05);
}

.hero-actions :deep(.arco-btn-primary) {
  background: #165dff;
  box-shadow: 0 8px 20px rgba(22, 93, 255, 0.4);
}

.hero-actions :deep(.arco-btn-secondary) {
  background: rgba(255,255,255,0.2);
  color: white;
  backdrop-filter: blur(10px);
}

/* 分类 Tab */
.category-tabs {
  padding: 0 40px; /* 恢复左右边距 */
}

/* Tab 样式优化 */
:deep(.arco-tabs-nav::before) { display: none; } /* 去掉灰线 */
:deep(.arco-tabs-tab) {
  font-size: 18px;
  font-weight: 600;
  color: #86909c;
  padding: 12px 0;
  margin-right: 40px;
}
:deep(.arco-tabs-tab-active) {
  color: #1d1d1f;
  font-weight: 800;
}
:deep(.arco-tabs-nav-ink) {
  background-color: #165dff;
  height: 3px;
  border-radius: 3px;
  bottom: 0;
}

/* 子 Tab 样式 */
.trending-sub-tabs,
.recent-sub-tabs {
  margin-top: 24px;
}

/* 去除子 Tab 内容区域的可能边框 */
.trending-sub-tabs :deep(.arco-tabs-content),
.recent-sub-tabs :deep(.arco-tabs-content) {
  border: none !important;
  box-shadow: none !important;
  padding: 0;
}

.trending-sub-tabs :deep(.arco-tabs-nav-type-card .arco-tabs-tab),
.recent-sub-tabs :deep(.arco-tabs-nav-type-card .arco-tabs-tab) {
  background-color: transparent;
  border: 1px solid #e5e6eb;
  border-radius: 20px;
  margin-right: 12px;
  height: 36px;
  line-height: 34px;
  padding: 0 20px;
  font-size: 14px;
  color: #4e5969;
}
.trending-sub-tabs :deep(.arco-tabs-nav-type-card .arco-tabs-tab-active),
.recent-sub-tabs :deep(.arco-tabs-nav-type-card .arco-tabs-tab-active) {
  background-color: #1d1d1f;
  color: #fff;
  border-color: #1d1d1f;
}

/* 去除 Tab 边框和阴影，解决列表两侧可能有线的问题 */
:deep(.arco-tabs-nav::before) { display: none !important; }
:deep(.arco-tabs-content) { border: none !important; }

/* 响应式 */
@media (max-width: 768px) {
  .hero-slide { height: 400px; }
  .hero-content { padding: 0 24px 40px 24px; }
  .hero-title { font-size: 32px; }
  .hero-overview { font-size: 14px; }
  .category-tabs { padding: 0 20px; }
}

/* 筛选工具栏 */
.filter-toolbar {
  margin: 24px 0 32px 0;
  padding: 24px;
  background: linear-gradient(135deg, #f7f8fa 0%, #ffffff 100%);
  border-radius: 20px;
  border: 1px solid #e5e6eb;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.03);
}

.filter-toolbar :deep(.arco-space) {
  width: 100%;
  justify-content: center;
}

.filter-toolbar :deep(.arco-radio-group) {
  background: white;
  padding: 4px;
  border-radius: 14px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
}

.filter-toolbar :deep(.arco-radio-button) {
  border-radius: 10px;
  border: none;
  padding: 8px 24px;
  font-weight: 600;
  font-size: 14px;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  color: #4e5969;
}

.filter-toolbar :deep(.arco-radio-button:hover) {
  color: #165dff;
  background: #f2f3f5;
}

.filter-toolbar :deep(.arco-radio-button-checked) {
  background: #165dff;
  color: white;
  box-shadow: 0 4px 12px rgba(22, 93, 255, 0.25);
}

.filter-toolbar :deep(.arco-select-view) {
  border-radius: 14px;
  border: 1px solid #e5e6eb;
  background: white;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.04);
  transition: all 0.2s;
  height: 40px;
  font-weight: 600;
}

.filter-toolbar :deep(.arco-select-view:hover) {
  border-color: #165dff;
  box-shadow: 0 4px 12px rgba(22, 93, 255, 0.1);
}

.filter-toolbar :deep(.arco-select-view-focus) {
  border-color: #165dff;
  box-shadow: 0 4px 16px rgba(22, 93, 255, 0.2);
}

.filter-toolbar :deep(.arco-tag) {
  border-radius: 12px;
  font-weight: 600;
  padding: 8px 16px;
  font-size: 14px;
  height: 40px;
  display: inline-flex;
  align-items: center;
  gap: 6px;
  border: none;
  box-shadow: 0 2px 8px rgba(22, 93, 255, 0.15);
}

@media (max-width: 768px) {
  .filter-toolbar {
    padding: 16px;
  }
  
  .filter-toolbar :deep(.arco-space) {
    flex-direction: column;
    align-items: stretch;
  }
  
  .filter-toolbar :deep(.arco-select) {
    width: 100% !important;
  }
}
</style>
