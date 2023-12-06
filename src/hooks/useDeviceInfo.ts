import { useCallback, useEffect, useState } from 'react'
import type { Arch, OsType, Platform } from '@tauri-apps/api/os'
import { arch, platform, type } from '@tauri-apps/api/os'
import { invoke } from '@tauri-apps/api/tauri'
import { useWindowSize } from 'usehooks-ts'

export default function useDeviceInfo() {
  const [platformType, setPlatformType] = useState<Platform>()
  const [osType, setOsType] = useState<OsType>()
  const [archType, setArchType] = useState<Arch>()
  const [machineId, setMachineId] = useState<string>()

  const windowSize = useWindowSize()
  const fetchDeviceInfo = useCallback(async () => {
    setPlatformType(await platform())
    setArchType(await arch())
    setOsType(await type())
  }, [])

  const fetchExtraInfo = async () => {
    const id = await invoke<string>('get_machine_id')
    setMachineId(id)
  }

  useEffect(() => {
    fetchDeviceInfo()
    fetchExtraInfo()
  }, [fetchDeviceInfo])

  return { platform: platformType, os: osType, arch: archType, machineId, windowSize }
}
