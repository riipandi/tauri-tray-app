// Copyright 2023-2024 Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

import Link from '@/components/link'
import { invoke } from '@tauri-apps/api/core'
import { BoltIcon, LifeBuoyIcon, RefreshCcwDotIcon } from 'lucide-solid'
import type { ParentComponent } from 'solid-js'

const SettingsLayout: ParentComponent = ({ children }) => {
  return (
    <div class="dark:bg-black disable-select flex flex-1">
      <div class="absolute w-full h-7 bg-transparent z-10" data-tauri-drag-region />
      <aside class="fixed flex flex-col w-44 pt-6 h-screen overflow-y-auto bg-white border-r border-neutral-200 dark:bg-neutral-950 dark:border-neutral-800">
        <div class="mt-4 mx-2">
          <h1 class="mx-2 text-base font-semibold dark:text-white">Settings</h1>
        </div>

        <div class="flex flex-col justify-between flex-1 p-3">
          <nav class="space-y-1.5 h-full mb-2">
            <Link
              href="/settings"
              activeClass="text-neutral-700 dark:text-neutral-200 bg-neutral-100 dark:bg-neutral-900"
              inactiveClass="text-neutral-600 dark:text-neutral-400 hover:bg-neutral-100 dark:hover:bg-neutral-800 dark:hover:text-neutral-200 hover:text-neutral-700"
              class="flex items-center px-3 py-2 transition-colors duration-75 transform rounded-md cursor-default"
            >
              <BoltIcon class="size-4" strokeWidth={1.6} />
              <span class="mx-2 text-sm font-medium">General</span>
            </Link>

            <Link
              href="/settings/updates"
              activeClass="text-neutral-700 dark:text-neutral-200 bg-neutral-100 dark:bg-neutral-900"
              inactiveClass="text-neutral-600 dark:text-neutral-400 hover:bg-neutral-100 dark:hover:bg-neutral-800 dark:hover:text-neutral-200 hover:text-neutral-700"
              class="flex items-center px-3 py-2 transition-colors duration-75 transform rounded-md cursor-default"
            >
              <RefreshCcwDotIcon class="size-4" strokeWidth={1.6} />
              <span class="mx-2 text-sm font-medium">Updates</span>
            </Link>
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
      <main class="ml-44 px-4 pt-10 pb-4 dark:bg-dark-grey size-full dark:text-white">
        {children}
      </main>
    </div>
  )
}

export default SettingsLayout
