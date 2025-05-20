'use client'

import { useState, useEffect, useRef, useCallback } from 'react'
import { CommandRecord, Message } from 'vitium-api'
import { TurboInput } from '@/components/turbo-input'
import { panic, defined } from '@/lib/util'
import { Bubble, Entry } from './bubble'
import { SSE } from './sse'
import { json } from 'typia'
import { useHostStore } from './host'

/**
 * The chat column of the UI, including display and text input.
 */
export const Chat = () => {
  const [entries, setEntries] = useState<Entry[]>([])

  const container = useRef<HTMLDivElement>(null)
  useEffect(() => {
    const curr = container.current ?? panic()
    curr.scrollTop = curr.scrollHeight
  }, [entries])

  const { host } = useHostStore()

  const handle = useCallback(
    (incoming: Entry) => setEntries((prev) => prev.concat([incoming])),
    [setEntries]
  )
  const chat = useCallback(
    (data: string) => handle(json.assertParse<Message>(data)),
    [handle]
  )
  const cmd = useCallback(
    (data: string) => handle(json.assertParse<CommandRecord>(data)),
    [handle]
  )

  return (
    <div className="flex flex-col-reverse h-full w-full gap-2">
      <SSE url={defined`https://${host}/api/chat`} downstream={chat} />
      <SSE url={defined`https://${host}/api/cmd`} downstream={cmd} />
      <div className="flex w-full">
        <TurboInput />
      </div>
      <div className="flex flex-grow h-0 w-full rounded-md border">
        <div
          ref={container}
          className="flex flex-col p-2 gap-2 w-full overflow-auto"
        >
          {entries.map((entry, idx) => (
            <div className="flex w-full" key={idx}>
              <Bubble entry={entry} />
            </div>
          ))}
        </div>
      </div>
    </div>
  )
}
