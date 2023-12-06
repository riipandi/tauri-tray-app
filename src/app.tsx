import { useCallback, useEffect, useState } from 'react'
import { type Platform, platform } from '@tauri-apps/api/os'
import { Route, Switch } from 'wouter'

import { TailwindIndicator } from './components/common'
import WindowControls from './components/ui-controls'
import { cn } from './libraries/utils'
import NotFoundScreen from './screens/not-found'
import SettingScreen from './screens/settings'
import WelcomeScreen from './screens/welcome'

export default function App() {
  const [osType, setOsType] = useState<Platform>('darwin')

  const fetchOsType = useCallback(async () => {
    setOsType(await platform())
  }, [])

  useEffect(() => {
    fetchOsType()
  }, [fetchOsType])

  return (
    <div className={cn('disable-select')}>
      <WindowControls
        platform='darwin'
        className={cn(
          osType === 'darwin' ? 'sticky' : 'hidden',
          'z-999 absolute top-0 h-7 w-full bg-transparent'
        )}
      />

      <Switch>
        <Route path='/'>
          <WelcomeScreen />
        </Route>
        <Route path='/settings'>
          <SettingScreen />
        </Route>
        <Route>
          <NotFoundScreen />
        </Route>
      </Switch>

      <TailwindIndicator />
    </div>
  )
}
