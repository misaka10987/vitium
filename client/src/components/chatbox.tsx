'use client'

import { useState, useEffect, useRef, useCallback } from 'react'
import { CommandLine, Message } from 'vitium-api'
import { TurboInput } from '@/components/turbo-input'
import { panic } from '@/lib/util'
import { ChatSSE } from './chat-sse'
import { Bubble } from './bubble'

export const Chatbox = () => {
  type Entry = Message | CommandLine
  const [entries, setEntries] = useState<Entry[]>([])
  const container = useRef<HTMLDivElement>(null)

  useEffect(() => {
    const curr = container.current ?? panic()
    curr.scrollTop = curr.scrollHeight
  }, [entries])

  const handle = useCallback((incoming: Entry) => setEntries((prev) => prev.concat([incoming])), [setEntries])

  return (
    <div className="flex flex-col-reverse h-full w-full gap-2">
      <ChatSSE downstream={handle} />
      <div className="flex w-full">
        <TurboInput />
      </div>
      <div className="flex flex-grow h-0 w-full rounded-md border">
        <div
          ref={container}
          className="flex flex-col p-2 gap-2 w-full overflow-auto"
        >
          {entries.map((content, idx) => (
            <div className="flex w-full" key={idx}>
              <Bubble content={content} />
            </div>
          ))}
        </div>
      </div>
    </div>
  )
}
