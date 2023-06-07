import React from 'react'
import ReactDOM from 'react-dom/client'
import { HashRouter,Routes,Route } from "react-router-dom";

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
    <HashRouter basename={"/"}>
      <Routes>
        <Route path="/" element={<App />} />
      </Routes>
    </HashRouter>
  </React.StrictMode>
)
