<template>
  <div class="home-view">
    <div class="page-container">
      <!-- Hero 轮播 -->
      <section class="hero-section" v-if="featuredMovies.length > 0">
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
                      <IconStarFill />
                      {{ item.rating.toFixed(1) }}
                    </span>
                  </div>
                  <div class="hero-actions">
                    <a-button type="primary" size="large" @click="viewDetails(item, 'movie')">
                      <template #icon><IconPlayArrow /></template>
                      查看详情
                    </a-button>
                  </div>
                </div>
              </div>
            </div>
          </a-carousel-item>
        </a-carousel>
      </section>

      <!-- 待看列表 -->
      <MediaRail 
        v-if="isLoggedIn && upNextItems.length > 0"
        title="待看清单"
        :items="upNextItems"
        media-type="show"
        @click-title="$router.push('/up-next')"
        has-more
      >
        <template #icon><IconPlayCircle /></template>
      </MediaRail>

      <!-- 热门电影 -->
      <MediaRail 
        title="热门电影"
        :items="trendingMovies"
        media-type="movie"
        :loading="loading.trendingMovies"
        @click-title="$router.push('/movies?tab=trending')"
        has-more
      >
        <template #icon><IconFire /></template>
      </MediaRail>

      <!-- 热门剧集 -->
      <MediaRail 
        title="热门剧集"
        :items="trendingShows"
        media-type="show"
        :loading="loading.trendingShows"
        @click-title="$router.push('/shows?tab=trending')"
        has-more
      >
        <template #icon><IconThunderbolt /></template>
      </MediaRail>

      <!-- 推荐电影 -->
      <MediaRail 
        title="推荐电影"
        :items="recommendedMovies"
        media-type="movie"
        :loading="loading.recommended"
        @click-title="handleRecommendedMoviesClick"
        has-more
      >
        <template #icon><IconThumbUp /></template>
      </MediaRail>

      <!-- 推荐剧集 -->
      <MediaRail 
        title="推荐剧集"
        :items="recommendedShows"
        media-type="show"
        :loading="loading.recommendedShows"
        @click-title="handleRecommendedShowsClick"
        has-more
      >
        <template #icon><IconHeart /></template>
      </MediaRail>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, inject } from 'vue'
import { useRouter } from 'vue-router'
import { 
  IconStarFill, IconPlayArrow, 
  IconPlayCircle, IconFire, IconThunderbolt, IconThumbUp, IconHeart 
} from '@arco-design/web-vue/es/icon'
import MediaRail from '../components/MediaRail.vue'
import { invoke } from "@tauri-apps/api/core"
import { preloadMovieTranslations, getMovieChineseTranslation, type TranslationResult } from '../utils/translation'
import type { Movie, Show, MovieTrendingResponse, ShowTrendingResponse } from '../types/api'

const router = useRouter()
const isLoggedIn = inject('isLoggedIn', ref(false))

const featuredMovies = ref<Movie[]>([])
const featuredTranslations = ref<Record<number, TranslationResult>>({})
const upNextItems = ref<Show[]>([])
const trendingMovies = ref<Movie[]>([])
const trendingShows = ref<Show[]>([])
const recommendedMovies = ref<Movie[]>([])
const recommendedShows = ref<Show[]>([])

const loading = ref({
  trendingMovies: true,
  trendingShows: true,
  recommended: true,
  recommendedShows: true
})

const loadFeatured = async () => {
  try {
    const res = await invoke<MovieTrendingResponse>('movie_trending')
    const movies = res.slice(0, 5).map((i: any) => i.movie as Movie)
    featuredMovies.value = movies
    
    movies.forEach(async (movie) => {
      if (movie.ids?.trakt) {
        const trans = await getMovieChineseTranslation(movie.ids.trakt)
        if (trans) featuredTranslations.value[movie.ids.trakt] = trans
      }
    })
  } catch (e) {
    console.error('Failed to load featured:', e)
  }
}

const loadUpNext = async () => {
  if (!isLoggedIn.value) return
  try {
    const res = await invoke<any>('get_up_next', { 
      username: 'me',
      page: 1, 
      limit: 10 
    })
    upNextItems.value = res.map((item: any) => ({
      ...item.show,
      next_episode: item.next_episode
    }))
  } catch (e) {
    console.warn('Load up next failed', e)
  }
}

