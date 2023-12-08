import { message } from '@tauri-apps/api/dialog'
import { invoke } from '@tauri-apps/api/tauri'
import {
  Menu,
  MenuContent,
  MenuContextTrigger,
  MenuItem,
  MenuPositioner,
  MenuSeparator,
  Portal,
} from '@ark-ui/react'
import { info } from 'tauri-plugin-log-api'
import { useLocation } from 'wouter'

import { openWebview } from '../libraries/utils'
interface WithContextMenuProps {
  children: React.ReactNode
  className?: string
}

const WithContextMenu = ({ children, className }: WithContextMenuProps) => {
  const [_location, navigate] = useLocation()

  const handleContextItem = async () => {
    info('not yet implemented')
    await message('Not yet implemented', { title: 'Tauri App', type: 'info' })
  }

  const handleOpenChildWindow = async () => {
    await invoke('create_child_window', { id: 'setting-window', title: 'Setting' })
  }

  const handleOpenWebpage = async () => {
    const sizes = [840, 480]
    openWebview('second-window', 'https://browserleaks.com/ip', {
      title: 'External Site',
      minHeight: sizes[1],
      minWidth: sizes[2],
      maxWidth: sizes[1],
      maxHeight: sizes[2],
    })
  }

  return (
    <div className={className}>
      <Menu>
        <MenuContextTrigger className='flex h-full min-h-[540px] w-full cursor-default flex-col items-center justify-center rounded-lg border-2 border-dashed border-gray-300 bg-white/50 p-12 text-center dark:bg-gray-800'>
          {children}
        </MenuContextTrigger>
        <Portal>
          <MenuPositioner>
            <MenuContent className='disable-select dark:bg-background-dark w-[168px] space-y-1 rounded border border-gray-200 bg-white p-2 text-sm shadow-sm dark:border-gray-800'>
              <MenuItem
                id='back'
                className='dark:text-foreground-dark cursor-default rounded px-2 py-1 hover:bg-gray-100/90 dark:hover:bg-gray-700'
                onClick={() => navigate('/')}
              >
                Back
              </MenuItem>
              <MenuItem
                id='forward'
                className='cursor-default rounded px-2 py-1 text-gray-600 hover:bg-gray-100/90 dark:text-gray-700 dark:hover:bg-gray-600 dark:hover:text-gray-500'
                disabled
              >
                Forward
              </MenuItem>
              <MenuItem
                id='reload'
                className='dark:text-foreground-dark cursor-default rounded px-2 py-1 hover:bg-gray-100/90 dark:hover:bg-gray-700'
                onClick={() => window.location.reload()}
              >
                Reload
              </MenuItem>
              <MenuSeparator />
              <MenuItem
                id='sample-dialog'
                className='dark:text-foreground-dark cursor-default rounded px-2 py-1 hover:bg-gray-100/90 dark:hover:bg-gray-700'
                onClick={handleContextItem}
              >
                Open Dialog
              </MenuItem>
              <MenuItem
                id='open-webpage'
                className='dark:text-foreground-dark cursor-default rounded px-2 py-1 hover:bg-gray-100/90 dark:hover:bg-gray-700'
                onClick={handleOpenWebpage}
              >
                Open webpage
              </MenuItem>
              <MenuItem
                id='open-webpage'
                className='dark:text-foreground-dark cursor-default rounded px-2 py-1 hover:bg-gray-100/90 dark:hover:bg-gray-700'
                onClick={handleOpenChildWindow}
              >
                Open child window
              </MenuItem>
              <MenuSeparator />
              <MenuItem
                id='inspect'
                className='dark:text-foreground-dark cursor-default rounded px-2 py-1 hover:bg-gray-100/90 dark:hover:bg-gray-700'
                onClick={async () => await invoke('open_devtools')}
              >
                Inspect
              </MenuItem>
            </MenuContent>
          </MenuPositioner>
        </Portal>
      </Menu>
    </div>
  )
}

export default function SettingScreen() {
  const handleCheckUpdate = async () => {
    await invoke('check_update')
  }

  return (
    <div className='flex h-full min-h-screen w-full flex-col justify-center'>
      <WithContextMenu className='h-full w-full p-8 sm:p-12 lg:p-20'>
        <h1 className='block text-3xl font-bold dark:text-gray-300 sm:text-4xl'>Howdy</h1>
        <div className='mt-3 text-base text-gray-700 dark:text-gray-400 sm:mt-4 sm:text-lg'>
          <p className='leading-8'>
            Right click on your mouse in this area to see custom context menu.
          </p>
          <div className='mt-4'>
            <button
              type='submit'
              className='rounded-lg border border-gray-300 bg-gray-200 px-5 py-2 text-center text-sm font-medium text-gray-600 transition-all hover:border-gray-200 hover:bg-gray-200 focus:ring focus:ring-gray-50 disabled:border-gray-50 disabled:bg-gray-50 disabled:text-gray-400 dark:bg-gray-100'
              onClick={handleCheckUpdate}
            >
              Check for Updates
            </button>
          </div>
        </div>
      </WithContextMenu>
    </div>
  )
}
