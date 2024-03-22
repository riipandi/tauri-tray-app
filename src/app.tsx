// Copyright 2023-2024 Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

/**
 * This file contains basic example for using Tauri, Vite, and SolidJS.
 *
 * @ref: https://beta.tauri.app/features/commands
 * @ref: https://beta.tauri.app/references/v2/js/core/namespaceevent
 */

import { type RouteDefinition, Router } from '@solidjs/router'
import { listen } from '@tauri-apps/api/event'
import { type ParentComponent, lazy, onMount } from 'solid-js'

import { DefaultTitleBar } from '@/components/titlebar'
import type { AppSettings } from '@/types/generated'

export const routes: RouteDefinition[] = [
  { path: '/', component: lazy(() => import('./screens/welcome')) },
  {
    path: '/settings',
    component: lazy(() => import('./screens/settings/_layout')),
    children: [
      { path: '/', component: lazy(() => import('./screens/settings/general')) },
      { path: '/updates', component: lazy(() => import('./screens/settings/updates')) },
      { path: '/about', component: lazy(() => import('./screens/settings/about')) },
    ],
  },
  { path: '*404', component: lazy(() => import('./screens/not-found')) },
]

const RootLayout: ParentComponent = ({ children }) => {
  return (
    <div class="root-container">
      <DefaultTitleBar />
      <div class="main-container">{children}</div>
    </div>
  )
}

export default function App() {
  onMount(async () => {
    // TODO reload component on change, unlisten onCleanup
    await listen<AppSettings>('settings-updated', ({ payload }) => {
      console.info('Settings updated', payload)
    })
  })

  return (
    <Router root={RootLayout} preload>
      {routes}
    </Router>
  )
}
