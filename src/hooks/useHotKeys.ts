import { useCallback, useEffect } from 'react'
import { TauriEvent } from '@tauri-apps/api/event'
import { register, unregisterAll } from '@tauri-apps/api/globalShortcut'
import { appWindow } from '@tauri-apps/api/window'

// Custom hook for registering and unregistering hotkeys
const useHotKeys = (handler: () => void, keys: string) => {
  const registerHotKeys = useCallback(() => {
    register(keys, handler).catch(() => {})
  }, [handler, keys])

  useEffect(() => {
    registerHotKeys()
    appWindow.listen(TauriEvent.WINDOW_FOCUS, () => registerHotKeys())
    appWindow.listen(TauriEvent.WINDOW_BLUR, () => unregisterAll())
    return () => {
      unregisterAll()
    }
  }, [registerHotKeys])
}

export default useHotKeys
