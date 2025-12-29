import { For, createEffect, createSignal } from 'solid-js'
import { ChatInput } from '~/components/ChatInput'
import { PureDOM } from '~/components/PureDOM'
import { fetchRecentMessages, subscribeChatSSE, Message } from '~/lib/chat'


export const Chat = () => {
  const [messages, setMessages] = createSignal<Message[]>([
    { id: 1, author: 'System', content: 'Welcome to Vitium.' },
  ])

  // Fetch messages from the last 10 minutes on mount
  createEffect(() => {
    const now = Date.now()
    const tenMinutesAgo = now - 10 * 60 * 1000
    fetchRecentMessages(tenMinutesAgo)
      .then(setMessages)
      .catch((err) => {
        console.warn('Failed to fetch recent chat messages:', err)
      })
  })

  // Subscribe to SSE for real-time chat
  createEffect(() => {
    const unsubscribe = subscribeChatSSE((msg) => {
      setMessages((prev) => {
        // if (prev.some((m) => m.id === msg.id)) return prev // This would check if there is a duplicate
        return [...prev, msg]
      })
    })
    return unsubscribe
  })

  return (
    <div class="w-1/4 h-full">
      <div class="flex h-full flex-col overflow-hidden rounded-lg border bg-card shadow-sm">
        <div class="flex-1 space-y-3 overflow-y-auto p-3">
          <For each={messages()}>
            {(msg) => (
              <div class="rounded-md border border-border/60 bg-muted/50 p-2 text-sm">
                <div class="text-xs font-semibold text-muted-foreground">{msg.author}</div>
                {msg.htmlEnabled ? (
                  <PureDOM html={msg.content} />
                ) : (
                  <div>{msg.content}</div>
                )}
              </div>
            )}
          </For>
        </div>
        <div class="border-t border-border bg-background p-2">
          <ChatInput />
        </div>
      </div>
    </div>
  )
}
