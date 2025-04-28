import { hostStore } from '@/components/host'
import { userStore } from '@/components/user'
import { Message } from 'vitium-api'
import { json } from 'typia'
import { panic } from './util'

export const sendMessage = async (content: string, html: boolean = false) => {
  const host = hostStore.getState().host ?? panic('Missing hostname')
  const user = userStore.getState().user ?? panic('Missing username')

  const res = await fetch(`https://${host}/api/chat`, {
    method: 'POST',
    credentials: 'include',
    headers: {
      'Content-Type': 'application/json',
    },
    body: json.assertStringify<Message>({
      time: Date.now(),
      sender: user,
      content,
      html,
    }),
  })
  if (!res.ok) panic('HTTP error', res)
}
