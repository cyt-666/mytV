import { invoke } from "@tauri-apps/api/core"
import type { Movie, Show, TranslationData } from '../types/api'

// 翻译结果类型
export interface TranslationResult {
  title?: string;
  overview?: string;
  tagline?: string;
}

// 内存缓存，用于减少重复请求
const memoryCache = new Map<string, TranslationResult | null>()
const pendingRequests = new Map<string, Promise<TranslationResult | null>>()

// 全局并发控制
class TranslationConcurrencyManager {
  private maxConcurrent: number = 3 // 最大并发数
  private currentRequests: number = 0
  private requestQueue: Array<() => void> = []

  async executeRequest<T>(requestFn: () => Promise<T>): Promise<T> {
    return new Promise((resolve, reject) => {
      const executeNow = () => {
        this.currentRequests++
        requestFn()
          .then(resolve)
          .catch(reject)
          .finally(() => {
            this.currentRequests--
            this.processQueue()
          })
      }

      if (this.currentRequests < this.maxConcurrent) {
        executeNow()
      } else {
        this.requestQueue.push(executeNow)
      }
    })
  }

  private processQueue() {
    if (this.requestQueue.length > 0 && this.currentRequests < this.maxConcurrent) {
      const nextRequest = this.requestQueue.shift()!
      nextRequest()
    }
  }

  getStats() {
    return {
      currentRequests: this.currentRequests,
      queueLength: this.requestQueue.length
    }
  }
}

const concurrencyManager = new TranslationConcurrencyManager()

/**
 * 获取电影的中文翻译信息（带持久化缓存）
 * @param movieId 电影的trakt ID
 * @returns 中文翻译信息，如果没有找到则返回null
 */
export async function getMovieChineseTranslation(movieId: number): Promise<TranslationResult | null> {
  const cacheKey = `movie_${movieId}`
  
  console.log('获取电影翻译:', movieId, '缓存key:', cacheKey)
  
  // 检查内存缓存
  if (memoryCache.has(cacheKey)) {
    console.log('从内存缓存返回电影翻译:', memoryCache.get(cacheKey))
    return memoryCache.get(cacheKey)!
  }

  // 检查是否已有相同请求在进行中
  if (pendingRequests.has(cacheKey)) {
    console.log('等待现有电影翻译请求完成')
    try {
      // 使用Promise.race添加超时，避免长时间阻塞
      return await Promise.race([
        pendingRequests.get(cacheKey)!,
        new Promise<TranslationResult | null>((_, reject) => 
          setTimeout(() => reject(new Error('Pending request timeout')), 5000)
        )
      ])
    } catch (error) {
      console.warn('等待现有请求超时，返回null:', error)
      return null
    }
  }

  try {
    console.log('发起电影翻译API请求:', movieId)
    
    // 使用并发管理器执行请求
    const requestPromise = concurrencyManager.executeRequest(async () => {
      return Promise.race([
        invoke<TranslationData | null>("get_movie_translation_cached", { id: movieId }),
        // 8秒超时
        new Promise<never>((_, reject) => 
          setTimeout(() => reject(new Error('Translation request timeout')), 8000)
        )
      ])
    }).then(translationData => {
      console.log('电影翻译API响应:', translationData)
      
      const result: TranslationResult | null = translationData ? {
        title: translationData.title,
        overview: translationData.overview,
        tagline: translationData.tagline
      } : null
      
      // 缓存结果到内存
      memoryCache.set(cacheKey, result)
      console.log('电影翻译已缓存到内存:', result)
      return result
    }).catch(error => {
      console.warn(`获取电影 ${movieId} 的中文翻译失败:`, error)
      // 短期缓存失败结果，避免频繁重试
      memoryCache.set(cacheKey, null)
      // 5分钟后清除失败缓存，允许重试
      setTimeout(() => {
        if (memoryCache.get(cacheKey) === null) {
          memoryCache.delete(cacheKey)
        }
      }, 5 * 60 * 1000)
      return null
    }).finally(() => {
      // 清理pending状态
      pendingRequests.delete(cacheKey)
    })

    // 记录pending状态
    pendingRequests.set(cacheKey, requestPromise)
    
    return await requestPromise
  } catch (error) {
    console.warn(`获取电影 ${movieId} 的中文翻译失败:`, error)
    memoryCache.set(cacheKey, null)
    return null
  }
}

/**
 * 获取电视剧的中文翻译信息（带持久化缓存）
 * @param showId 电视剧的trakt ID
 * @returns 中文翻译信息，如果没有找到则返回null
 */
