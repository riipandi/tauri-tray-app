/// <reference types="vitest" />
/// <reference types="vite/client" />

import { resolve } from 'node:path'

import react from '@vitejs/plugin-react'
import { defineConfig } from 'vite'

export default defineConfig(async () => ({
  plugins: [react()],
  define: { 'import.meta.env.APP_VERSION': `"${process.env.npm_package_version}"` },
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  // @reference: https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
  clearScreen: false,
  server: { port: 1420, strictPort: true },
  envPrefix: ['VITE_', 'TAURI_'],
  resolve: {
    alias: [
      { find: '@', replacement: resolve(__dirname, 'src') },
      { find: '~', replacement: resolve(__dirname, 'public') },
    ],
  },
  build: {
    emptyOutDir: true,
    chunkSizeWarningLimit: 1200,
    reportCompressedSize: false,
    outDir: resolve(__dirname, 'dist'),
  },
  test: {
    globals: true,
    environment: 'jsdom',
    cache: { dir: './node_modules/.vitest' },
    include: ['./**/*.{test,spec}.{ts,tsx}'],
  },
}))
