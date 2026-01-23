import { listen, type Event, type UnlistenFn } from '@tauri-apps/api/event'
import { onMounted, onUnmounted } from 'vue'

export function useEvent<T>(name: string, callback: (payload: T) => void) {
  let unlisten: UnlistenFn | null = null

  onMounted(async () => {
    unlisten = await listen<T>(name, (event: Event<T>) => {
      callback(event.payload)
    })
  })

  onUnmounted(() => {
    if (unlisten) {
      unlisten()
    }
  })
}

// 媒体详情更新事件
export interface MediaUpdatePayload {
  type: 'movie' | 'show' | 'seasons' | 'season' | 'episode';
  id: number;
  season?: number;
  episode?: number;
  data: any;
}

export function useMediaUpdate(callback: (payload: MediaUpdatePayload) => void) {
  useEvent<MediaUpdatePayload>('media-update', callback)
}

// 用户数据更新事件
export interface UserDataUpdatePayload {
  key: string;
  data: any;
}

export function useUserDataUpdate(callback: (payload: UserDataUpdatePayload) => void) {
  useEvent<UserDataUpdatePayload>('user-data-update', callback)
}
