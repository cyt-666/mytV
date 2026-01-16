<template>
  <div class="home-view">
    <div class="page-container">
      <!-- Hero 轮播区域 -->
      <section class="hero-section">
        <a-carousel
          :style="{ height: '400px' }"
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
                  <h1 class="hero-title">{{ item.title }}</h1>
                  <p class="hero-overview">{{ item.overview }}</p>
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
              :loading="loading.movies"
              :has-more="false"
              media-type="movie"
            />
          </a-tab-pane>
          
          <a-tab-pane key="shows" title="📺 推荐剧集">
            <MediaGrid 
              :items="recommendedShows" 
              :loading="loading.shows"
              :has-more="false"
              media-type="show"
            />
          </a-tab-pane>
          
          <a-tab-pane key="recent" title="🆕 最新发布">
            <MediaGrid 
              :items="recentItems" 
              :loading="loading.recent"
              @load-more="loadMoreRecent"
              media-type="auto"
            />
          </a-tab-pane>
        </a-tabs>
      </section>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch, nextTick, onBeforeUnmount } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { 
  IconStarFill, IconPlayArrow, IconPlusCircle 
} from '@arco-design/web-vue/es/icon'
import MediaGrid from '../components/MediaGrid.vue'
import type { Movie, Show, MoviesRecommendResponse, ShowsRecommendResponse, MovieTrendingResponse, ShowTrendingResponse } from '../types/api'
import { invoke } from "@tauri-apps/api/core";
import { preloadMovieTranslations } from '../utils/translation'
import { useHomePageState } from '../composables/usePageState'

const router = useRouter()
const route = useRoute()

// 使用状态管理
const { saveHomeState, restoreHomeState } = useHomePageState()

// 定义组件名称用于keep-alive
defineOptions({
  name: 'HomeView'
})

// 响应式数据
const activeTab = ref('trending')
const trendingSubTab = ref('movies')
const featuredMovies = ref<Movie[]>([])
const trendingMovies = ref<Movie[]>([])
const trendingShows = ref<Show[]>([])
const recommendedMovies = ref<Movie[]>([])
const recommendedShows = ref<Show[]>([])
const recentItems = ref<(Movie | Show)[]>([])

const loading = ref({
  featured: false,
  trendingMovies: false,
  trendingShows: false,
  movies: false,
  shows: false,
  recent: false
})

// 添加标志防止重复加载
const dataLoaded = ref({
  trendingMovies: false,
  trendingShows: false,
  movies: false,
  shows: false,
  recent: false
})

const trendingMoviesPage = ref(1)
const trendingShowsPage = ref(1)

// 计算属性 (currently unused but may be needed for future features)
// const currentItems = computed(() => {
//   switch (activeTab.value) {
//     case 'trending': return trendingMovies.value
//     case 'movies': return recommendedMovies.value
//     case 'shows': return recommendedShows.value
//     case 'recent': return recentItems.value
//     default: return []
//   }
// })

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
  router.push({
    name: `${type}-detail`,
    params: { id: item.ids?.slug || item.ids?.trakt }
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
        await loadMoviesData()
      }
      break
    case 'shows':
      if (!dataLoaded.value.shows) {
        await loadShowsData()
      }
      break
    case 'recent':
      if (!dataLoaded.value.recent) {
        await loadRecentData()
      }
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
  } catch (error) {
    console.error('加载轮播数据失败:', error)
  } finally {
    loading.value.featured = false
  }
}

const loadMoviesData = async () => {
  if (loading.value.movies || dataLoaded.value.movies) return
  
  loading.value.movies = true
  try {
    // 调用API获取推荐电影
    const movies = await invoke<MoviesRecommendResponse>("movies_recommand")
    recommendedMovies.value = movies
    dataLoaded.value.movies = true
    console.log('加载推荐电影')
    
    // 在后台预加载翻译
    preloadMovieTranslations(movies, (loaded, total) => {
      console.log(`翻译加载进度: ${loaded}/${total}`)
    })
  } catch (error) {
    console.error('加载推荐电影失败:', error)
  } finally {
    loading.value.movies = false
  }
}

const loadShowsData = async () => {
  if (loading.value.shows || dataLoaded.value.shows) return
  
  loading.value.shows = true
  try {
    recommendedShows.value = await invoke<ShowsRecommendResponse>("shows_recommand")
    dataLoaded.value.shows = true
  } catch (error) {
    console.error('加载推荐剧集失败:', error)
  } finally {
    loading.value.shows = false
  }
}

