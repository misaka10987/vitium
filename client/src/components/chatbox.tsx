'use client'

import { useState, useEffect, useRef } from 'react'
import { MessageBubble } from '@/components/message-bubble'
import { Message } from 'vitium-api'
import { TurboInput } from '@/components/turbo-input'
import { panic } from '@/lib/util'
import { ChatSSE } from './chat-sse'

export const Chatbox = () => {
  const [messages, setMessages] = useState<Message[]>([])
  const container = useRef<HTMLDivElement>(null)

  useEffect(() => {
    const curr = container.current ?? panic()
    curr.scrollTop = curr.scrollHeight
  }, [messages])

  return (
    <div className="flex flex-col-reverse h-full w-full gap-2">
      <ChatSSE
        downstream={(msg) => setMessages((prev) => prev.concat([msg]))}
      />
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
