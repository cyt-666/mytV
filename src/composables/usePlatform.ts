import { ref } from 'vue'

export function usePlatform() {
  const userAgent = window.navigator.userAgent
  const isMacOS = ref(userAgent.indexOf('Mac OS X') !== -1)
  const isWindows = ref(userAgent.indexOf('Windows') !== -1)

  return {
    isMacOS,
    isWindows
  }
}
