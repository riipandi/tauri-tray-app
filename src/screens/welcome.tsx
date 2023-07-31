import { useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { info } from '@tauri-apps/plugin-log'

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
            <input
              id='greet-input'
              className='block dark:text-gray-900 font-medium text-sm rounded-md border-gray-300 shadow-sm focus:border-blue-400 focus:ring focus:ring-blue-200 focus:ring-opacity-50 disabled:cursor-not-allowed disabled:bg-gray-50 disabled:text-gray-500'
              onChange={(e) => setName(e.currentTarget.value)}
              placeholder='Enter a name...'
              autoComplete='off'
              autoCorrect='off'
            />
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
