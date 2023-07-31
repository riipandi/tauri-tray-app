import { create } from 'zustand'
import { devtools, persist, createJSONStorage } from 'zustand/middleware'
import { zustandStorage } from './engine'

export type ThemesType = 'dark' | 'light' | 'system'

interface UIConfigState {
  theme: ThemesType
  setTheme: (val: ThemesType) => void
  resetState: () => void
}

export const useUIConfigStore = create<UIConfigState>()(
  devtools(
    persist(
      (set) => ({
        theme: 'system',
        setTheme: (val) => set(() => ({ theme: val })),
        // clear the entire store and actions states
        resetState: () => set({}, true),
      }),
      {
        name: 'ui-config',
        storage: createJSONStorage(() => zustandStorage),
      }
    )
  )
)
