import { ThemeType, useUIConfigStore } from '../stores/ui-store'
import { appWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api/tauri'
import { cn } from '../libraries/utils'

export function ThemeSwitcher() {
  const { darkmode, setDarkMode } = useUIConfigStore((state) => state)

  const handleSelect = async (e: React.ChangeEvent<HTMLSelectElement>) => {
    e.preventDefault()

    let windowTheme = await appWindow.theme()
    let selectedTheme = e.target.value as ThemeType

    if (windowTheme === 'light' && selectedTheme === 'dark') {
      await invoke('set_darkmode', { enable: true })
      setDarkMode(true)
    } else {
      await invoke('set_darkmode', { enable: false })
      setDarkMode(false)
    }
  }

  return (
    <div className='max-w-[160px]'>
      <label htmlFor='theme-switcher' className='sr-only'>
        Language
      </label>
      <div className='relative'>
        <div className='pointer-events-none absolute inset-y-0 left-0 flex items-center px-2.5'>
          <span
            className={cn(
              darkmode ? 'i-heroicons-moon' : 'i-heroicons-light-bulb',
              'h-4 w-4 text-gray-500'
            )}
          ></span>
        </div>
        <select
          id='theme-switcher'
          className='block w-full rounded-md dark:text-gray-300 dark:bg-background-dark dark:border-gray-700 border-gray-300 pl-9 text-sm focus:border-blue-300 focus:ring focus:ring-blue-200 focus:ring-opacity-50 disabled:cursor-not-allowed disabled:bg-gray-50'
          onChange={handleSelect}
          defaultValue={darkmode ? 'dark' : 'light'}
        >
          <option disabled>Select theme</option>
          <option value='dark'>Dark Theme</option>
          <option value='light'>Light Theme</option>
        </select>
      </div>
    </div>
  )
}
