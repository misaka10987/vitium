import { Router } from '@solidjs/router'
import { FileRoutes } from '@solidjs/start/router'
import { Suspense } from 'solid-js'
import { NavBar } from '~/components/NavBar'
import './app.css'
import { ColorMode } from './components/ColorMode'

export default function App() {
  const storageManager = cookieStorageManagerSSR(isServer ? getServerCookies() : document.cookie)
  return (
    <Router
      root={(props) => (
        <>
          <ColorMode>
            <Nav />
            <Suspense>{props.children}</Suspense>
          </ColorMode>
        </>
      )}
    >
      <FileRoutes />
    </Router>
  )
}