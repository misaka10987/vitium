'use client'

import { useState, useEffect, useRef } from 'react'
import { MessageBubble } from '@/components/message-bubble'
import { setSSEListener } from '@/lib/chat'
import { useHostStore } from '@/components/host'
import { useRouter } from 'next/navigation'
import { Message } from 'vitium-api'
import { TurboInput } from '@/components/turbo-input'
import { panic } from '@/lib/util'

export const Chatbox = () => {
  const [messages, setMessages] = useState<Message[]>([])
  const { hostname } = useHostStore()
  const connectAttempts = useRef(0)
  const router = useRouter()
  const container = useRef<HTMLDivElement>(null)

  useEffect(() => {
    const curr = container.current ?? panic()
    curr.scrollTop = curr.scrollHeight
  }, [messages])

  useEffect(() => {
    if (!hostname) return

    let eventSource: EventSource | null = null

    const subscribe = () => {
      // The browser automatically sets Accept: text/event-stream for EventSource connections
      eventSource = new EventSource(`https://${hostname}/api/chat`)

      window.addEventListener('beforeunload', () => eventSource?.close())

      // Set up the SSE listener
      setSSEListener(eventSource, setMessages)

      // Reset attempts counter on successful connection
      eventSource.addEventListener('open', () => {
        connectAttempts.current = 0
      })

      // Handle reconnection if needed
      eventSource.addEventListener('error', () => {
        if (eventSource?.readyState == EventSource.CLOSED) {
          connectAttempts.current += 1
          console.debug(
            `Connection closed. Reconnect attempt ${connectAttempts.current}/3`
          )

          if (connectAttempts.current >= 3) {
            console.debug(
              'Max reconnection attempt reached, redirecting to login'
            )
            eventSource?.close()
            router.replace('/login')
            return
          }

          setTimeout(subscribe, 3000) // Reconnect after 3 seconds
        }
      })
    }

    subscribe()

    return () => eventSource?.close()
  }, [hostname, router])

  return (
    <div className="flex flex-col-reverse h-full w-full gap-2">
      <div className="flex w-full">
        <TurboInput />
      </div>
      <div className="flex flex-grow h-0 w-full rounded-md border">
        <div
          ref={container}
          className="flex flex-col p-2 gap-2 w-full overflow-auto"
        >
          {messages.map((msg, index) => (
            <div className="flex w-full" key={index}>
              <MessageBubble {...msg} />
            </div>
          ))}
        </div>
      </div>
    </div>
  )
}
