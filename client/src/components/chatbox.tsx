'use client'

import React, { useState, useEffect, useRef } from 'react'
import { Textarea } from '@/components/ui/textarea'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Chatbubble } from '@/components/chatbubble'
import { Button } from '@/components/ui/button'
import { sendMessage, setSSEListener } from '@/lib/chat'
import {
  Send,
  // due to an eslint bug, it requires you to provide an `alt` prop
  // this is a walkaround
  Image as Photo,
} from 'lucide-react'
import { useHostStore } from '@/components/host'
import { useRouter } from 'next/navigation'
import { Message } from 'vitium-api'

export const Chatbox = () => {
  const msgForm = useRef<HTMLFormElement>(null)
  const msgInput = useRef<HTMLTextAreaElement>(null)
  const [messages, setMessages] = useState<Message[]>([])
  const { hostname } = useHostStore()
  const connectAttempts = useRef(0)
  const router = useRouter()

  useEffect(() => {
    if (!hostname) return

    let eventSource: EventSource | null = null

    const connectToSSE = () => {
      // The browser automatically sets Accept: text/event-stream for EventSource connections
      console.debug('Establishing SSE connection to', hostname)

      eventSource = new EventSource(`https://${hostname}/chat`)

      window.addEventListener('beforeunload', () => eventSource?.close())

      // Set up the SSE listener
      setSSEListener(eventSource, setMessages)

      // Reset attempts counter on successful connection
      eventSource.addEventListener('open', () => {
        connectAttempts.current = 0
      })

      // Handle reconnection if needed
      eventSource.addEventListener('error', () => {
        if (eventSource?.readyState === EventSource.CLOSED) {
          connectAttempts.current += 1
          console.debug(
            `Connection closed. Attempt ${connectAttempts.current}/3`
          )

          if (connectAttempts.current >= 3) {
            console.debug(
              'Maximum connection attempts reached. Redirecting to login.'
            )
            router.replace('/login')
          }

          setTimeout(connectToSSE, 3000) // Reconnect after 3 seconds
        }
      })
    }

    connectToSSE()

    return () => eventSource?.close()
  }, [hostname, router])

  return (
    <div className="flex flex-col h-full w-full">
      <div className="flex-1 flex flex-col">
        <ScrollArea
          className="h-[calc(100vh-180px)] flex w-full rounded-md border p-1 mb-2"
          type="auto"
          scrollHideDelay={100}
        >
          <div className="p-2">
            {messages.map((msg, index) => (
              <Chatbubble key={index} {...msg} />
            ))}
          </div>
        </ScrollArea>
        <form
          ref={msgForm}
          className="flex gap-2 items-center"
          onSubmit={(e) => {
            e.preventDefault()
            const form = new FormData(e.currentTarget)
            const msg = form.get('msg')?.toString()
            if (msg == undefined) throw Error()
            sendMessage(msg)
            if (msgInput?.current == null) throw Error()
            msgInput.current.value = ''
          }}
        >
          <Textarea
            ref={msgInput}
            name="msg"
            className="h-[90px] w-full resize-none overflow-auto py-2"
            placeholder="Type your message here..."
            required
            onKeyDown={(e) => {
              if (e.key === 'Enter' && !e.shiftKey) {
                e.preventDefault()
                msgForm.current?.requestSubmit()
              }
            }}
          />
          <div className="flex flex-col gap-1">
            <Button
              className="h-[40px] px-4"
              type="submit"
              aria-label="Send message"
            >
              <Send className="h-4 w-4" />
            </Button>
            <Button
              className="h-[40px] px-4 mt-1"
              type="button"
              variant="outline"
              aria-label="Upload image"
            >
              <Photo className="h-4 w-4" />
            </Button>
          </div>
        </form>
      </div>
    </div>
  )
}
