import { ThemesType, useUIConfigStore } from '../stores/ui-store'
// import { appWindow } from '@tauri-apps/api/window'

export function ThemeSwitcher() {
  const { theme, setTheme } = useUIConfigStore((state) => state)

  const handleSelect = async (e: React.ChangeEvent<HTMLSelectElement>) => {
    e.preventDefault()
    setTheme(e.target.value as ThemesType)

    // // document.getElementsByTagName('html')[0].dataset.theme = e.target.value

    // // On page load or when changing themes, best to add inline in `head` to avoid FOUC
    // if (!theme && window.matchMedia('(prefers-color-scheme: dark)').matches) {
    //   document.documentElement.classList.add('dark')
    //   if (e.target.value === 'dark') {
    //     document.documentElement.classList.add('dark')
    //   } else {
    //     document.documentElement.classList.remove('dark')
    //   }
    // } else if (!theme && window.matchMedia('(prefers-color-scheme: light)').matches) {
    //   document.documentElement.classList.remove('dark')
    //   if (e.target.value === 'light') {
    //     document.documentElement.classList.remove('dark')
    //   } else {
    //     document.documentElement.classList.add('dark')
    //   }
    // }

    // if (e.target.value === 'dark') {
    //   document.documentElement.classList.add('dark')
    // } else {
    //   document.documentElement.classList.remove('dark')
    // }
  }

  return (
    <div className='max-w-[180px]'>
      <label htmlFor='theme-switcher' className='sr-only'>
        Language
      </label>
      <div className='relative'>
        <div className='pointer-events-none absolute inset-y-0 left-0 flex items-center px-2.5'>
          <svg
            xmlns='http://www.w3.org/2000/svg'
            fill='none'
            viewBox='0 0 24 24'
            strokeWidth='1.5'
            stroke='currentColor'
            className='h-5 w-5 text-gray-500'
          >
            <path
              strokeLinecap='round'
              strokeLinejoin='round'
              d='M10.5 21l5.25-11.25L21 21m-9-3h7.5M3 5.621a48.474 48.474 0 016-.371m0 0c1.12 0 2.233.038 3.334.114M9 5.25V3m3.334 2.364C11.176 10.658 7.69 15.08 3 17.502m9.334-12.138c.896.061 1.785.147 2.666.257m-4.589 8.495a18.023 18.023 0 01-3.827-5.802'
            />
          </svg>
        </div>
        <select
          id='theme-switcher'
          className='block w-full rounded-md border-gray-300 pl-9 text-sm focus:border-blue-300 focus:ring focus:ring-blue-200 focus:ring-opacity-50 disabled:cursor-not-allowed disabled:bg-gray-50'
          onChange={handleSelect}
          defaultValue={theme}
        >
          <option disabled>Select theme</option>
          <option value='dark'>Dark</option>
          <option value='light'>Light</option>
          <option value='system'>System</option>
        </select>
      </div>
    </div>
  )
}
