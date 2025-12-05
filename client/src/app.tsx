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
