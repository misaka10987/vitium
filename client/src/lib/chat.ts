import { serverAddress, userName } from '~/lib/auth'

// Message type for chat
export type Message = {
  id: number
  author: string
  content: string
  htmlEnabled?: boolean
}

// Fetch chat messages after a given timestamp (ms)
export async function fetchRecentMessages(after: number): Promise<Message[]> {
  const url = `${serverAddress().toString()}/api/chat?after=${after}`
  const res = await fetch(url)
  if (!res.ok) throw new Error('Failed to fetch chat messages')
  return await res.json()
}

// Subscribe to chat messages via SSE
// onMessage: callback for each new message
// Returns: unsubscribe function
export function subscribeChatSSE(onMessage: (msg: Message) => void): () => void {
  const es = new window.EventSource(`${serverAddress().toString()}/api/chat`)
  es.onmessage = (event) => {
    try {
      const msg: Message = JSON.parse(event.data)
      onMessage(msg)
    } catch (e) {
      console.warn('Failed to parse SSE chat message:', e)
    }
  }
  es.onerror = (err) => {
    console.warn('SSE connection error:', err)
    es.close()
  }
  return () => es.close()
}

export async function sendMessage(content: string, isHtml: boolean) {
  try {
    const res = await fetch(new URL("/api/chat", serverAddress()),
      {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(
          {
            sender: userName(),
            time: Math.floor(Date.now() / 1000),
            content: content,
            html: isHtml
          }
        )
      }
    );
    if (!res.ok) {
      throw new Error(`Error sending message: ${res.statusText}`);
    }
  } catch (error) {
    console.error(error);
  }
}