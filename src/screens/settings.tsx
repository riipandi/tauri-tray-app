import { useState } from 'react'
import * as Ariakit from '@ariakit/react'
import { Link } from 'wouter'

export default function SettingScreen() {
  const [anchorRect, setAnchorRect] = useState({ x: 0, y: 0 })
  const menu = Ariakit.useMenuStore()

  return (
    <div className='mx-auto flex h-full min-h-screen w-full flex-col'>
      <header className='mb-auto w-full' aria-hidden></header>
      <div className='px-4 py-10 text-center sm:px-6 lg:px-8'>
        <h1 className='block text-7xl font-bold dark:text-gray-300 dark:text-white sm:text-8xl'>
          Holla!
        </h1>
        <div className='mt-6 text-lg dark:text-gray-200 dark:text-gray-400 sm:mt-8'>
          <p className='leading-8'>This is just an example page.</p>
        </div>

        <div
          className='flex mt-8 bg-white max-w-sm mx-auto rounded p-4'
          onContextMenu={(event) => {
            event.preventDefault()
            setAnchorRect({ x: event.clientX, y: event.clientY })
            menu.show()
          }}
        >
          Right click here
          <Ariakit.Menu
            store={menu}
            modal
            getAnchorRect={() => anchorRect}
            className='relative flex w-full flex-col bg-white shadow p-2 rounded overflow-auto text-sm'
          >
            <Ariakit.MenuItem className='hover:bg-gray-200 rounded cursor-default py-1 px-2'>
              Back
            </Ariakit.MenuItem>
            <Ariakit.MenuItem
              className='hover:bg-gray-200 rounded cursor-default py-1 px-2 text-gray-400'
              disabled
            >
              Forward
            </Ariakit.MenuItem>
            <Ariakit.MenuItem className='hover:bg-gray-200 rounded cursor-default py-1 px-2'>
              Reload
            </Ariakit.MenuItem>
            <Ariakit.MenuSeparator className='my-1' />
            <Ariakit.MenuItem className='hover:bg-gray-200 rounded cursor-default py-1 px-2'>
              View Page Source
            </Ariakit.MenuItem>
            <Ariakit.MenuItem className='hover:bg-gray-200 rounded cursor-default py-1 px-2'>
              Inspect
            </Ariakit.MenuItem>
          </Ariakit.Menu>
        </div>

        <div className='mt-8 flex flex-col items-center justify-center'>
          <Link
            href='/'
            className='inline-flex w-full items-center justify-center gap-2 rounded-md border border-transparent px-3 py-2 text-sm font-semibold text-blue-500 ring-offset-white transition-all hover:text-blue-600 focus:outline-none focus:ring-1 focus:ring-blue-500 focus:ring-offset-2 dark:ring-offset-slate-900 sm:w-auto'
          >
            <svg className='h-2.5 w-2.5' width={20} height={20} viewBox='0 0 16 16' fill='none'>
              <path
                d='M11.2792 1.64001L5.63273 7.28646C5.43747 7.48172 5.43747 7.79831 5.63273 7.99357L11.2792 13.64'
                stroke='currentColor'
                strokeWidth={2}
                strokeLinecap='round'
              />
            </svg>
            Back to main page
          </Link>
        </div>
      </div>
      <footer className='mt-auto py-5 text-center'>
        <div className='mx-auto max-w-7xl px-4 sm:px-6 lg:px-8'>
          <p className='text-sm text-gray-500'>
            © All Rights Reserved. {new Date().getFullYear()}
          </p>
        </div>
      </footer>
    </div>
  )
}
