import { Chatbubble } from '@/components/chatbubble'
import { hostStore } from '@/components/host'
import { useUserStore } from '@/components/user'

function unwrapMessage(msgEvent: MessageEvent) {
  const data = JSON.parse(msgEvent.data)
  return Chatbubble({
    author: data.sender,
    timestamp: data.time,
    message: data.content,
  })
}

export const setSSEListener = (
  es: EventSource,
  messagesDispatch: React.Dispatch<React.SetStateAction<any[]>>
) => {
  es.addEventListener('message', (event) => {
    try {
      const data = JSON.parse(event.data)
      // Add the received message to the messages state
      messagesDispatch((prevMessages) => [
        ...prevMessages,
        {
          author: data.sender,
          timestamp: data.time,
          message: data.content,
          variant:
            data.sender === useUserStore.getState().username
              ? 'send'
              : 'receive',
          html: data.html || false,
        },
      ])
    } catch (error) {
      console.error('Error parsing SSE message:', error)
    }
  })

  es.addEventListener('error', (event) => {
    console.error('Error in SSE:', event)
    es.close()
  })
}

export const sendMessage = async (message: string) => {
  console.info('Sending chat message: ', message)
  const hostname = hostStore.getState().hostname
  if (typeof hostname === 'undefined' || hostname === '') {
    console.error('Hostname is not set.')
    return
  }
  if (
    typeof useUserStore.getState().username === 'undefined' ||
    useUserStore.getState().username === ''
  ) {
    console.error('Username is not set.')
    return
  }
  const res = await fetch(`https://${hostname}/chat`, {
    method: 'POST',
    credentials: 'include',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      time: Date.now(),
      sender: useUserStore.getState().username,
      content: message,
      html: false,
    }),
  })
  if (!res.ok) {
    console.error('HTTP error sending message:', res)
  }
  console.debug('Message sent successfully')
}

export const sendImage = () => {
  console.log('Uploading image...')
  console.log('Image upload not implemented yet.')
  // Implement your image upload logic here
}
