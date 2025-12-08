import { Router } from '@solidjs/router'
import { FileRoutes } from '@solidjs/start/router'
import { Suspense } from 'solid-js'
import { NavBar } from '~/components/NavBar'
import './app.css'
import { ColorMode } from './components/ColorMode'

export default function App() {
  return (
    <Router
      root={(props) => (
        <>
          <ColorMode>
            <div class="flex flex-col min-h-screen">
              <NavBar />
              <div class="flex-1 flex flex-col">
                <Suspense>{props.children}</Suspense>
              </div>
            </div>
          </ColorMode>
        </>
      )}
    >
      <FileRoutes />
    </Router>
  )
}