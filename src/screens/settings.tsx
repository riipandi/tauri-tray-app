// Copyright 2023-2024 Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

import { clx } from '@/utils/helpers'
import { invoke } from '@tauri-apps/api/core'
import {
  LifeBuoyIcon,
  LightbulbIcon,
  MonitorDotIcon,
  MoonStarIcon,
  PaletteIcon,
  RefreshCcwDotIcon,
  UserRoundCogIcon,
} from 'lucide-solid'

export default function SettingScreen() {
  return (
    <div class="dark:bg-black disable-select flex flex-1">
      <div class="absolute w-full h-7 bg-transparent z-10" data-tauri-drag-region />
      <aside class="fixed flex flex-col w-44 pt-6 h-screen overflow-y-auto bg-white border-r border-neutral-200 dark:bg-neutral-950 dark:border-neutral-800">
        <div class="mt-4 mx-2">
          <h1 class="mx-2 text-base font-semibold dark:text-white">Settings</h1>
        </div>

        <div class="flex flex-col justify-between flex-1 p-3">
          <nav class="space-y-1.5 h-full mb-2">
            <a
              class="flex items-center px-3 py-2 text-neutral-700 bg-neutral-100 rounded-md dark:bg-neutral-800 dark:text-neutral-200 cursor-default"
              href="/settings"
            >
              <UserRoundCogIcon class="size-4" strokeWidth={1.6} />
              <span class="mx-2 text-sm font-medium">Account</span>
            </a>

            <a
              class="flex items-center px-3 py-2 text-neutral-600 transition-colors duration-75 transform rounded-md dark:text-neutral-400 hover:bg-neutral-100 dark:hover:bg-neutral-800 dark:hover:text-neutral-200 hover:text-neutral-700 cursor-default"
              href="/settings"
            >
              <PaletteIcon class="size-4" strokeWidth={1.6} />
              <span class="mx-2 text-sm font-medium">Appearance</span>
            </a>

            <a
              class="flex items-center px-3 py-2 text-neutral-600 transition-colors duration-75 transform rounded-md dark:text-neutral-400 hover:bg-neutral-100 dark:hover:bg-neutral-800 dark:hover:text-neutral-200 hover:text-neutral-700 cursor-default"
              href="/settings"
            >
              <RefreshCcwDotIcon class="size-4" strokeWidth={1.6} />
              <span class="mx-2 text-sm font-medium">Updates</span>
            </a>
          </nav>

          <button
            type="button"
            class="flex items-center px-3 py-2 text-neutral-600 transition-colors duration-75 transform rounded-md dark:text-neutral-400 hover:bg-neutral-100 dark:hover:bg-neutral-800 dark:hover:text-neutral-200 hover:text-neutral-700"
            onClick={() => invoke('open_with_shell', { url: 'https://github.com/riipandi' })}
          >
            <LifeBuoyIcon class="size-4" strokeWidth={1.6} />
            <span class="mx-2 text-sm font-medium">Help &amp; Support</span>
          </button>
        </div>
      </aside>
      <main class="ml-44 px-4 pt-10 pb-4 dark:bg-neutral-900 size-full dark:text-white">
        <div class="flex flex-col size-full justify-between pb-4">
          <div class="grid grid-cols-2 gap-x-8 gap-y-4">
            <div>
              <h2 class="text-base font-semibold leading-6 dark:text-white">Change password</h2>
              <p class="mt-1 text-sm leading-4 text-neutral-500 dark:text-neutral-400">
                Update your password associated with your account.
              </p>
            </div>

            <form class="md:col-span-2">
              <div class="grid grid-cols-1 gap-x-6 gap-y-5 sm:max-w-xl sm:grid-cols-6">
                <div class="col-span-full">
                  <label for="current-password" class="block text-sm leading-4 dark:text-white">
                    Current password
                  </label>
                  <div class="mt-2">
                    <input
                      id="current-password"
                      name="current_password"
                      type="password"
                      class={clx(
                        'py-1.5 px-2 block w-full border-neutral-200 rounded-md text-base focus:border-blue-500',
                        'focus:outline-none disabled:opacity-50 disabled:pointer-events-none dark:bg-neutral-900',
                        'dark:border-neutral-700 dark:text-neutral-400 shadow-sm'
                      )}
                    />
                  </div>
                </div>

                <div class="col-span-full">
                  <label for="new-password" class="block text-sm leading-4 dark:text-white">
                    New password
                  </label>
                  <div class="mt-2">
                    <input
                      id="new-password"
                      name="new_password"
                      type="password"
                      class={clx(
                        'py-1.5 px-2 block w-full border-neutral-200 rounded-md text-base focus:border-blue-500',
                        'focus:outline-none disabled:opacity-50 disabled:pointer-events-none dark:bg-neutral-900',
                        'dark:border-neutral-700 dark:text-neutral-400 shadow-sm'
                      )}
                    />
                  </div>
                </div>
              </div>

              <div class="mt-6 flex">
                <button
                  type="submit"
                  class={clx(
                    'py-1.5 px-3 inline-flex items-center gap-x-2 text-sm rounded-md border border-neutral-200',
                    'bg-white text-neutral-800 shadow-sm hover:bg-neutral-50 disabled:opacity-50 disabled:pointer-events-none',
                    'dark:bg-neutral-900 dark:border-neutral-700 dark:text-white dark:hover:bg-neutral-800 focus:outline-none',
                    'focus:border-blue-500 active:border-blue-500 block w-full'
                  )}
                >
                  Save
                </button>
              </div>
            </form>
          </div>

          <div class="grid grid-cols-2 gap-x-8 gap-y-4">
            <div>
              <h2 class="text-base font-semibold leading-6 dark:text-white">Theme</h2>
              <p class="mt-1 text-sm leading-4 text-neutral-500 dark:text-neutral-400">
                Select your preferred theme.
              </p>
            </div>

            <div class="flex overflow-hidden h-9 bg-white border divide-x rounded-md dark:bg-neutral-900 dark:border-neutral-700 dark:divide-neutral-700">
              <button
                type="button"
                class="inline-flex justify-center w-full items-center text-neutral-600 transition-colors duration-75 dark:hover:bg-neutral-800 dark:text-neutral-300 hover:bg-neutral-100"
              >
                <MonitorDotIcon class="size-5" strokeWidth={1.6} />
              </button>

              <button
                type="button"
                class="inline-flex justify-center w-full items-center text-neutral-600 transition-colors duration-75 dark:hover:bg-neutral-800 dark:text-neutral-300 hover:bg-neutral-100"
              >
                <LightbulbIcon class="size-5" strokeWidth={1.6} />
              </button>

              <button
                type="button"
                class="inline-flex justify-center w-full items-center text-neutral-600 transition-colors duration-75 dark:hover:bg-neutral-800 dark:text-neutral-300 hover:bg-neutral-100"
              >
                <MoonStarIcon class="size-5" strokeWidth={1.6} />
              </button>
            </div>
          </div>

          <div class="grid grid-cols-2 gap-x-8 gap-y-4">
            <div>
              <h2 class="text-base font-semibold leading-6 dark:text-white">Delete Account</h2>
              <p class="mt-1 text-sm leading-4 text-neutral-500 dark:text-neutral-400">
                No longer want to use our service? You can delete your account here. This action is
                not reversible. All information related to this account will be deleted permanently.
              </p>
            </div>

            <form class="flex items-start md:col-span-2">
              <button
                type="submit"
                class="rounded-md bg-red-500 py-1.5 px-3 text-sm text-white shadow-sm hover:bg-red-400 block w-full"
              >
                Yes, delete my account
              </button>
            </form>
          </div>
        </div>
      </main>
    </div>
  )
}
