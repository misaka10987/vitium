'use client'

import { useEffect, useRef, useCallback } from 'react'
import { CommandRecord, Message } from 'vitium-api'
import { TurboInput } from '@/components/turbo-input'
import { panic, defined } from '@/lib/util'
import { Bubble, Entry } from './bubble'
import { SSE } from './sse'
import { json } from 'typia'
import { useHostStore } from './host'
import { create } from 'zustand'
import { createJSONStorage, persist } from 'zustand/middleware'

/**
 * React hook for accessing the chat record stored.
 */
export const useChatStore = create<{
  /**
   * The chat record.
   */
  chat: Entry[]

  /**
   * Add a new message to the end of chat record.
   *
   * @param entry the new message
   */
  addNew: (entry: Entry) => void

  /**
   * Clear the chat record, removing all entries.
   */
  clear: () => void
}>()(
  persist(
    (set) => ({
      chat: [],
      addNew: (entry) => set(({ chat }) => ({ chat: [...chat, entry] })),
      clear: () => set(() => ({ chat: [] })),
    }),
    {
      name: 'chat',
      // uses session storage
      storage: createJSONStorage(() => sessionStorage),
    }
  )
)

/**
 * The chat column of the UI, including display and text input.
 */
export const Chat = () => {
  const { chat, addNew } = useChatStore()

  const container = useRef<HTMLDivElement>(null)
  useEffect(() => {
    const curr = container.current ?? panic()
    curr.scrollTop = curr.scrollHeight
  }, [chat])

  const { host } = useHostStore()

  const msg = useCallback(
    (data: string) => addNew(json.assertParse<Message>(data)),
    [addNew]
  )

  const cmd = useCallback(
    (data: string) => addNew(json.assertParse<CommandRecord>(data)),
    [addNew]
  )

  return (
    <div className="flex flex-col-reverse h-full w-full gap-2">
      <SSE url={defined`https://${host}/api/chat`} downstream={msg} />
      <SSE url={defined`https://${host}/api/cmd`} downstream={cmd} />
      <div className="flex w-full">
        <TurboInput />
      </div>
      <div className="flex flex-grow h-0 w-full rounded-md border">
        <div
          ref={container}
          className="flex flex-col p-2 gap-2 w-full overflow-auto"
        >
          {chat.map((entry, idx) => (
            <div className="flex w-full" key={idx}>
              <Bubble entry={entry} />
            </div>
          ))}
        </div>
      </div>
    </div>
  )
}