export async function getShowChineseTranslation(showId: number): Promise<TranslationResult | null> {
  const cacheKey = `show_${showId}`
  
  console.log('获取电视剧翻译:', showId, '缓存key:', cacheKey)
  
  // 检查内存缓存
  if (memoryCache.has(cacheKey)) {
    console.log('从内存缓存返回电视剧翻译:', memoryCache.get(cacheKey))
    return memoryCache.get(cacheKey)!
  }

  // 检查是否已有相同请求在进行中
  if (pendingRequests.has(cacheKey)) {
    console.log('等待现有电视剧翻译请求完成')
    try {
      // 使用Promise.race添加超时，避免长时间阻塞
      return await Promise.race([
        pendingRequests.get(cacheKey)!,
        new Promise<TranslationResult | null>((_, reject) => 
          setTimeout(() => reject(new Error('Pending request timeout')), 5000)
        )
      ])
    } catch (error) {
      console.warn('等待现有请求超时，返回null:', error)
      return null
    }
  }

  try {
    console.log('发起电视剧翻译API请求:', showId)
    
    // 使用并发管理器执行请求
    const requestPromise = concurrencyManager.executeRequest(async () => {
      return Promise.race([
        invoke<TranslationData | null>("get_show_translation_cached", { id: showId }),
        // 8秒超时
        new Promise<never>((_, reject) => 
          setTimeout(() => reject(new Error('Translation request timeout')), 8000)
        )
      ])
    }).then(translationData => {
      console.log('电视剧翻译API响应:', translationData)
      
      const result: TranslationResult | null = translationData ? {
        title: translationData.title,
        overview: translationData.overview,
        tagline: translationData.tagline
      } : null
      
      // 缓存结果到内存
      memoryCache.set(cacheKey, result)
      console.log('电视剧翻译已缓存到内存:', result)
      return result
    }).catch(error => {
      console.warn(`获取电视剧 ${showId} 的中文翻译失败:`, error)
      // 短期缓存失败结果，避免频繁重试
      memoryCache.set(cacheKey, null)
      // 5分钟后清除失败缓存，允许重试
      setTimeout(() => {
        if (memoryCache.get(cacheKey) === null) {
          memoryCache.delete(cacheKey)
        }
      }, 5 * 60 * 1000)
      return null
    }).finally(() => {
      // 清理pending状态
      pendingRequests.delete(cacheKey)
    })

    // 记录pending状态
    pendingRequests.set(cacheKey, requestPromise)
    
    return await requestPromise
  } catch (error) {
    console.warn(`获取电视剧 ${showId} 的中文翻译失败:`, error)
    memoryCache.set(cacheKey, null)
    return null
  }
}

/**
 * 获取季度的中文翻译信息（带持久化缓存）
 * @param showId 电视剧的trakt ID
 * @param seasonNumber 季度编号
 * @returns 中文翻译信息，如果没有找到则返回null
 */
export async function getSeasonChineseTranslation(showId: number, seasonNumber: number): Promise<TranslationResult | null> {
  const cacheKey = `season_${showId}_${seasonNumber}`
  
  console.log('获取季度翻译:', showId, '季度:', seasonNumber, '缓存key:', cacheKey)
  
  // 检查内存缓存
  if (memoryCache.has(cacheKey)) {
    console.log('从内存缓存返回季度翻译:', memoryCache.get(cacheKey))
    return memoryCache.get(cacheKey)!
  }

  // 检查是否已有相同请求在进行中
  if (pendingRequests.has(cacheKey)) {
    console.log('等待现有季度翻译请求完成')
    try {
      // 使用Promise.race添加超时，避免长时间阻塞
      return await Promise.race([
        pendingRequests.get(cacheKey)!,
        new Promise<TranslationResult | null>((_, reject) => 
          setTimeout(() => reject(new Error('Pending request timeout')), 5000)
        )
      ])
    } catch (error) {
      console.warn('等待现有请求超时，返回null:', error)
      return null
    }
  }

  try {
    console.log('发起季度翻译API请求:', showId, '季度:', seasonNumber)
    
    // 使用并发管理器执行请求
    const requestPromise = concurrencyManager.executeRequest(async () => {
      return Promise.race([
        invoke<TranslationData | null>("get_season_translation_cached", { 
          showId: showId, 
          season: seasonNumber 
        }),
        // 8秒超时
        new Promise<never>((_, reject) => 
          setTimeout(() => reject(new Error('Translation request timeout')), 8000)
        )
      ])
    }).then(translationData => {
      console.log('季度翻译API响应:', translationData)
      
      const result: TranslationResult | null = translationData ? {
        title: translationData.title,
        overview: translationData.overview,
        tagline: translationData.tagline
      } : null
      
      // 缓存结果到内存
      memoryCache.set(cacheKey, result)
      console.log('季度翻译已缓存到内存:', result)
      return result
    }).catch(error => {
      console.warn(`获取季度 ${showId}-${seasonNumber} 的中文翻译失败:`, error)
      // 短期缓存失败结果，避免频繁重试
      memoryCache.set(cacheKey, null)
      // 5分钟后清除失败缓存，允许重试
      setTimeout(() => {
        if (memoryCache.get(cacheKey) === null) {
          memoryCache.delete(cacheKey)
        }
      }, 5 * 60 * 1000)
      return null
    }).finally(() => {
      // 清理pending状态
      pendingRequests.delete(cacheKey)
    })

    // 记录pending状态
    pendingRequests.set(cacheKey, requestPromise)
    
    return await requestPromise
  } catch (error) {
    console.warn(`获取季度 ${showId}-${seasonNumber} 的中文翻译失败:`, error)
    memoryCache.set(cacheKey, null)
    return null
  }
}

