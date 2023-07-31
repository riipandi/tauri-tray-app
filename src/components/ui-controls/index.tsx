import { useEffect, useState } from 'react'
import { type as OSTypes } from '@tauri-apps/plugin-os'
import { MacOS, Gnome, Windows } from './controls'

export interface WindowControlsProps {
  platform?: 'windows' | 'darwin' | 'gnome'
  className?: string
  hideButtons?: boolean
}

const WindowControls = ({
  platform,
  hideButtons = false,
  className,
  ...props
}: WindowControlsProps) => {
  const [osType, setOsType] = useState('')

  const windows = <Windows data-tauri-drag-region className={className} {...props} />
  const macos = <MacOS data-tauri-drag-region className={className} {...props} />
  const gnome = <Gnome data-tauri-drag-region className={className} {...props} />

  useEffect(() => {
    async function fetchOsType() {
      const typeResult = await OSTypes()
      setOsType(typeResult)
      console.log(typeResult)
    }
    fetchOsType()
  }, [platform])

  // Check the platform and render the appropriate controls
  switch (platform) {
    case 'windows':
      return windows
    case 'darwin':
      return macos
    case 'gnome':
      return gnome
    default:
      switch (osType) {
        case 'Windows_NT' && 'Linux_NT':
          return windows
        case 'Darwin':
          return macos
        case 'Linux':
          return gnome
        default:
          return macos
      }
  }
}

export default WindowControls
