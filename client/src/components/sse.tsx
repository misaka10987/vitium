import { useEffect, useRef } from 'react'

/**
 * A React component for for handling an SSE endpoint.
 * 
 * Set `url` to `undefined` to shutdown the connection.
 * 
 * @param url the URL to subscribe to
 * @param downstream downstream handler for received events
 * @returns `null`
 */
export const SSE = ({ url, downstream }: { url?: string, downstream: (_: string) => void }) => {
    const es = useRef<EventSource>(null)

    const close = () => {
        es.current?.close()
        es.current = null
    }

    useEffect(() => {
        if (url == undefined) close()
        else es.current = new EventSource(url)
        return close
    }, [url])

    useEffect(() => {
        const handle = (evt: { data: string }) => downstream(evt.data)
        es.current?.addEventListener('message', handle)
        return () => es.current?.removeEventListener('message', handle)
    }, [url, downstream])

    return null
}