const loadRecentData = async () => {
  if (loading.value.recent || dataLoaded.value.recent) return
  
  loading.value.recent = true
  try {
    // 调用API获取最新发布
    console.log('加载最新发布')
    recentItems.value = []
    dataLoaded.value.recent = true
  } catch (error) {
    console.error('加载最新发布失败:', error)
  } finally {
    loading.value.recent = false
  }
}

// 加载更多数据的方法
const loadMoreTrendingMovies = async () => {
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
      trendingMovies.value.push(...movies)
      
      // 在后台预加载翻译
      preloadMovieTranslations(movies, (loaded, total) => {
        console.log(`更多热门电影翻译加载进度: ${loaded}/${total}`)
      })
    }
    trendingMoviesPage.value++
  } catch (error) {
    console.error('加载更多热门电影失败:', error)
  } finally {
    loading.value.trendingMovies = false
  }
}

const loadMoreTrendingShows = async () => {
  loading.value.trendingShows = true
  try {
    const res = await invoke<ShowTrendingResponse>("show_trending_page", { page: trendingShowsPage.value, limit: 40 })
    if (res) {
      for (const item of res) {
        if (item.show) {
          item.show.watchers = item.watchers
          trendingShows.value.push(item.show)
        }
      }
    }
    trendingShowsPage.value++
  } catch (error) {
    console.error('加载更多热门电视剧失败:', error)
  } finally {
    loading.value.trendingShows = false
  }
}

const loadMoreRecent = () => {
  console.log('加载更多最新发布')
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

const loadTrendingMoviesData = async () => {
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
      trendingMovies.value.push(...movies)
      dataLoaded.value.trendingMovies = true
      
      // 在后台预加载翻译
      preloadMovieTranslations(movies, (loaded, total) => {
        console.log(`热门电影翻译加载进度: ${loaded}/${total}`)
      })
    }
    trendingMoviesPage.value++
  } catch (error) {
    console.error('加载热门电影失败:', error)
  } finally {
    loading.value.trendingMovies = false
  }
}

const loadTrendingShowsData = async () => {
  if (loading.value.trendingShows || dataLoaded.value.trendingShows) return
  
  loading.value.trendingShows = true
  try {
    // 调用API获取热门电视剧
    const res = await invoke<ShowTrendingResponse>("show_trending")
    if (res && res.length > 0) {
      for (const item of res) {
        if (item.show) {
          item.show.watchers = item.watchers
          trendingShows.value.push(item.show)
        }
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
}

.hero-section {
  margin-bottom: 40px;
  border-radius: 16px;
  overflow: hidden;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
}

.hero-slide {
  height: 400px;
  position: relative;
  display: flex;
  align-items: center;
  color: white;
}

.hero-content {
  width: 100%;
  padding: 0 60px;
}

.hero-info {
  max-width: 600px;
}

.hero-title {
  font-size: 48px;
  font-weight: 700;
  margin: 0 0 16px 0;
  text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.8);
}

.hero-overview {
  font-size: 16px;
  line-height: 1.6;
  margin: 0 0 20px 0;
  opacity: 0.9;
  text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.8);
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.hero-meta {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-bottom: 24px;
  font-size: 14px;
}

.hero-year,
.hero-rating,
.hero-genre {
  display: flex;
  align-items: center;
  gap: 4px;
  background: rgba(255, 255, 255, 0.2);
  padding: 6px 12px;
  border-radius: 20px;
  backdrop-filter: blur(10px);
}

.hero-actions {
  display: flex;
  gap: 12px;
}

.category-tabs {
  margin-top: 40px;
}

.trending-sub-tabs {
  margin-top: 16px;
}

.trending-sub-tabs .arco-tabs-nav {
  margin-bottom: 20px;
}

.trending-sub-tabs .arco-tabs-tab {
  font-size: 14px;
  font-weight: 500;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .hero-slide {
    height: 300px;
  }
  
  .hero-content {
    padding: 0 24px;
  }
  
  .hero-title {
    font-size: 32px;
  }
  
  .hero-overview {
    font-size: 14px;
    -webkit-line-clamp: 2;
  }
  
  .hero-meta {
    flex-wrap: wrap;
    gap: 8px;
  }
  
  .hero-actions {
    flex-direction: column;
    align-items: flex-start;
  }
  
  .hero-actions .a-btn {
    width: fit-content;
  }
}

@media (max-width: 480px) {
  .hero-title {
    font-size: 24px;
  }
  
  .hero-actions {
    gap: 8px;
  }
}
</style> 