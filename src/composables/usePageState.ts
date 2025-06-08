import { ref, onMounted } from 'vue'

// 页面状态管理
interface PageState {
  [key: string]: any
}

const pageStates = ref<Record<string, PageState>>({})

export function usePageState(pageKey: string) {
  // 获取状态存储的key
  const getStorageKey = () => `mytv_page_state_${pageKey}`
  
  // 保存状态到sessionStorage
  const saveState = (state: PageState) => {
    try {
      pageStates.value[pageKey] = state
      sessionStorage.setItem(getStorageKey(), JSON.stringify(state))
      console.log(`[状态管理] 保存 ${pageKey} 页面状态:`, state)
    } catch (error) {
      console.warn('保存页面状态失败:', error)
    }
  }
  
  // 从sessionStorage恢复状态
  const restoreState = (): PageState | null => {
    try {
      const stored = sessionStorage.getItem(getStorageKey())
      if (stored) {
        const state = JSON.parse(stored)
        pageStates.value[pageKey] = state
        console.log(`[状态管理] 恢复 ${pageKey} 页面状态:`, state)
        return state
      }
    } catch (error) {
      console.warn('恢复页面状态失败:', error)
    }
    return null
  }
  
  // 清除状态
  const clearState = () => {
    delete pageStates.value[pageKey]
    sessionStorage.removeItem(getStorageKey())
  }
  
  // 获取当前状态
  const getState = (): PageState | null => {
    return pageStates.value[pageKey] || null
  }
  
  // 更新状态
  const setState = (newState: Partial<PageState>) => {
    const currentState = getState() || {}
    const updatedState = { ...currentState, ...newState }
    saveState(updatedState)
  }
  
  // 在页面进入时恢复状态
  onMounted(() => {
    restoreState()
  })
  
  return {
    saveState,
    restoreState,
    clearState,
    getState,
    setState
  }
}

// 专门用于首页状态管理
export function useHomePageState() {
  const { saveState, restoreState, getState, setState } = usePageState('home')
  
  // 保存首页状态
  const saveHomeState = (activeTab: string, trendingSubTab?: string, scrollPosition?: number) => {
    const state = {
      activeTab,
      trendingSubTab,
      scrollPosition: scrollPosition || window.scrollY,
      timestamp: Date.now()
    }
    saveState(state)
  }
  
  // 恢复首页状态
  const restoreHomeState = () => {
    const state = restoreState()
    if (state && state.timestamp) {
      // 5分钟内的状态才恢复
      const fiveMinutes = 5 * 60 * 1000
      if (Date.now() - state.timestamp < fiveMinutes) {
        return {
          activeTab: state.activeTab || 'trending',
          trendingSubTab: state.trendingSubTab || 'movies',
          scrollPosition: state.scrollPosition || 0
        }
      }
    }
    return null
  }
  
  return {
    saveHomeState,
    restoreHomeState,
    setState,
    getState
  }
} 