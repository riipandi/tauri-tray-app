import { useCallback, useRef, useState } from 'react'
import { message } from '@tauri-apps/api/dialog'
import { invoke } from '@tauri-apps/api/tauri'
import { info } from 'tauri-plugin-log-api'
import { Link } from 'wouter'

import { Button } from '@/components/ui-elements/button'

import useHotKeys from '@/hooks/useHotKeys'
import { ApiResponse } from '@/types/api-response'
import { Quote } from '@/types/quote'
import { AllQuotes } from '@/types/quotes'

import reactLogo from '../assets/react.svg'
import { ThemeSwitcher } from '../components/theme-switcher'
import { cn, randomNumber } from '../libraries/utils'

export default function WelcomeScreen() {
  const [greetMsg, setGreetMsg] = useState<string | undefined>(undefined)
  const [name, setName] = useState<string | undefined>(undefined)

  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  const greet = async () => {
    info('The button clicked')
    setGreetMsg(undefined)

    try {
      const quotes = await invoke<ApiResponse<AllQuotes>>('get_quotes')
      console.info(quotes)

      const resp = await invoke<ApiResponse<Quote>>('get_single_quote', { id: randomNumber() })
      setGreetMsg(`Hello ${name}! ${resp.data?.quote}`)
    } catch (err: any) {
      setGreetMsg(`Something wrong: ${err.message}`)
    }
  }

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

  const inputRef = useRef<HTMLInputElement>(null)
  const handleKeyF = useCallback(() => inputRef.current?.focus(), [inputRef])
  const handleKeyL = async () => {
    await message('This hotkeys not yet implemented!')
  }

  // Use the custom hook for registering hotkeys
  useHotKeys(() => handleKeyF(), 'CmdOrCtrl+F')
  useHotKeys(() => handleKeyL(), 'CmdOrCtrl+Shift+L')

  return (
    <div className='mx-auto flex h-screen flex-col items-center justify-center'>
      <div className='text-center'>
        <h1 className='text-5xl font-bold dark:text-gray-100'>Howdy!</h1>

        <div className='mt-16 flex flex-wrap justify-center gap-5'>
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

        <div className='mt-12 text-center leading-8 dark:text-gray-100'>
          <p>This is the default screen of Tauri Tray App v{import.meta.env.APP_VERSION}.</p>
          <p>
            Visit{' '}
            <Link href='/settings' className='font-medium text-blue-800 hover:text-blue-600'>
              this sample page
            </Link>{' '}
            to see if the router is working or not.
          </p>
        </div>
        <div className='mt-8 flex flex-wrap items-center justify-center space-x-4'>
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
                    className='block rounded-md border-gray-300 px-9 text-sm font-medium shadow-sm focus:border-gray-400 focus:ring focus:ring-gray-200/50 disabled:cursor-not-allowed disabled:bg-gray-50 disabled:text-gray-500 dark:border-gray-700 dark:bg-background-dark dark:text-gray-300'
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
                    <span className='group-hover:border-primary-500 group-hover:text-primary-500 rounded border px-1.5 text-sm text-gray-400 shadow-sm transition-all dark:border-gray-700 dark:bg-background-dark dark:text-gray-300'>
                      <kbd>⌘</kbd> <kbd>F</kbd>
                    </span>
                  </div>
                </div>
              </div>
            </div>

            <Button type='submit'>Say Hello</Button>
          </form>
        </div>
        <p className={cn(name === '' ? 'hidden' : 'mt-8 max-w-2xl text-center dark:text-gray-100')}>
          {greetMsg}
        </p>
      </div>
    </div>
  )
}
