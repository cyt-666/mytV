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

// 具体业务的事件监听器
export interface MediaUpdatePayload {
  type: 'movie' | 'show' | 'seasons';
  id: number;
  data: any;
}

export function useMediaUpdate(callback: (payload: MediaUpdatePayload) => void) {
  useEvent<MediaUpdatePayload>('media-update', callback)
}
