import { useEffect, useRef } from 'react'
import { useHostStore } from './host'
import { json } from 'typia'
import { Message } from 'vitium-api'

export function ChatSSE({ downstream }: { downstream: (_: Message) => void }) {
  const { host } = useHostStore()
  const es = useRef<EventSource>(null)

  useEffect(() => {
    if (host == undefined) return
    const url = `https://${host}/api/chat`
    es.current = new EventSource(url)
    return () => es.current?.close()
  }, [host])

  useEffect(() => {
    const handle = (evt: { data: string }) => {
      const data = json.assertParse<Message>(evt.data)
      downstream(data)
    }
    es.current?.addEventListener('message', handle)
    return () => es.current?.removeEventListener('message', handle)
  }, [host, downstream])

  return null
}
