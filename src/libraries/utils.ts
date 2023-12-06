import { appWindow, WebviewWindow } from '@tauri-apps/api/window'
import { type ClassValue, clsx } from 'clsx'
import { twMerge } from 'tailwind-merge'
import { info } from 'tauri-plugin-log-api'

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

interface OpenWebviewProps {
  title?: string
  minHeight?: number
  minWidth?: number
  maxWidth?: number
  maxHeight?: number
}

export async function openWebview(label: string, url: string, props?: OpenWebviewProps) {
  const secondWindow = WebviewWindow.getByLabel(label)
  const windowTheme = await appWindow.theme()
  const userAgent =
    'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.2 Safari/605.1.15'

  const options = {
    tabbingIdentifier: 'tauri-tray-app',
    theme: windowTheme || 'light',
    decorations: true,
    hiddenTitle: true,
    skipTaskbar: true,
    minimizable: false,
    maximizable: false,
    resizable: false,
    center: true,
    focus: true,
    userAgent,
    ...props,
  }

  if (secondWindow) {
    secondWindow?.setFocus()
  }

  const webview = new WebviewWindow(label, { url, ...options })

  webview.once('tauri://created', function () {
    info('webview window successfully created')
  })

  webview.once('tauri://error', function (e) {
    console.error(e)
  })

  webview.once('tauri://close-requested', async function (e) {
    console.info(e)
  })
}
