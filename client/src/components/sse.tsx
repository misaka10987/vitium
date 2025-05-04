import { useEffect, useRef } from 'react'
import { useHostStore } from './host'

export function SSE({ path, downstream }: { path: string, downstream: (_: string) => void }) {
    const { host } = useHostStore()
    const es = useRef<EventSource>(null)
    const url = `https://${host}${path}`

    useEffect(() => {
        if (host == undefined) return
        es.current = new EventSource(url)
        return () => es.current?.close()
    }, [host, url])

    useEffect(() => {
        const handle = (evt: { data: string }) => downstream(evt.data)
        es.current?.addEventListener('message', handle)
        return () => es.current?.removeEventListener('message', handle)
    }, [url, downstream])

    return null
}