/**
 * 清理过期的翻译缓存
 */
export async function clearExpiredTranslations(): Promise<number> {
  try {
    const clearedCount = await invoke<number>("clear_expired_translations")
    console.log(`已清理 ${clearedCount} 个过期翻译缓存`)
    return clearedCount
  } catch (error) {
    console.warn('清理过期翻译缓存失败:', error)
    return 0
  }
}

/**
 * 获取翻译缓存统计信息
 */
export async function getTranslationCacheStats(): Promise<{ total: number; expired: number }> {
  try {
    const [total, expired] = await invoke<[number, number]>("get_translation_cache_stats")
    return { total, expired }
  } catch (error) {
    console.warn('获取翻译缓存统计失败:', error)
    return { total: 0, expired: 0 }
  }
}

/**
 * 获取翻译请求的并发状态
 */
export function getTranslationConcurrencyStats() {
  return {
    ...concurrencyManager.getStats(),
    memoryCacheSize: memoryCache.size,
    pendingRequestsCount: pendingRequests.size
  }
}

/**
 * 设置翻译请求的最大并发数
 */
export function setTranslationMaxConcurrency(maxConcurrent: number) {
  // 通过反射设置私有属性（仅用于调试）
  (concurrencyManager as any).maxConcurrent = Math.max(1, Math.min(10, maxConcurrent))
  console.log('翻译并发数已设置为:', maxConcurrent)
}

/**
 * 为电影异步加载翻译（不阻塞渲染）
 * @param movie 电影对象
 * @param onTranslationLoaded 翻译加载完成的回调
 */
export async function loadMovieTranslationAsync(
  movie: Movie,
  onTranslationLoaded: (translation: TranslationResult | null) => void
) {
  if (!movie.ids?.trakt) return
  
  const cacheKey = `movie_${movie.ids.trakt}`
  
  // 立即检查缓存
  if (memoryCache.has(cacheKey)) {
    const cached = memoryCache.get(cacheKey) ?? null
    onTranslationLoaded(cached)
    return
  }
  
  // 异步加载翻译
  setTimeout(async () => {
    const translation = await getMovieChineseTranslation(movie.ids!.trakt)
    onTranslationLoaded(translation)
  }, 0)
}

/**
 * 为电视剧异步加载翻译（不阻塞渲染）
 * @param show 电视剧对象
 * @param onTranslationLoaded 翻译加载完成的回调
 */
export async function loadShowTranslationAsync(
  show: Show,
  onTranslationLoaded: (translation: TranslationResult | null) => void
) {
  if (!show.ids?.trakt) return
  
  const cacheKey = `show_${show.ids.trakt}`
  
  // 立即检查缓存
  if (memoryCache.has(cacheKey)) {
    const cached = memoryCache.get(cacheKey) ?? null
    onTranslationLoaded(cached)
    return
  }
  
  // 异步加载翻译
  setTimeout(async () => {
    const translation = await getShowChineseTranslation(show.ids!.trakt)
    onTranslationLoaded(translation)
  }, 0)
}

/**
 * 清理翻译缓存
 */
export function clearTranslationCache() {
  memoryCache.clear()
  pendingRequests.clear()
}

/**
 * 为电影列表预加载翻译（渐进式加载）
 * @param movies 电影列表
 * @param onProgress 进度回调
 * @param maxConcurrent 最大并发数
 */
export async function preloadMovieTranslations(
  movies: Movie[],
  onProgress?: (loaded: number, total: number) => void,
  maxConcurrent: number = 3
) {
  const moviesToLoad = movies.filter(m => m.ids?.trakt && !memoryCache.has(`movie_${m.ids.trakt}`))
  let loaded = 0
  
  for (let i = 0; i < moviesToLoad.length; i += maxConcurrent) {
    const batch = moviesToLoad.slice(i, i + maxConcurrent)
    
    await Promise.all(
      batch.map(async movie => {
        if (movie.ids?.trakt) {
          await getMovieChineseTranslation(movie.ids.trakt)
          loaded++
          onProgress?.(loaded, moviesToLoad.length)
        }
      })
    )
    
    // 批次间添加小延迟
    if (i + maxConcurrent < moviesToLoad.length) {
      await new Promise(resolve => setTimeout(resolve, 200))
    }
  }
} 
/**
 * 为季度异步加载翻译（不阻塞渲染）
 * @param showId 电视剧ID
 * @param seasonNumber 季度编号
 * @param onTranslationLoaded 翻译加载完成的回调
 */
export async function loadSeasonTranslationAsync(
  showId: number,
  seasonNumber: number,
  onTranslationLoaded: (translation: TranslationResult | null) => void
) {
  if (!showId) return
  
  const cacheKey = `season_${showId}_${seasonNumber}`
  
  // 立即检查缓存
  if (memoryCache.has(cacheKey)) {
    const cached = memoryCache.get(cacheKey) ?? null
    onTranslationLoaded(cached)
    return
  }
  
  // 异步加载翻译
  setTimeout(async () => {
    const translation = await getSeasonChineseTranslation(showId, seasonNumber)
    onTranslationLoaded(translation)
  }, 0)
}
