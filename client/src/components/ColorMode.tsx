import {
  ColorModeProvider,
  ColorModeScript,
  cookieStorageManagerSSR,
} from '@kobalte/core'
import { ParentProps } from 'solid-js'
import { isServer } from 'solid-js/web'
import { getCookie } from 'vinxi/http'

const getServerCookies = () => {
  'use server'
  const colorMode = getCookie('kb-color-mode')
  return colorMode ? `kb-color-mode=${colorMode}` : ''
}

export const ColorMode = (props: ParentProps) => {
  const storageManager = cookieStorageManagerSSR(
    isServer ? getServerCookies() : document.cookie
  )

  return (
    <>
      <ColorModeScript storageType={storageManager.type} />
      <ColorModeProvider storageManager={storageManager}>
        {props.children}
      </ColorModeProvider>
    </>
  )
}
