import { useEffect } from 'react'
import { useHostStore } from './host'
import { json } from 'typia'
import { Message } from 'vitium-api'

export function ChatSSE({ downstream }: { downstream: (_: Message) => void }) {
  const { host } = useHostStore()
  useEffect(() => {
    if (host == undefined) return
    const url = `https://${host}/api/chat`
    const es = new EventSource(url)
    es.addEventListener('message', (evt) => {
      const data = json.assertParse<Message>(evt.data)
      downstream(data)
    })
    return () => es.close()
  }, [host, downstream])
  return undefined
}
