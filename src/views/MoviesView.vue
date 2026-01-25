<template>
  <div class="movies-view page-container">
    <div class="page-header">
      <h1 class="page-title">电影</h1>
    </div>

    <div class="control-container">
      <a-radio-group
        v-model="activeTab"
        type="button"
        size="large"
        @change="handleTabChange"
        class="custom-segmented-control"
      >
        <a-radio value="trending">
          <span class="radio-label"><IconThunderbolt /> 热门趋势</span>
        </a-radio>
        <a-radio value="popular">
          <span class="radio-label"><IconTrophy /> 历史经典</span>
        </a-radio>
        <a-radio value="recommended" v-if="isLoggedIn">
          <span class="radio-label"><IconThumbUp /> 为你推荐</span>
        </a-radio>
        <a-radio value="anticipated">
          <span class="radio-label"><IconStar /> 最受期待</span>
        </a-radio>
      </a-radio-group>
    </div>

    <div class="content-container">
      <div v-show="activeTab === 'trending'">
        <MediaGrid
          :items="trendingItems"
          :loading="loading.trending"
          :loading-more="loading.trending"
          @load-more="loadMoreTrending"
          media-type="movie"
        />
      </div>

      <div v-show="activeTab === 'popular'">
        <MediaGrid
          :items="popularItems"
          :loading="loading.popular && popularPage === 1"
          :loading-more="loading.popular && popularPage > 1"
          :has-more="true"
          @load-more="loadMorePopular"
          media-type="movie"
        />
      </div>

      <div v-show="activeTab === 'recommended'" v-if="isLoggedIn">
        <MediaGrid
          :items="recommendedItems"
          :loading="loading.recommended && recommendedPage === 1"
          :loading-more="loading.recommended && recommendedPage > 1"
          :has-more="true"
          @load-more="loadMoreRecommended"
          media-type="movie"
        />
      </div>

      <div v-show="activeTab === 'anticipated'">
        <MediaGrid
          :items="anticipatedItems"
          :loading="loading.anticipated && anticipatedPage === 1"
          :loading-more="loading.anticipated && anticipatedPage > 1"
          :has-more="true"
          @load-more="loadMoreAnticipated"
          media-type="movie"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, inject, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { IconThunderbolt, IconTrophy, IconThumbUp, IconStar } from '@arco-design/web-vue/es/icon'
import MediaGrid from '../components/MediaGrid.vue'
import { invoke } from "@tauri-apps/api/core"
import { preloadMovieTranslations } from '../utils/translation'
import type { Movie, MovieTrendingResponse, MovieAnticipated, MoviesRecommendResponse } from '../types/api'

defineOptions({ name: 'MoviesView' })

const route = useRoute()
const router = useRouter()
const isLoggedIn = inject('isLoggedIn', ref(false))

const activeTab = ref('trending')
const loading = ref({ trending: false, popular: false, anticipated: false, recommended: false })

const trendingItems = ref<Movie[]>([])
const trendingPage = ref(1)

const popularItems = ref<Movie[]>([])
const popularPage = ref(1)

const recommendedItems = ref<Movie[]>([])
const recommendedPage = ref(1)

const anticipatedItems = ref<Movie[]>([])
const anticipatedPage = ref(1)

const loadMoreTrending = async () => {
  if (loading.value.trending) return
  loading.value.trending = true
  try {
    const res = await invoke<MovieTrendingResponse>("movie_trending_page", { 
      page: trendingPage.value, 
      limit: 40,
      genres: null,
      countries: null
    })
    const items = res.map((i: any) => ({ ...(i.movie as Movie), watchers: i.watchers }))
    
    const existing = new Set(trendingItems.value.map(i => i.ids?.trakt))
    const newItems = items.filter(i => i.ids?.trakt && !existing.has(i.ids.trakt))
    
    trendingItems.value.push(...newItems)
    trendingPage.value++
    preloadMovieTranslations(newItems, () => {})
  } catch (e) {
    console.error('Load trending failed', e)
  } finally {
    loading.value.trending = false
  }
}

