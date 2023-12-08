import { afterEach } from 'node:test'

import { clearMocks, mockIPC, mockWindows } from '@tauri-apps/api/mocks'
import { invoke } from '@tauri-apps/api/tauri'
import { render } from '@testing-library/react'
import { describe, expect, test } from 'vitest'

import App from './app'

describe('app test', () => {
  afterEach(() => {
    clearMocks()
  })

  test('should render successfully', () => {
    mockWindows('main')
    expect(window).toHaveProperty('__TAURI_METADATA__')
  })

  test('should called command successfully', () => {
    mockIPC((cmd, _args) => {
      switch (cmd) {
        case 'get_machine_id':
          return '00000000-0000-0000-0000-000000000000'
        default:
          break
      }
    })

    expect(invoke('get_machine_id')).resolves.toBe('00000000-0000-0000-0000-000000000000')

    const { baseElement } = render(<App />)
    expect(baseElement).toBeTruthy()
  })
})
