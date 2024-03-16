// Copyright 2023-2024 Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

import { A } from '@solidjs/router'

export default function NotFound() {
  return (
    <div class="mx-auto h-screen flex flex-col dark:bg-black">
      <div class="flex-1 flex flex-col justify-center items-center space-y-8">
        <h1 class="text-2xl font-bold dark:text-white">404 Not Found</h1>
        <div class="mt-6">
          <A href="/" class="text-blue-light hover:text-blue-light/80">
            Back to main screen
          </A>
        </div>
      </div>
    </div>
  )
}
