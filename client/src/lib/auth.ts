import { useHostStore } from '@/components/host'
import { SignUp } from 'vitium-api'

/**
 * Generate HTTP header for [Basic](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Authorization#basic_authentication) authorization.
 *
 * @param user username
 * @param pass password
 * @returns an object representing the header map
 */
export const basicAuthHeader = (user: string, pass: string) => {
  const text = `${user}:${pass}`
  const encoded = Buffer.from(text).toString('base64')
  return {
    Authorization: `Basic ${encoded}`,
  }
}

/**
 * Attempt to login to the game server with specified username and password.
 * @param user username
 * @param pass password
 * @returns response for this login request
 */
export const login = async (user: string, pass: string) => {
  const host = useHostStore.getState().host
  const url = `https://${host}/api/auth`
  const header = basicAuthHeader(user, pass)
  console.debug(`Fetching token from ${url}`)

  const res = await fetch(url, {
    method: 'GET',
    headers: header,
    credentials: 'include',
  })

  return res
}

/**
 * Attempt to sign up to the game server with specified username and password.
 * @param signupInfo object containing user and pass
 * Submits in html-form format.
 * @returns response for this signup request
 * @deprecated
 */
export const signup = async (signupInfo: SignUp) => {
  const host = useHostStore.getState().host
  const url = `https://${host}/api/auth`

  const res = await fetch(url, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded',
    },
    body: new URLSearchParams({
      user: signupInfo.user,
      pass: signupInfo.pass,
    }),
    credentials: 'include',
  })

  return res
}
