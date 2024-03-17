// Copyright 2023-2024 Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

/**
 * This file contains basic example for using Tauri, Vite, and SolidJS.
 *
 * @ref: https://beta.tauri.app/features/commands
 * @ref: https://beta.tauri.app/references/v2/js/core/namespaceevent
 */

import { type RouteDefinition, Router } from '@solidjs/router'
import { type ParentComponent, lazy } from 'solid-js'

export const routes: RouteDefinition[] = [
  { path: '/', component: lazy(() => import('./screens/welcome')) },
  { path: '/settings', component: lazy(() => import('./screens/settings')) },
  { path: '*404', component: lazy(() => import('./screens/not-found')) },
]

const RootLayout: ParentComponent = ({ children }) => {
  return (
    <div class="main-container dark:bg-dark-grey">
      <div class="absolute w-full h-7 bg-transparent z-10" data-tauri-drag-region />
      {children}
    </div>
  )
}

export default function App() {
  return <Router root={RootLayout}>{routes}</Router>
}
