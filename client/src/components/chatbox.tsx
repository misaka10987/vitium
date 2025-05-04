'use client'

import { useState, useEffect, useRef, useCallback } from 'react'
import { CommandRecord, Message } from 'vitium-api'
import { TurboInput } from '@/components/turbo-input'
import { panic } from '@/lib/util'
import { Bubble } from './bubble'
import { SSE } from './sse'
import { json } from 'typia'

export const Chatbox = () => {
  type Entry = Message | CommandRecord
  const [entries, setEntries] = useState<Entry[]>([])

  const container = useRef<HTMLDivElement>(null)

  useEffect(() => {
    const curr = container.current ?? panic()
    curr.scrollTop = curr.scrollHeight
  }, [entries])

  const handle = useCallback((incoming: Entry) => setEntries((prev) => prev.concat([incoming])), [setEntries])
  const chat = useCallback((data: string) => handle(json.assertParse<Message>(data)), [handle])
  const cmd = useCallback((data: string) => handle(json.assertParse<CommandRecord>(data)), [handle])

  return (
    <div className="flex flex-col-reverse h-full w-full gap-2">
      <SSE path="/api/chat" downstream={chat} />
      <SSE path="/api/cmd" downstream={cmd} />
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
