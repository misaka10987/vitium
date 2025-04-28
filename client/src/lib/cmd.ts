import { hostStore } from '@/components/host'
import { userStore } from '@/components/user'
import { CommandLine } from 'vitium-api'
import { panic } from './util'
import { json } from 'typia'

export const sendCommand = async (line: string) => {
  const host = hostStore.getState().hostname ?? panic('Missing hostname')
  const user = userStore.getState().username ?? panic('Missing username')

  console.info(user)

  const res = await fetch(`https://${host}/api/cmd`, {
    method: 'POST',
    credentials: 'include',
    headers: {
      'Content-Type': 'application/json',
    },
    body: json.assertStringify<CommandLine>({ user, line }),
  })
  if (!res.ok) panic('HTTP error', res)
}
