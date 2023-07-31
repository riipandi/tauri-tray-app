import { StateStorage } from 'zustand/middleware'
import { Store } from '@tauri-apps/plugin-store'

// Initialize Tauri data store
// Stored at: ~/Library/Application Support/<bundle_identifier>
export const store = new Store('settings.dat')

// Custom storage object for Zustand
export const zustandStorage: StateStorage = {
  getItem: async (name: string): Promise<string | null> => {
    const val: any = await store.get(name)
    return val || null
  },
  setItem: async (name: string, value: string): Promise<void> => {
    await store.set(name, value)
    // this manually saves the store, otherwise the store
    // is only saved when your app is closed.
    await store.save()
  },
  removeItem: async (name: string): Promise<void> => {
    await store.delete(name)
  },
}
