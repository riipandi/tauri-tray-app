import React from 'react'
import ReactDOM from 'react-dom/client'
import { attachConsole, info } from 'tauri-plugin-log-api'

import App from './app'
import './styles.css'

attachConsole() // Tauri logging integration
info('ui_client_loaded')

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
)
