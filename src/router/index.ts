import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'home',
    component: () => import('../views/HomeView.vue'),
    meta: { title: '首页' }
  },
  {
    path: '/search',
    name: 'search',
    component: () => import('../views/SearchView.vue'),
    meta: { title: '搜索' }
  },
  {
    path: '/movie/:id',
    name: 'movie-detail',
    component: () => import('../views/MovieDetailView.vue'),
    meta: { title: '电影详情' }
  },
  {
    path: '/show/:id',
    name: 'show-detail',
    component: () => import('../views/ShowDetailView.vue'),
    meta: { title: '电视剧详情' }
  },
  {
    path: '/show/:id/season/:season',
    name: 'season-detail',
    component: () => import('../views/SeasonDetailView.vue'),
    meta: { title: '季度详情' }
  },
  {
    path: '/show/:id/season/:season/episode/:episode',
    name: 'episode-detail',
    component: () => import('../views/EpisodeDetailView.vue'),
    meta: { title: '剧集详情' }
  },
  {
    path: '/watchlist',
    name: 'watchlist',
    component: () => import('../views/WatchlistView.vue'),
    meta: { title: '我的清单' }
  },
  {
    path: '/collection',
    name: 'collection',
    component: () => import('../views/CollectionView.vue'),
    meta: { title: '我的收藏' }
  },
  {
    path: '/history',
    name: 'history',
    component: () => import('../views/HistoryView.vue'),
    meta: { title: '观看历史' }
  },
  {
    path: '/profile',
    name: 'profile',
    component: () => import('../views/ProfileView.vue'),
    meta: { title: '个人中心' }
  },
  {
    path: '/up-next',
    name: 'up-next',
    component: () => import('../views/UpNextView.vue'),
    meta: { title: '待看' }
  },
  {
    path: '/calendar',
    name: 'calendar',
    component: () => import('../views/CalendarView.vue'),
    meta: { title: '剧集日历' }
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

router.beforeEach((to) => {
  document.title = `${to.meta.title} - MyTV`
})

export default router 