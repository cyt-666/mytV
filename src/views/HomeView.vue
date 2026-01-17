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
                  <h1 class="hero-title">{{ featuredTranslations[item.ids?.trakt]?.title || item.title }}</h1>
                  <p class="hero-overview">{{ featuredTranslations[item.ids?.trakt]?.overview || item.overview }}</p>
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
import { preloadMovieTranslations, getMovieChineseTranslation, type TranslationResult } from '../utils/translation'
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
// 存储轮播图的翻译数据
const featuredTranslations = ref<Record<number, TranslationResult>>({})

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

/* ... */

.hero-content {
  position: relative;
  z-index: 2;
  width: 100%;
  padding: 0 60px 48px 60px; /* 减小底部 padding，防止按钮贴底被切 */
  max-width: 1200px;
}

.hero-slide {
  height: 500px; /* 增加高度 */
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
.trending-sub-tabs {
  margin-top: 24px;
}
.trending-sub-tabs :deep(.arco-tabs-nav-type-card .arco-tabs-tab) {
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
.trending-sub-tabs :deep(.arco-tabs-nav-type-card .arco-tabs-tab-active) {
  background-color: #1d1d1f;
  color: #fff;
  border-color: #1d1d1f;
}

/* 响应式 */
@media (max-width: 768px) {
  .hero-slide { height: 400px; }
  .hero-content { padding: 0 24px 40px 24px; }
  .hero-title { font-size: 32px; }
  .hero-overview { font-size: 14px; }
  .category-tabs { padding: 0 20px; }
}
</style>