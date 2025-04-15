import { Chatbubble } from '@/components/chatbubble'
import { hostStore } from '@/components/host'
import { username } from '@/components/user'

function unwrapMessage(msgEvent: MessageEvent) {
  const data = JSON.parse(msgEvent.data)
  return Chatbubble({
    author: data.sender,
    timestamp: data.time,
    message: data.content,
  })
}

export function setSSEListener(
  es: EventSource,
  messagesDispatch: React.Dispatch<React.SetStateAction<any[]>>
) {
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
            data.sender === username.getState().name ? 'send' : 'receive',
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

export function sendMessage(message: string) {
  console.log('Sending message:', message)
  const hostname = hostStore.getState().hostname
  console.log('to: ', hostname)
  if (typeof hostname === 'undefined' || hostname === '') {
    console.error('Hostname is not set.')
    return
  }
  if (
    typeof username.getState().name === 'undefined' ||
    username.getState().name === ''
  ) {
    console.error('Username is not set.')
    return
  }
  fetch(`https://${hostname}/chat`, {
    method: 'POST',
    credentials: 'include',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      time: Date.now(),
      sender: username.getState().name,
      content: message,
      html: false,
    }),
  }).then(
    (response) => {
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`)
      }
      console.debug('Message sent successfully')
    },
    (error) => {
      console.error('Error sending message:', error)
    }
  )
}

export function sendImage() {
  console.log('Uploading image...')
  console.log('Image upload not implemented yet.')
  // Implement your image upload logic here
}
