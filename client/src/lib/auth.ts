import { hostStore } from '@/components/host'

export const basicAuthHeader = (user: string, pass: string) => {
  const text = `${user}:${pass}`
  const encoded = Buffer.from(text).toString('base64')
  return {
    Authorization: `Basic ${encoded}`,
  }
}

export const grabToken = async (user: string, pass: string) => {
  const hostname = hostStore.getState().hostname
  const url = `https://${hostname}/auth`
  const header = basicAuthHeader(user, pass)
  const res = await fetch(url, {
    method: 'GET',
    headers: header,
    credentials: 'include',
  })
  if (res.ok) console.debug(`Fetched token from ${url}`)
  return res.ok
}
