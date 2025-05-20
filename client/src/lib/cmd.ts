import { hostStore } from '@/components/host'
import { userStore } from '@/components/user'
import { CommandLine } from 'vitium-api'
import { panic } from './util'
import { json } from 'typia'

/**
 * Send a chat message to the game server with current username.
 *
 * @param line the command
 */
export const sendCommand = async (line: string) => {
  const host = hostStore.getState().host ?? panic('Missing hostname')
  const user = userStore.getState().user ?? panic('Missing username')

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
