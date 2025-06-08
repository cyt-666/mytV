import { inject } from 'vue'
import type { Ref } from 'vue'
import type { User } from '../types/api'

export function useAuth() {
  const userInfo = inject<Ref<User | null>>('userInfo')
  const isLoggedIn = inject<Ref<boolean>>('isLoggedIn')
  const refreshUserInfo = inject<() => Promise<void>>('refreshUserInfo')

  if (!userInfo || !isLoggedIn || !refreshUserInfo) {
    throw new Error('useAuth必须在AppLayout的子组件中使用')
  }

  return {
    userInfo,
    isLoggedIn,
    refreshUserInfo
  }
} 