import React from 'react'
import ReactDOM from 'react-dom/client'
import { Route, Switch } from 'wouter'

import App from './App'
import './styles.css'

/**
 * The recommended approach for routing in React is HashRouter.
 * HashRouter is for use in web browsers when the URL should not
 * (or cannot) be sent to the server for some reason.
 * Ref: https://reactrouter.com/en/main/router-components/hash-router
 */
ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <Switch>
      <Route path='/' children={<App />} />
      <Route>404, Not Found!</Route>
    </Switch>
  </React.StrictMode>
)
