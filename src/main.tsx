import React from 'react'
import ReactDOM from 'react-dom/client'
import { attachConsole } from 'tauri-plugin-log-api'

import App from './app'

import './styles.css'
// import './libraries/updater'

// Tauri logging integration
if (import.meta.env.DEV) {
  attachConsole()
}

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
)
