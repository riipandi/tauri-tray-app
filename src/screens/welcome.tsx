import { useCallback, useEffect, useRef, useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { info } from '@tauri-apps/plugin-log'
import { TauriEvent } from '@tauri-apps/api/event'
import { appWindow } from '@tauri-apps/api/window'
import { register, unregisterAll } from '@tauri-apps/api/globalShortcut'

import reactLogo from '../assets/react.svg'
import { cn } from '../libraries/utils'
import { Link } from 'wouter'
import { ThemeSwitcher } from '../components/theme-switcher'

export default function WelcomeScreen() {
  const [greetMsg, setGreetMsg] = useState<string | undefined>(undefined)
  const [name, setName] = useState<string | undefined>(undefined)

  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  const greet = async () => {
    setGreetMsg(await invoke('greet', { name }))
    info('The Greet button clicked')
  }

  const inputRef = useRef<HTMLInputElement>(null)
  const handleFocus = useCallback(() => {
    info('Focus triggered from hotkeys')
    inputRef.current?.focus()
  }, [inputRef])

  const handleKeyDown = async (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Escape') {
      e.preventDefault()
      e.stopPropagation()
      if (name) {
        setName(undefined)
      } else {
        e.currentTarget.blur()
      }
    }
  }

  const registerHotKeys = useCallback(() => {
    register('CmdOrCtrl+F', () => handleFocus()).catch(() => {})
  }, [])

  useEffect(() => {
    registerHotKeys()
    appWindow.listen(TauriEvent.WINDOW_FOCUS, () => registerHotKeys())
    appWindow.listen(TauriEvent.WINDOW_BLUR, () => unregisterAll())
    return () => {
      unregisterAll()
    }
  }, [])

  return (
    <div className='h-screen flex flex-col mx-auto items-center justify-center'>
      <div className='text-center'>
        <h1 className='text-5xl font-bold dark:text-gray-100'>Welcome to Tauri!</h1>

        <div className='flex flex-wrap justify-center gap-5 mt-16'>
          <span className='h-20 w-20'>
            <img src='/vite.svg' className='h-full w-full' alt='Vite logo' />
          </span>
          <span className='h-20 w-20'>
            <img src='/tauri.svg' className='h-full w-full' alt='Tauri logo' />
          </span>
          <span className='h-20 w-20'>
            <img src={reactLogo} className='h-full w-full' alt='React logo' />
          </span>
        </div>

        <div className='mt-12 dark:text-gray-100 text-center leading-8'>
          <p>Click on the Tauri, Vite, and React logos to learn more.</p>
          <p>
            Visit{' '}
            <Link href='/settings' className='font-medium text-blue-500 hover:text-blue-600'>
              this sample page
            </Link>{' '}
            to see if the router is working or not.
          </p>
        </div>
        <div className='flex items-center justify-center flex-wrap space-x-4 mt-8'>
          <ThemeSwitcher />
          <form
            className='flex flex-wrap justify-center gap-4'
            onSubmit={(e) => {
              e.preventDefault()
              greet()
            }}
          >
            <div className='mx-auto max-w-xs'>
              <div>
                <div className='group relative'>
                  <input
                    type='text'
                    ref={inputRef}
                    id='greet-input'
                    className='block dark:text-gray-900 px-9 font-medium text-sm rounded-md border-gray-300 shadow-sm focus:border-blue-400 focus:ring focus:ring-blue-200 focus:ring-opacity-50 disabled:cursor-not-allowed disabled:bg-gray-50 disabled:text-gray-500'
                    onChange={(e) => setName(e.currentTarget.value)}
                    onKeyDown={handleKeyDown}
                    placeholder='Enter a name...'
                    autoComplete='off'
                    autoCorrect='off'
                  />
                  <div className='pointer-events-none absolute inset-y-0 left-0 flex items-center px-2.5 text-gray-500'>
                    <svg
                      xmlns='http://www.w3.org/2000/svg'
                      viewBox='0 0 20 20'
                      fill='currentColor'
                      className='h-4 w-4'
                    >
                      <path
                        fillRule='evenodd'
                        d='M9 3.5a5.5 5.5 0 100 11 5.5 5.5 0 000-11zM2 9a7 7 0 1112.452 4.391l3.328 3.329a.75.75 0 11-1.06 1.06l-3.329-3.328A7 7 0 012 9z'
                        clipRule='evenodd'
                      />
                    </svg>
                  </div>
                  <div className='pointer-events-none absolute inset-y-0 right-0 flex items-center px-2.5'>
                    <span className='rounded border px-1.5 text-sm text-gray-400 shadow-sm transition-all group-hover:border-primary-500 group-hover:text-primary-500'>
                      <kbd>⌘</kbd> <kbd>F</kbd>
                    </span>
                  </div>
                </div>
              </div>
            </div>

            <button
              type='submit'
              className='rounded-lg border border-blue-100 bg-blue-100 px-5 py-2 text-center text-sm font-medium text-blue-600 transition-all hover:border-blue-200 hover:bg-blue-200 focus:ring focus:ring-blue-50 disabled:border-blue-50 disabled:bg-blue-50 disabled:text-blue-400'
            >
              Say Hello
            </button>
          </form>
        </div>
        <p className={cn(name === '' ? 'hidden' : 'dark:text-gray-100 mt-8 text-center')}>
          {greetMsg}
        </p>
      </div>
    </div>
  )
}
