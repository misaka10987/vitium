import { Router } from '@solidjs/router'
import { FileRoutes } from '@solidjs/start/router'
import { Suspense } from 'solid-js'
import Nav from '~/components/Nav'
import './app.css'
import { ColorMode } from './components/ColorMode'

export default function App() {
  return (
    <Router
      root={(props) => (
        <>
          <ColorMode>
            <div class="fixed left-0 top-0 right-0 bottom-0">
              <div class="flex flex-col h-full">
                <Nav />
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