const loadSections = async () => {
  invoke<MovieTrendingResponse>('movie_trending_page', { 
    page: 1, 
    limit: 15,
    genres: null,
    countries: null
  }).then(res => {
    trendingMovies.value = res.map((i: any) => i.movie as Movie)
    loading.value.trendingMovies = false
    preloadMovieTranslations(trendingMovies.value, () => {})
  }).catch(e => {
    console.error('Trending movies failed:', e)
    loading.value.trendingMovies = false
  })

  invoke<ShowTrendingResponse>('show_trending_page', { 
    page: 1, 
    limit: 15,
    genres: null,
    countries: null
  }).then(res => {
    trendingShows.value = res.map((i: any) => i.show as Show)
    loading.value.trendingShows = false
  }).catch(e => {
    console.error('Trending shows failed:', e)
    loading.value.trendingShows = false
  })

  if (isLoggedIn.value) {
    invoke<any>('movies_recommand_page', { page: 1, limit: 15 }).then(res => {
      recommendedMovies.value = res as Movie[]
      loading.value.recommended = false
    }).catch(() => fallbackRecommendationMovies())
  } else {
    fallbackRecommendationMovies()
  }

  if (isLoggedIn.value) {
    invoke<any>('shows_recommand_page', { page: 1, limit: 15 }).then(res => {
      recommendedShows.value = res as Show[]
      loading.value.recommendedShows = false
    }).catch(() => fallbackRecommendationShows())
  } else {
    fallbackRecommendationShows()
  }
}

const fallbackRecommendationMovies = () => {
  invoke<Movie[]>('movie_popular_page', { 
    page: 1, 
    limit: 15,
    genres: null,
    countries: null
  }).then(res => {
    recommendedMovies.value = res
    loading.value.recommended = false
    preloadMovieTranslations(res, () => {})
  }).catch(e => {
    console.error('Popular movies failed:', e)
    loading.value.recommended = false
  })
}

const fallbackRecommendationShows = () => {
  invoke<Show[]>('show_popular_page', { 
    page: 1, 
    limit: 15,
    genres: null,
    countries: null
  }).then(res => {
    recommendedShows.value = res
    loading.value.recommendedShows = false
  }).catch(e => {
    console.error('Popular shows failed:', e)
    loading.value.recommendedShows = false
  })
}

const handleRecommendedMoviesClick = () => {
  if (isLoggedIn.value) {
    router.push('/movies?tab=recommended')
  } else {
    router.push('/movies?tab=popular')
  }
}

const handleRecommendedShowsClick = () => {
  if (isLoggedIn.value) {
    router.push('/shows?tab=recommended')
  } else {
    router.push('/shows?tab=popular')
  }
}

const getHeroBackground = (item: Movie) => {
  const fanart = item.images?.fanart?.[0]
  return fanart 
    ? { backgroundImage: `linear-gradient(rgba(0,0,0,0.4), rgba(0,0,0,0.6)), url(https://${fanart})`, backgroundSize: 'cover', backgroundPosition: 'center' }
    : { background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)' }
}

const viewDetails = (item: Movie, type: 'movie') => {
  if (item.ids?.trakt) router.push({ name: `${type}-detail`, params: { id: item.ids.trakt } })
}

onMounted(() => {
  loadFeatured()
  loadUpNext()
  loadSections()
})
</script>

<style scoped>
.home-view { min-height: 100vh; }
:deep(.page-container) { max-width: 1600px; padding: 0 40px 40px 40px; padding-top: 0; }

.hero-section {
  position: relative;
  margin-bottom: 48px;
  border-radius: 24px;
  overflow: hidden;
  box-shadow: 0 20px 40px rgba(0,0,0,0.15);
  z-index: 10;
}
.hero-carousel { height: 500px; }
.hero-slide {
  height: 100%;
  position: relative;
  display: flex;
  align-items: flex-end;
  background-size: cover;
  background-position: center top;
}
.hero-slide::before {
  content: '';
  position: absolute;
  top: 0; left: 0; right: 0; bottom: 0;
  background: linear-gradient(to top, rgba(0,0,0,0.9) 0%, rgba(0,0,0,0.5) 40%, rgba(0,0,0,0) 100%);
  z-index: 1;
}
.hero-content {
  position: relative;
  z-index: 2;
  width: 100%;
  padding: 0 60px 60px 60px;
  max-width: 1200px;
}
.hero-title {
  font-size: 56px;
  font-weight: 800;
  margin: 0 0 16px 0;
  color: #fff;
  line-height: 1.1;
  text-shadow: 0 4px 12px rgba(0,0,0,0.5);
}
.hero-overview {
  font-size: 18px;
  line-height: 1.6;
  color: rgba(255,255,255,0.9);
  margin: 0 0 24px 0;
  max-width: 700px;
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
.hero-year, .hero-rating {
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

.hero-actions :deep(.arco-btn) {
  height: 48px;
  padding: 0 32px;
  font-size: 16px;
  font-weight: 600;
  border-radius: 24px;
}

@media (max-width: 768px) {
  .hero-slide { height: 400px; }
  .hero-content { padding: 0 24px 40px 24px; }
  .hero-title { font-size: 32px; }
}
</style>
