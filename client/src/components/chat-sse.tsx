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
    if (es.current == null) return
    es.current.onmessage = (evt) => {
      const data = json.assertParse<Message>(evt.data)
      downstream(data)
    }
    return () => {
      if (es.current != null) es.current.onmessage = () => {}
    }
  }, [es, downstream])

  return null
}
