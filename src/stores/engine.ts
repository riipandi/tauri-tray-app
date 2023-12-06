import { Store } from '@tauri-apps/plugin-store'
import { StateStorage } from 'zustand/middleware'

// Initialize Tauri data store
export const cfgStore = new Store('settings.json')

// Custom storage object for Zustand
export const zustandStorage: StateStorage = {
  getItem: async (name: string): Promise<string | null> => {
    const val: any = await cfgStore.get(name)
    return JSON.stringify(val) || null
  },
  setItem: async (name: string, value: string): Promise<void> => {
    await cfgStore.set(name, JSON.parse(value))
    // this manually saves the store, otherwise the store
    // is only saved when your app is closed.
    await cfgStore.save()
  },
  removeItem: async (name: string): Promise<void> => {
    await cfgStore.delete(name)
  },
}
