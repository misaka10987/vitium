import { hostStore } from '@/components/host'
import { userStore } from '@/components/user'
import { Message } from 'vitium-api'
import { json } from 'typia'

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

export const sendMessage = async (message: string) => {
  console.info('Sending chat message: ', message)
  const hostname = hostStore.getState().hostname
  if (!hostname?.trim()) {
    console.error('Hostname not set')
    return
  }
  const username = userStore.getState().username
  if (!username?.trim()) {
    console.error('Username not set')
    return
  }
  const res = await fetch(`https://${hostname}/api/chat`, {
    method: 'POST',
    credentials: 'include',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      time: Date.now(),
      sender: username,
      content: message,
      html: false,
    }),
  })
  if (!res.ok) {
    console.error('HTTP error sending message:', res)
  }
  console.debug('Message sent successfully')
}
