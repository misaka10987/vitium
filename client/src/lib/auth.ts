import { hostStore } from '@/components/host'

export const basicAuthHeader = (user: string, pass: string) => {
  const text = `${user}:${pass}`
  const encoded = Buffer.from(text).toString('base64')
  return {
    Authorization: `Basic ${encoded}`,
  }
}

export const grabToken = async (user: string, pass: string) => {
  const host = hostStore.getState().host
  const url = `https://${host}/api/auth`
  const header = basicAuthHeader(user, pass)
  console.debug(`Fetching token from ${url}`)

  const response = await fetch(url, {
    method: 'GET',
    headers: header,
    credentials: 'include',
  })

  return response
}
