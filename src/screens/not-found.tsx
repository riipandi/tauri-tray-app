// Copyright 2023-2024 Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

import { useLocation, useNavigate } from '@solidjs/router'

export default function NotFound() {
  const navigate = useNavigate()
  const { state } = useLocation<{ previous?: string }>()
  const backPath = () => (state?.previous ? -1 : '/') as string

  return (
    <div class="mx-auto h-screen flex flex-col bg-transparent disable-select">
      <div class="flex-1 flex flex-col justify-center items-center space-y-8">
        <h1 class="text-2xl font-bold dark:text-white">404 Not Found</h1>
        <div class="mt-6">
          <button
            type="button"
            class="text-blue-light hover:text-blue-light/80"
            onClick={() => navigate(backPath())}
          >
            Back to main screen
          </button>
        </div>
      </div>
    </div>
  )
}
