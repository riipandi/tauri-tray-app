import { useCallback, useEffect, useState } from 'react'
import { type as getOSType, OsType } from '@tauri-apps/api/os'
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
  const [osType, setOsType] = useState<OsType>('Darwin')

  const windows = <Windows data-tauri-drag-region className={className} {...props} />
  const macos = <MacOS data-tauri-drag-region className={className} {...props} />
  const gnome = <Gnome data-tauri-drag-region className={className} {...props} />

  const fetchOsType = useCallback(async () => {
    setOsType(await getOSType())
  }, [])

  useEffect(() => {
    fetchOsType()
  }, [fetchOsType])

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
        case 'Windows_NT':
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
