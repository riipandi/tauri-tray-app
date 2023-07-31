import { useLocation } from 'wouter'
import { info } from '@tauri-apps/plugin-log'
import { message } from '@tauri-apps/api/dialog'
import { WebviewWindow } from '@tauri-apps/api/window'

import {
  Menu,
  MenuContent,
  MenuContextTrigger,
  MenuItem,
  MenuPositioner,
  MenuSeparator,
  Portal,
} from '@ark-ui/react'

export default function SettingScreen() {
  const [_location, navigate] = useLocation()

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
        <Menu>
          <MenuContextTrigger className='bg-white/50 w-full h-full min-h-[540px] rounded-lg border-2 border-dashed border-gray-300 p-12 text-center flex flex-col justify-center cursor-default items-center'>
            <h1 className='block text-3xl font-bold dark:text-gray-300 dark:text-white sm:text-4xl'>
              Howdy
            </h1>
            <div className='mt-3 text-base sm:text-lg text-gray-700 dark:text-gray-400 sm:mt-4'>
              <p className='leading-8'>
                Right click on your mouse in this area to see custom context menu.
              </p>
            </div>
          </MenuContextTrigger>
          <Portal>
            <MenuPositioner>
              <MenuContent className='disable-select w-[140px] bg-white p-2 rounded border border-gray-200 shadow-sm space-y-1 text-sm'>
                <MenuItem
                  id='back'
                  className='hover:bg-gray-100/90 px-2 py-1 rounded cursor-default'
                  onClick={() => navigate('/')}
                >
                  Back
                </MenuItem>
                <MenuItem
                  id='forward'
                  className='hover:bg-gray-100/90 px-2 py-1 rounded cursor-default text-gray-600'
                  disabled
                >
                  Forward
                </MenuItem>
                <MenuItem
                  id='reload'
                  className='hover:bg-gray-100/90 px-2 py-1 rounded cursor-default'
                  onClick={() => window.location.reload()}
                >
                  Reload
                </MenuItem>
                <MenuSeparator />
                <MenuItem
                  id='page-source'
                  className='hover:bg-gray-100/90 px-2 py-1 rounded cursor-default'
                  onClick={handleContextItem}
                >
                  View Source
                </MenuItem>
                <MenuItem
                  id='duplicate'
                  className='hover:bg-gray-100/90 px-2 py-1 rounded cursor-default'
                  onClick={handleInspect}
                >
                  Inspect
                </MenuItem>
              </MenuContent>
            </MenuPositioner>
          </Portal>
        </Menu>
      </div>
    </div>
  )
}
