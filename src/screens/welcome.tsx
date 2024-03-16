// Copyright 2023-2024 Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

import { A } from '@solidjs/router'
import { getVersion } from '@tauri-apps/api/app'
import { invoke } from '@tauri-apps/api/core'
import { CogIcon, SearchIcon } from 'lucide-solid'
import { createSignal } from 'solid-js'

import solidjsLogo from '../assets/images/solid.svg'
import tauriLogo from '../assets/images/tauri.svg'
import viteLogo from '../assets/images/vite.svg'
import { clx } from '../utils/helpers'

// Get application version from Tauri
const appVersion = await getVersion()

export default function WelcomeScreen() {
  const [greetMsg, setGreetMsg] = createSignal('')
  const [name, setName] = createSignal('')

  async function handleGreet() {
    const result = await invoke<string>('greet', { name: name() })
    setGreetMsg(result)
  }

  async function handleSetting() {
    await invoke('open_settings_window')
  }

  return (
    <div class="flex flex-col items-center justify-center size-full p-4">
      <div class="text-center">
        <div class="mt-16 flex flex-wrap justify-center gap-8">
          <span class="size-16">
            <img src={viteLogo} class="size-full" alt="Vite logo" />
          </span>
          <span class="size-16">
            <img src={tauriLogo} class="size-full" alt="Tauri logo" />
          </span>
          <span class="size-16">
            <img src={solidjsLogo} class="size-full" alt="React logo" />
          </span>
        </div>

        <div class="mt-12 text-center leading-8 dark:text-gray-100">
          <p>This is the default screen of Tauri App v{appVersion}</p>
          <p>
            Visit{' '}
            <A href="/not-found" class="font-medium text-blue-light hover:text-blue-light/80">
              this sample page
            </A>{' '}
            to see if the router is working or not.
          </p>
        </div>

        <div class="mt-8 flex flex-wrap items-center justify-center space-x-2">
          <form
            class="flex flex-wrap justify-center gap-2"
            onSubmit={(e) => {
              e.preventDefault()
              handleGreet()
            }}
          >
            <div class="mx-auto max-w-xs">
              <div>
                <div class="group relative">
                  <input
                    type="text"
                    id="greet-input"
                    onChange={(e) => setName(e.currentTarget.value)}
                    class={clx(
                      'py-2 pl-9 pr-3 block w-full border-gray-200 rounded-md text-sm focus:border-blue-500',
                      'focus:ring-blue-500 disabled:opacity-50 disabled:pointer-events-none dark:bg-gray-900',
                      'dark:border-gray-700 dark:text-gray-400 dark:focus:ring-gray-600'
                    )}
                    placeholder="Enter a name..."
                  />
                  <div class="pointer-events-none absolute inset-y-0 left-0 flex items-center px-2.5 text-gray-500">
                    <SearchIcon class="size-4" strokeWidth={1.6} />
                  </div>{' '}
                </div>
              </div>
            </div>
            <button
              type="submit"
              class={clx(
                'py-2 px-3 inline-flex items-center gap-x-2 text-sm font-medium rounded-md border border-gray-200',
                'bg-white text-gray-800 shadow-sm hover:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none',
                'dark:bg-gray-900 dark:border-gray-700 dark:text-white dark:hover:bg-gray-800 dark:focus:outline-none',
                'dark:focus:ring-1 dark:focus:ring-gray-600'
              )}
            >
              Say Hello
            </button>
          </form>
          <button
            type="button"
            onClick={handleSetting}
            class={clx(
              'py-2 px-3 inline-flex items-center gap-x-2 text-sm font-medium rounded-md border border-gray-200',
              'bg-white text-gray-800 shadow-sm hover:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none',
              'dark:bg-gray-900 dark:border-gray-700 dark:text-white dark:hover:bg-gray-800 dark:focus:outline-none',
              'dark:focus:ring-1 dark:focus:ring-gray-600'
            )}
          >
            <CogIcon class="size-4" strokeWidth={1.6} />
            <span class="sr-only">Settings</span>
          </button>
        </div>
        <p class="mt-8 max-w-2xl text-center dark:text-gray-100">{greetMsg()}</p>
      </div>
    </div>
  )
}
