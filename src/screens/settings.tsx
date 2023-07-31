import { useState } from 'react'
import { useLocation } from 'wouter'
import * as Ariakit from '@ariakit/react'
import { info } from '@tauri-apps/plugin-log'
import { message } from '@tauri-apps/api/dialog'
import { WebviewWindow } from '@tauri-apps/api/window'

export default function SettingScreen() {
  const [_location, navigate] = useLocation()
  const [anchorRect, setAnchorRect] = useState({ x: 0, y: 0 })
  const menu = Ariakit.useMenuStore()

  const handleContextMenu = (e: React.MouseEvent<HTMLDivElement>) => {
    e.preventDefault()
    setAnchorRect({ x: e.clientX, y: e.clientY })
    menu.show()
  }

  const handleContextItem = async () => {
    info('not yet implemented')
    await message('Not yet implemented', { title: 'Tauri App', type: 'info' })
  }

  const handleInspect = async () => {
    const webview = new WebviewWindow('external-page', {
      url: 'https://talk.brave.com/RYPtOrN6vkOJOe-IdD5xkjfarmb4aGRy5PSmpqIZPIc',
      title: 'External Site',
      userAgent:
        'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.2 Safari/605.1.15',
    })
    webview.once('tauri://created', function () {
      info('webview window successfully created')
    })
    webview.once('tauri://error', function (e) {
      console.error(e)
    })
  }

  return (
    <div className='flex h-full w-full min-h-screen flex-col justify-center'>
      <div className='p-8 sm:p-12 lg:p-20 h-full w-full'>
        <div
          className='bg-white/50 w-full h-full min-h-[540px] rounded-lg border-2 border-dashed border-gray-300 p-12 text-center flex flex-col justify-center'
          onContextMenu={handleContextMenu}
        >
          <h1 className='block text-3xl font-bold dark:text-gray-300 dark:text-white sm:text-4xl'>
            Howdy
          </h1>
          <div className='mt-3 text-base sm:text-lg text-gray-700 dark:text-gray-400 sm:mt-4'>
            <p className='leading-8'>
              Right click on your mouse in this area to see custom context menu.
            </p>
          </div>

          {/* begin context menu */}
          <Ariakit.Menu
            store={menu}
            getAnchorRect={() => anchorRect}
            className='disable-select relative flex w-full flex-col bg-white shadow p-2 rounded overflow-auto text-sm'
            modal
          >
            <Ariakit.MenuItem
              className='hover:bg-gray-200 rounded cursor-default py-1 px-2'
              onClick={() => navigate('/')}
            >
              Back
            </Ariakit.MenuItem>
            <Ariakit.MenuItem
              className='hover:bg-gray-200 rounded cursor-default py-1 px-2 text-gray-400'
              onClick={handleContextItem}
              disabled
            >
              Forward
            </Ariakit.MenuItem>
            <Ariakit.MenuItem
              className='hover:bg-gray-200 rounded cursor-default py-1 px-2'
              onClick={() => window.location.reload()}
            >
              Reload
            </Ariakit.MenuItem>
            <Ariakit.MenuSeparator className='my-1' />
            <Ariakit.MenuItem
              className='hover:bg-gray-200 rounded cursor-default py-1 px-2'
              onClick={handleContextItem}
            >
              View Page Source
            </Ariakit.MenuItem>
            <Ariakit.MenuItem
              className='hover:bg-gray-200 rounded cursor-default py-1 px-2'
              onClick={handleInspect}
            >
              Inspect
            </Ariakit.MenuItem>
          </Ariakit.Menu>
          {/* end context menu */}
        </div>
      </div>
    </div>
  )
}
