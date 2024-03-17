import { invoke } from '@tauri-apps/api/core'

import type { Theme } from '@/types/generated'

type SettingTypes = {
  theme: Theme
}

export type SettingsParam = keyof SettingTypes

interface SettingsItem {
  param: SettingsParam
  value: Settings[SettingsParam]
}

export type Settings = {
  [param in SettingsParam]: SettingTypes[param] | null
}

export async function saveSetting(param: SettingsParam, value: Settings[SettingsParam]) {
  if (param === 'theme') {
    await invoke('set_theme', { theme: value })
  }
  return await invoke('save_setting', { param, value })
}

export async function getSettings(): Promise<Settings> {
  const settings = await invoke<SettingsItem[]>('load_settings')
  const result: Settings = {} as Settings
  for (const { param, value } of settings) {
    result[param] = value
  }
  return result
}
