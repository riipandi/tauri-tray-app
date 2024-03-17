// Copyright 2023-2024 Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

import { getVersion } from '@tauri-apps/api/app'
import { invoke } from '@tauri-apps/api/core'
import { CogIcon, UserIcon } from 'lucide-solid'
import { Suspense, createSignal } from 'solid-js'

import Link from '@/components/link'
import PageLoader from '@/components/loader'

import solidjsLogo from '@/assets/images/solid.svg'
import tauriLogo from '@/assets/images/tauri.svg'
import viteLogo from '@/assets/images/vite.svg'
import { clx } from '@/utils/helpers'

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
    <Suspense fallback={<PageLoader />}>
      <div class="flex flex-col items-center justify-center size-full p-4 disable-select">
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

          <div class="mt-12 text-base text-center leading-7 dark:text-neutral-100 enable-select">
            <p>This is the default screen of Tauri App v{appVersion}</p>
            <p>
              Visit{' '}
              <Link href="/not-found" class="font-medium text-blue-light hover:text-blue-light/80">
                this sample page
              </Link>{' '}
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
                        'py-1.5 pl-7 pr-2 block w-full border-neutral-200 rounded-md text-base focus:border-blue-500',
                        'focus:outline-none disabled:opacity-50 disabled:pointer-events-none dark:bg-neutral-900',
                        'dark:border-neutral-700 dark:text-neutral-400 shadow-sm'
                      )}
                      placeholder="Enter a name..."
                    />
                    <div class="pointer-events-none absolute inset-y-0 left-0 flex items-center px-2 text-neutral-500">
                      <UserIcon class="size-4" strokeWidth={1.6} />
                    </div>{' '}
                  </div>
                </div>
              </div>
              <button
                type="submit"
                class={clx(
                  'py-1.5 px-3 inline-flex items-center gap-x-2 text-sm rounded-md border border-neutral-200',
                  'bg-white text-neutral-800 shadow-sm hover:bg-neutral-50 disabled:opacity-50 disabled:pointer-events-none',
                  'dark:bg-neutral-900 dark:border-neutral-700 dark:text-white dark:hover:bg-neutral-800 focus:outline-none',
                  'focus:border-blue-500 active:border-blue-500'
                )}
              >
                Say Hello
              </button>
            </form>
            <button
              type="button"
              onClick={handleSetting}
              class={clx(
                'p-1.5 inline-flex items-center gap-x-2 text-sm rounded-md border border-neutral-200',
                'bg-white text-neutral-800 shadow-sm hover:bg-neutral-50 disabled:opacity-50 disabled:pointer-events-none',
                'dark:bg-neutral-900 dark:border-neutral-700 dark:text-white dark:hover:bg-neutral-800 dark:focus:outline-none',
                'dark:focus:ring-1 dark:focus:ring-neutral-600'
              )}
            >
              <CogIcon class="size-5" strokeWidth={1.6} />
              <span class="sr-only">Settings</span>
            </button>
          </div>
          <p class="mt-8 text-base max-w-2xl text-center dark:text-neutral-100">{greetMsg()}</p>
        </div>
      </div>
    </Suspense>
  )
}