const loadMorePopular = async () => {
  if (loading.value.popular) return
  loading.value.popular = true
  try {
    const res = await invoke<Movie[]>("movie_popular_page", { 
      page: popularPage.value, 
      limit: 40,
      genres: null,
      countries: null
    })
    const existing = new Set(popularItems.value.map(i => i.ids?.trakt))
    const newItems = res.filter(i => i.ids?.trakt && !existing.has(i.ids.trakt))
    
    popularItems.value.push(...newItems)
    popularPage.value++
    preloadMovieTranslations(newItems, () => {})
  } catch (e) {
    console.error('Load popular failed', e)
  } finally {
    loading.value.popular = false
  }
}

const loadMoreRecommended = async () => {
  if (loading.value.recommended) return
  loading.value.recommended = true
  try {
    const res = await invoke<MoviesRecommendResponse>("movies_recommand_page", { 
      page: recommendedPage.value, 
      limit: 40 
    })
    const existing = new Set(recommendedItems.value.map(i => i.ids?.trakt))
    const newItems = res.filter(i => i.ids?.trakt && !existing.has(i.ids.trakt))
    
    recommendedItems.value.push(...newItems)
    recommendedPage.value++
    preloadMovieTranslations(newItems, () => {})
  } catch (e) {
    console.error('Load recommended failed', e)
  } finally {
    loading.value.recommended = false
  }
}

const loadMoreAnticipated = async () => {
  if (loading.value.anticipated) return
  loading.value.anticipated = true
  try {
    const res = await invoke<MovieAnticipated[]>("movie_anticipated", { page: anticipatedPage.value, limit: 40 })
    const items = res.map((i: any) => ({ ...(i.movie as Movie), list_count: i.list_count }))
    
    const existing = new Set(anticipatedItems.value.map(i => i.ids?.trakt))
    const newItems = items.filter(i => i.ids?.trakt && !existing.has(i.ids.trakt))
    
    anticipatedItems.value.push(...newItems)
    anticipatedPage.value++
    preloadMovieTranslations(newItems, () => {})
  } catch (e) {
    console.error('Load anticipated failed', e)
  } finally {
    loading.value.anticipated = false
  }
}

const handleTabChange = (key: string | number) => {
  const k = String(key)
  activeTab.value = k
  router.replace({ query: { ...route.query, tab: k } })

  if (k === 'trending' && trendingItems.value.length === 0) loadMoreTrending()
  if (k === 'popular' && popularItems.value.length === 0) loadMorePopular()
  if (k === 'recommended' && recommendedItems.value.length === 0) loadMoreRecommended()
  if (k === 'anticipated' && anticipatedItems.value.length === 0) loadMoreAnticipated()
}

onMounted(() => {
  const tab = route.query.tab as string
  if (tab && ['trending', 'popular', 'recommended', 'anticipated'].includes(tab)) {
    if (tab === 'recommended' && !isLoggedIn.value) {
      activeTab.value = 'popular'
    } else {
      activeTab.value = tab
    }
  }
  handleTabChange(activeTab.value)
})

watch(isLoggedIn, (newVal) => {
  if (!newVal && activeTab.value === 'recommended') {
    activeTab.value = 'popular'
    handleTabChange('popular')
  }
})
</script>

<style scoped>
.page-header { margin-bottom: 24px; }
.page-title { font-size: 32px; font-weight: 800; color: #1d1d1f; margin: 0; }

.tab-title {
  display: flex;
  align-items: center;
  gap: 8px;
}

.control-container {
  margin-bottom: 24px;
}


:deep(.custom-segmented-control) {
  /* Handled by global macos-theme.css */
}


.radio-label {
  display: flex;
  align-items: center;
  gap: 6px;
  font-weight: 500;
}
</style>
