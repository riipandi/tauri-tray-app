import { getCurrent } from '@tauri-apps/api/window'
import { type OsType, type as getOsType } from '@tauri-apps/plugin-os'
import { type ParentComponent, createSignal, onMount } from 'solid-js'

import CaptionControl from './CaptionControl'
import TrafficLight from './TrafficLight'
import './titlebar.css'

export const CustomTitleBar: ParentComponent = ({ children }) => {
  const [platform, setPlatform] = createSignal<OsType>('macos')
  const [isFocused, setIsFocused] = createSignal<boolean>(true)

  const win = getCurrent()

  const handleMinimize = async () => {
    const isMinimized = await win.isMinimized()
    return isMinimized ? await win.unminimize() : await win.minimize()
  }

  const handleMaximize = async () => {
    const isMaximized = await win.isMaximized()
    return isMaximized ? await win.unmaximize() : await win.maximize()
  }

  const handleFullScreen = async () => {
    const isMaximized = await win.isFullscreen()
    return await win.setFullscreen(!isMaximized)
  }

  const handleClose = async () => await win.close()

  onMount(async () => {
    const osType = await getOsType()
    setPlatform(osType)

    // @ref: https://beta.tauri.app/references/v2/js/core/namespaceevent
    win.listen('tauri://webview-created', () => setIsFocused(true))
    win.listen('tauri://focus', () => setIsFocused(true))
    win.listen('tauri://blur', () => setIsFocused(false))
  })

  return platform() === 'macos' ? (
    <div data-tauri-drag-region class="titlebar-root">
      <TrafficLight
        handleMinimize={handleMinimize}
        handleMaximize={handleFullScreen}
        handleClose={handleClose}
        isFocused={isFocused()}
      />
      <div data-tauri-drag-region class="titlebar-content">
        {children}
      </div>
    </div>
  ) : (
    <div data-tauri-drag-region class="titlebar-root">
      <div data-tauri-drag-region class="titlebar-content">
        {children}
      </div>
      <CaptionControl
        handleMinimize={handleMinimize}
        handleMaximize={handleMaximize}
        handleClose={handleClose}
        isFocused={isFocused()}
      />
    </div>
  )
}

export const DefaultTitleBar = () => {
  return <div class="absolute w-full h-7 bg-transparent z-10" data-tauri-drag-region />
}
