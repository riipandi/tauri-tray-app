// Copyright 2023-2024 Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

import { invoke } from '@tauri-apps/api/core'
import { confirm, message } from '@tauri-apps/plugin-dialog'
import { LightbulbIcon, MonitorDotIcon, MoonStarIcon } from 'lucide-solid'
import { createSignal, onMount } from 'solid-js'

import { Theme } from '@/types/generated'
import { clx } from '@/utils/helpers'
import { getAppSettings, saveSetting } from '@/utils/settings'

export default function SettingGeneral() {
  const [theme, setTheme] = createSignal<Theme | undefined>(undefined)
  const [zoomFactor, setZoomFactor] = createSignal<number>(1)

  const handleSwitchTheme = async (theme: Theme) => {
    await invoke('set_theme', { theme }).then(() => setTheme(theme))
  }

  const handleZoomFactorChange = async ({ target }: Event) => {
    const value = Number.parseInt((target as HTMLInputElement).value)
    await saveSetting('zoom_factor', value).then(() => setZoomFactor(value))
  }

  const handleResetSettings = async () => {
    const dialogTitle = 'Reset Application Settings'
    const confirmation = await confirm('This action cannot be reverted. Are you sure?', {
      title: dialogTitle,
      kind: 'warning',
    })

    if (!confirmation) return

    await message('Application has been reset to default settings.', {
      title: dialogTitle,
      kind: 'info',
    })
  }

  onMount(async () => {
    const { theme, zoom_factor } = await getAppSettings()
    setTheme(theme === 'light' ? Theme.Light : theme === 'dark' ? Theme.Dark : Theme.Auto)
    setZoomFactor(zoom_factor)
  })

  return (
    <div class="flex flex-col size-full justify-between pb-4">
      <section class="space-y-6">
        <div class="grid grid-cols-2 gap-x-8 gap-y-4">
          <div>
            <h2 class="text-base font-semibold leading-6 dark:text-white">Theme</h2>
            <p class="mt-1 text-sm leading-4 text-neutral-500 dark:text-neutral-400">
              Select your preferred theme.
            </p>
          </div>

          <div class="flex overflow-hidden h-8 bg-white border border-neutral-200 divide-x divide-neutral-200 rounded-md dark:bg-neutral-900 dark:border-neutral-700 dark:divide-neutral-700 shadow-sm">
            <button
              type="button"
              class={clx(
                theme() === Theme.Auto
                  ? 'bg-blue-dark dark:bg-blue-dark dark:text-neutral-100 text-neutral-100'
                  : 'hover:bg-neutral-100 dark:hover:bg-neutral-800 text-neutral-600 dark:text-neutral-300',
                'inline-flex justify-center w-full items-center transition-colors duration-75'
              )}
              onClick={() => handleSwitchTheme(Theme.Auto)}
            >
              <MonitorDotIcon class="size-5" strokeWidth={1.6} />
              <span class="sr-only">Auto</span>
            </button>

            <button
              type="button"
              class={clx(
                theme() === Theme.Light
                  ? 'bg-blue-dark dark:bg-blue-dark dark:text-neutral-100 text-neutral-100'
                  : 'hover:bg-neutral-100 dark:hover:bg-neutral-800 text-neutral-600 dark:text-neutral-300',
                'inline-flex justify-center w-full items-center transition-colors duration-75'
              )}
              onClick={() => handleSwitchTheme(Theme.Light)}
            >
              <LightbulbIcon class="size-5" strokeWidth={1.6} />
              <span class="sr-only">Light</span>
            </button>

            <button
              type="button"
              class={clx(
                theme() === Theme.Dark
                  ? 'bg-blue-dark dark:bg-blue-dark dark:text-neutral-100 text-neutral-100'
                  : 'hover:bg-neutral-100 dark:hover:bg-neutral-800 text-neutral-600 dark:text-neutral-300',
                'inline-flex justify-center w-full items-center transition-colors duration-75'
              )}
              onClick={() => handleSwitchTheme(Theme.Dark)}
            >
              <MoonStarIcon class="size-5" strokeWidth={1.6} />
              <span class="sr-only">Dark</span>
            </button>
          </div>
        </div>

        <div class="grid grid-cols-2 gap-x-8 gap-y-4">
          <div>
            <h2 class="text-base font-semibold leading-6 dark:text-white">Zoom Factor</h2>
            <p class="mt-1 text-sm leading-4 text-neutral-500 dark:text-neutral-400">
              Controls the overall zoom level of the application.
            </p>
          </div>

          <div class="md:col-span-2">
            <div class="grid grid-cols-1 gap-x-6 gap-y-5 sm:max-w-xl sm:grid-cols-6">
              <div class="col-span-full">
                <label for="zoom-factor" class="sr-only">
                  Zoom Factor
                </label>
                <div class="mt-2">
                  <input
                    type="number"
                    id="zoom-factor"
                    name="zoom_factor"
                    value={zoomFactor()}
                    max={10}
                    min={-5}
                    class={clx(
                      'py-1.5 px-2 block w-full border-neutral-200 rounded-md text-base focus:border-blue-500',
                      'focus:outline-none disabled:opacity-50 disabled:pointer-events-none dark:bg-neutral-900',
                      'dark:border-neutral-700 dark:text-neutral-400 shadow-sm'
                    )}
                    onChange={handleZoomFactorChange}
                  />
                </div>
              </div>
            </div>
          </div>
        </div>

        <div class="grid grid-cols-2 gap-x-8 gap-y-4">
          <div>
            <h2 class="text-base font-semibold leading-6 dark:text-white">Exit to Tray</h2>
            <p class="mt-1 text-sm leading-4 text-neutral-500 dark:text-neutral-400">
              Close application to the tray.
            </p>
          </div>

          <div class="md:col-span-2">
            <div class="flex items-center">
              <button
                type="button"
                class="bg-gray-200 relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2"
                role="switch"
                aria-checked="false"
                aria-labelledby="availability-label"
                aria-describedby="availability-description"
              >
                <span
                  aria-hidden="true"
                  class="translate-x-0 pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out"
                />
              </button>
            </div>
          </div>
        </div>
      </section>

      <section class="grid grid-cols-2 gap-x-8 gap-y-4">
        <div>
          <h2 class="text-base font-semibold leading-6 dark:text-white">Reset Settings</h2>
          <p class="mt-1 text-sm leading-4 text-neutral-500 dark:text-neutral-400">
            This will reset all settings to their default values. This action is not reversible.
          </p>
        </div>
        <div class="flex items-start md:col-span-2">
          <button
            type="submit"
            class="rounded-md bg-red-500 py-1.5 px-3 text-sm text-white shadow-sm hover:bg-red-400 block w-full"
            onClick={handleResetSettings}
          >
            Reset Default Settings
          </button>
        </div>
      </section>
    </div>
  )
}
