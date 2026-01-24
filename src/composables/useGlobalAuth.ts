import { ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { Message } from '@arco-design/web-vue'
import type { User } from '../types/api'
import { usePlatform } from './usePlatform'

export function useGlobalAuth() {
  const isLoggedIn = ref(false)
  const userInfo = ref<User | null>(null)
  const avatarUrl = ref<string | null>(null)
  const { isMacOS } = usePlatform()

  const loadUserProfile = async () => {
    try {
      const profile = await invoke('get_user_profile')
      if (profile) userInfo.value = (profile as any).user
    } catch (error) { console.error(error) }
  }

  const checkLoginStatus = async () => {
    try {
      const token = await invoke<boolean>('check_login_status')
      if (token) {
        isLoggedIn.value = true
        await loadUserProfile()
      }
    } catch (error) { console.error(error) }
  }

  const login = async () => {
    try { await invoke('start_trakt_user_auth') }
    catch (e: any) { 
        if (e === 200) { isLoggedIn.value = true; await loadUserProfile() } 
    }
  }

  const logout = async () => {
    try { await invoke('revoke_token'); isLoggedIn.value = false; userInfo.value = null }
    catch (e) { isLoggedIn.value = false; userInfo.value = null }
  }

  const setupOAuthListener = () => {
      listen<string>("oauth-callback", async (event) => {
        if (event.payload) {
          try {
            await invoke("get_token", { code: event.payload })
            Message.success('登录成功，正在同步数据...')
            await loadUserProfile()
            if (!userInfo.value) {
              await new Promise(r => setTimeout(r, 1000))
              await loadUserProfile()
            }
            if (userInfo.value) {
              isLoggedIn.value = true
              Message.success(`欢迎回来, ${userInfo.value.username}`)
            } else {
              isLoggedIn.value = true
            }
          } catch (e) {
            console.error(e)
            Message.error('登录失败，请重试')
          }
        }
      })
  }
  
  watch(() => userInfo.value?.images?.avatar?.full, async (url) => {
    if (!url) {
      avatarUrl.value = null
      return
    }
    
    if (isMacOS.value) {
      try {
        const proxied = await invoke<string>('get_proxied_image', { url })
        avatarUrl.value = proxied
        return
      } catch (e) {
        console.warn('Avatar proxy failed, falling back to direct URL', e)
      }
    }
    
    avatarUrl.value = url.startsWith('http') ? url.replace(/^http:/, 'https:') : `https://${url}`
  }, { immediate: true })

  return {
    isLoggedIn,
    userInfo,
    avatarUrl,
    loadUserProfile,
    checkLoginStatus,
    login,
    logout,
    setupOAuthListener
  }
}
