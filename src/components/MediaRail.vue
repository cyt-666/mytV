<template>
  <div class="media-rail">
    <div class="rail-header" v-if="title">
      <h2 class="rail-title" @click="emitClickTitle">
        <!-- 新增图标插槽 -->
        <span class="rail-icon" v-if="$slots.icon">
          <slot name="icon" />
        </span>
        {{ title }}
        <IconRight v-if="hasMore" class="arrow-icon" />
      </h2>
    </div>
    
    <div class="rail-container" ref="railRef">
      <div class="rail-scroll">
        <div 
          v-for="item in items" 
          :key="item.ids?.trakt" 
          class="rail-item"
        >
          <MediaCard 
            :media="item" 
            :type="mediaType"
            :show-meta="false"
          />
        </div>
        
        <!-- 骨架屏 -->
        <template v-if="loading">
          <div v-for="n in 6" :key="`skeleton-${n}`" class="rail-item">
            <a-skeleton animation>
              <a-skeleton-shape class="skeleton-poster" />
              <a-space direction="vertical" size="mini" class="skeleton-text">
                <a-skeleton-line :rows="1" :widths="['80%']" />
                <a-skeleton-line :rows="1" :widths="['40%']" />
              </a-space>
            </a-skeleton>
          </div>
        </template>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { IconRight } from '@arco-design/web-vue/es/icon'
import MediaCard from './MediaCard.vue'
import type { Movie, Show } from '../types/api'

interface Props {
  title?: string
  items: (Movie | Show)[]
  mediaType: 'movie' | 'show'
  loading?: boolean
  hasMore?: boolean
}

// 移除未使用的变量声明
withDefaults(defineProps<Props>(), {
  title: '',
  items: () => [],
  loading: false,
  hasMore: false
})

const emit = defineEmits(['click-title'])
const railRef = ref<HTMLElement | null>(null)

const emitClickTitle = () => {
  emit('click-title')
}
</script>

<style scoped>
.media-rail {
  margin-bottom: 40px;
}

.rail-header {
  margin-bottom: 16px;
  padding: 0 4px;
}

.rail-title {
  font-size: 20px;
  font-weight: 700;
  color: #1d1d1f;
  margin: 0;
  display: inline-flex;
  align-items: center;
  cursor: pointer;
  transition: opacity 0.2s;
}

.rail-title:hover {
  opacity: 0.7;
}

.rail-icon {
  margin-right: 8px;
  display: flex;
  align-items: center;
  color: #165dff;
  font-size: 20px;
}

.arrow-icon {
  font-size: 16px;
  margin-left: 8px;
  color: #86909c;
}

.rail-container {
  position: relative;
  margin: 0 -40px;
  padding: 0 40px;
  overflow: hidden;
}

.rail-scroll {
  display: flex;
  gap: 20px;
  overflow-x: auto;
  padding-bottom: 20px; 
  margin-bottom: -20px;
  scroll-behavior: smooth;
  -webkit-overflow-scrolling: touch;
}

.rail-item {
  flex: 0 0 auto;
  width: 160px;
}

.rail-item :deep(.media-card) {
  width: 100%;
}

.rail-item :deep(.poster-container) {
  aspect-ratio: 2/3;
  width: 100%;
}

.skeleton-poster {
  width: 160px;
  height: 240px;
  border-radius: 12px;
  margin-bottom: 12px;
}

.skeleton-text {
  width: 160px;
}
</style>
