// Copyright 2023-2024 Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

import { attachConsole } from '@tauri-apps/plugin-log'
import { render } from 'solid-js/web'

import App from './app'
import './assets/styles/main.css'

// Print logs to the browser console (TargetKind::Webview)
const detach = await attachConsole()
const root = document.getElementById('root') as HTMLElement

if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
  throw new Error(
    'Root element not found. Did you forget to add it to your index.html? Or maybe the id attribute got misspelled?'
  )
}

// Set `withGlobalTauri` to `true` in `tauri.conf.json`.
// If the frontend running in browser, throw an error because
// this application will not work in Browser.
if (!('__TAURI__' in window)) {
  render(
    () => (
      <div class="w-full h-full p-4 min-h-screen items-center justify-center flex dark:bg-black">
        <p class="font-medium tracking-wide dark:text-white">
          This application will not work in Browser.
        </p>
      </div>
    ),
    root
  )
}

render(() => <App />, root)

detach() // detach the browser console from the log stream
