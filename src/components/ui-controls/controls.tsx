import React, { useCallback, useEffect, useState } from 'react'
import { appWindow } from '@tauri-apps/api/window'

import { Icons } from './icons'
import { cn } from '../../libraries/utils'

export function Windows({ className, ...props }: any) {
  const [isWindowMaximized, setIsWindowMaximized] = useState(false)

  const updateIsWindowMaximized = useCallback(async () => {
    const resolvedPromise = await appWindow.isMaximized()
    setIsWindowMaximized(resolvedPromise)
  }, [])

  useEffect(() => {
    updateIsWindowMaximized()
    let unlisten: () => void = () => {}
    const listen = async () => {
      unlisten = await appWindow.onResized(() => {
        updateIsWindowMaximized()
      })
    }
    listen()
    return () => unlisten && unlisten()
  }, [updateIsWindowMaximized])

  const minimizeWindow = async () => await appWindow.minimize()
  const maximizeWindow = async () => {
    await appWindow.toggleMaximize()
  }
  const closeWindow = async () => await appWindow.close()

  return (
    <div className={cn('inline-flex', className)} {...props}>
      <button
        onClick={minimizeWindow}
        className={cn(
          'items-center justify-center inline-flex',
          'h-[32px] w-[46px] cursor-default rounded-none bg-transparent text-black/90 hover:bg-black/[.05] active:bg-black/[.03]  dark:text-white dark:hover:bg-white/[.06] dark:active:bg-white/[.04]'
        )}
      >
        <Icons.minimizeWin />
      </button>
      <button
        onClick={maximizeWindow}
        className={cn(
          'items-center justify-center inline-flex',
          'h-[32px] w-[46px] cursor-default rounded-none bg-transparent text-black/90 hover:bg-black/[.05] active:bg-black/[.03]  dark:text-white dark:hover:bg-white/[.06] dark:active:bg-white/[.04]'
        )}
      >
        {!isWindowMaximized ? <Icons.maximizeWin /> : <Icons.maximizeRestoreWin />}
      </button>
      <button
        onClick={closeWindow}
        className={cn(
          'items-center justify-center inline-flex',
          'h-[32px] w-[46px] cursor-default rounded-none bg-transparent text-black/90 hover:bg-[#c42b1c] hover:text-white active:bg-[#c42b1c]/90 dark:text-white'
        )}
      >
        <Icons.closeWin />
      </button>
    </div>
  )
}

export function MacOS({ className, ...props }: React.HTMLProps<HTMLDivElement>) {
  return (
    <div className={cn('inline-flex gap-2 px-4', className)} {...props}>
      {/* Nothing to do */}
    </div>
  )
}

export function Gnome({ className, ...props }: React.HTMLProps<HTMLDivElement>) {
  const [isWindowMaximized, setIsWindowMaximized] = useState(false)

  const updateIsWindowMaximized = useCallback(async () => {
    const resolvedPromise = await appWindow.isMaximized()
    setIsWindowMaximized(resolvedPromise)
  }, [])

  useEffect(() => {
    updateIsWindowMaximized()
    let unlisten: () => void = () => {}
    const listen = async () => {
      unlisten = await appWindow.onResized(() => {
        updateIsWindowMaximized()
      })
    }
    listen()
    return () => unlisten && unlisten()
  }, [updateIsWindowMaximized])

  const minimizeWindow = async () => await appWindow.minimize()
  const maximizeWindow = async () => {
    await appWindow.toggleMaximize()
  }
  const closeWindow = async () => await appWindow.close()

  return (
    <div className={cn('inline-flex gap-4 items-center justify-center px-2', className)} {...props}>
      <button
        onClick={minimizeWindow}
        className={cn(
          'items-center justify-center inline-flex',
          'm-0 aspect-square h-[24px] cursor-default rounded-full bg-[#dadada] p-0 text-[#3d3d3d] hover:bg-[#d1d1d1] active:bg-[#bfbfbf] dark:bg-[#373737] dark:text-white dark:hover:bg-[#424242] dark:active:bg-[#565656]'
        )}
      >
        <Icons.minimizeWin className='w-[9px]' />
      </button>
      <button
        onClick={maximizeWindow}
        className={cn(
          'items-center justify-center inline-flex',
          'm-0 aspect-square h-[24px] cursor-default rounded-full bg-[#dadada] p-0 text-[#3d3d3d] hover:bg-[#d1d1d1] active:bg-[#bfbfbf] dark:bg-[#373737] dark:text-white dark:hover:bg-[#424242] dark:active:bg-[#565656]'
        )}
      >
        {!isWindowMaximized ? (
          <Icons.maximizeWin className='h-2 w-2' />
        ) : (
          <Icons.maximizeRestoreWin className='h-[9px] w-[9px]' />
        )}
      </button>
      <button
        onClick={closeWindow}
        className={cn(
          'items-center justify-center inline-flex',
          'm-0 aspect-square h-[24px] cursor-default rounded-full bg-[#dadada] p-0 text-[#3d3d3d] hover:bg-[#d1d1d1] active:bg-[#bfbfbf] dark:bg-[#373737] dark:text-white dark:hover:bg-[#424242] dark:active:bg-[#565656]'
        )}
      >
        <Icons.closeWin className='h-2 w-2' />
      </button>
    </div>
  )
}
