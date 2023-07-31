import { useLocation } from 'wouter'
import { info } from 'tauri-plugin-log-api'
import { message } from '@tauri-apps/api/dialog'
import { WebviewWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api/tauri'
import { appWindow } from '@tauri-apps/api/window'

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

  const handleOpenWebpage = async () => {
    const secondWindow = WebviewWindow.getByLabel('second-window')
    let windowTheme = await appWindow.theme()

    if (!secondWindow) {
      const sizes = [540, 480]
      const webview = new WebviewWindow('second-window', {
        tabbingIdentifier: 'tauri-tray-app',
        url: 'https://browserleaks.com/ip',
        title: 'External Site',
        minHeight: sizes[1],
        minWidth: sizes[2],
        maxWidth: sizes[1],
        maxHeight: sizes[2],
        theme: windowTheme || 'light',
        decorations: true,
        hiddenTitle: true,
        skipTaskbar: true,
        minimizable: false,
        maximizable: false,
        resizable: false,
        center: true,
        focus: true,
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

    secondWindow?.setFocus()
  }

  return (
    <div className='flex h-full w-full min-h-screen flex-col justify-center'>
      <div className='p-8 sm:p-12 lg:p-20 h-full w-full'>
        <Menu>
          <MenuContextTrigger className='bg-white/50 dark:bg-gray-800 w-full h-full min-h-[540px] rounded-lg border-2 border-dashed border-gray-300 p-12 text-center flex flex-col justify-center cursor-default items-center'>
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
              <MenuContent className='disable-select w-[140px] bg-white dark:bg-background-dark p-2 rounded border border-gray-200 dark:border-gray-800 shadow-sm space-y-1 text-sm'>
                <MenuItem
                  id='back'
                  className='hover:bg-gray-100/90 dark:hover:bg-gray-700 dark:text-foreground-dark px-2 py-1 rounded cursor-default'
                  onClick={() => navigate('/')}
                >
                  Back
                </MenuItem>
                <MenuItem
                  id='forward'
                  className='hover:bg-gray-100/90 dark:hover:bg-gray-600 dark:hover:text-gray-500 dark:text-gray-700 px-2 py-1 rounded cursor-default text-gray-600'
                  disabled
                >
                  Forward
                </MenuItem>
                <MenuItem
                  id='reload'
                  className='hover:bg-gray-100/90 dark:hover:bg-gray-700 dark:text-foreground-dark px-2 py-1 rounded cursor-default'
                  onClick={() => window.location.reload()}
                >
                  Reload
                </MenuItem>
                <MenuSeparator />
                <MenuItem
                  id='sample-dialog'
                  className='hover:bg-gray-100/90 dark:hover:bg-gray-700 dark:text-foreground-dark px-2 py-1 rounded cursor-default'
                  onClick={handleContextItem}
                >
                  Open Dialog
                </MenuItem>
                <MenuItem
                  id='open-webpage'
                  className='hover:bg-gray-100/90 dark:hover:bg-gray-700 dark:text-foreground-dark px-2 py-1 rounded cursor-default'
                  onClick={handleOpenWebpage}
                >
                  Open webpage
                </MenuItem>
                <MenuSeparator />
                <MenuItem
                  id='inspect'
                  className='hover:bg-gray-100/90 dark:hover:bg-gray-700 dark:text-foreground-dark px-2 py-1 rounded cursor-default'
                  onClick={async () => await invoke('open_devtools')}
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
