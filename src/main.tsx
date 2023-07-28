import React from 'react'
import ReactDOM from 'react-dom/client'
import { Route, Switch } from 'wouter'

import App from './app'
import './styles.css'

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <Switch>
      <Route path='/' children={<App />} />
      <Route>404, Not Found!</Route>
    </Switch>
  </React.StrictMode>
)
