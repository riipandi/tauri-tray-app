import { useEffect, useState } from 'react'
import { platform, type Platform } from '@tauri-apps/api/os'
import { Route, Switch } from 'wouter'

import { cn, disableBrowserEvents } from './libraries/utils'
import WindowControls from './components/ui-controls'
import WelcomeScreen from './screens/welcome'
import SettingScreen from './screens/settings'
import NotFoundScreen from './screens/not-found'
import { TailwindIndicator } from './components/common'

export default function App() {
  const [osType, setOsType] = useState<Platform>('darwin')

  useEffect(() => {
    disableBrowserEvents('contextmenu')
    async function fetchOsType() {
      setOsType(await platform())
    }
    fetchOsType()
  }, [platform, disableBrowserEvents])

  return (
    <div className={cn('disable-select')}>
      <WindowControls
        platform='darwin'
        className={cn(
          osType === 'darwin' ? 'sticky' : 'hidden',
          'w-full bg-transparent h-7 z-999 absolute top-0'
        )}
      />

      <Switch>
        <Route path='/' children={<WelcomeScreen />} />
        <Route path='/settings' children={<SettingScreen />} />
        <Route children={<NotFoundScreen />} />
      </Switch>

      <TailwindIndicator />
    </div>
  )
}
