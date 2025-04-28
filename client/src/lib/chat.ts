import { hostStore } from '@/components/host'
import { userStore } from '@/components/user'
import { Message } from 'vitium-api'
import { json } from 'typia'
import { panic } from './util'

export const setSSEListener = (
  es: EventSource,
  messagesDispatch: (_: (_: Message[]) => Message[]) => void
) => {
  es.addEventListener('message', (event) => {
    const data = json.assertParse<Message>(event.data)
    const username = userStore.getState().username
    // Add the received message to the messages state
    const update = (prev: Message[]) => [
      ...prev,
      {
        ...data,
        variant: data.sender === username ? 'send' : 'receive',
      },
    ]
    messagesDispatch(update)
  })

  es.addEventListener('error', console.error)
}

export const sendMessage = async (content: string, html: boolean = false) => {
  const host = hostStore.getState().hostname ?? panic('Missing hostname')
  const user = userStore.getState().username ?? panic('Missing username')

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
