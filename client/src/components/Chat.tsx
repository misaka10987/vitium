import { For } from 'solid-js'
import { ChatInput } from '~/components/ChatInput'

type Message = {
  id: number
  author: string
  content: string
}

export const Chat = () => {
  const messages: Message[] = [
    { id: 1, author: 'System', content: 'Welcome to the realm.' },
    { id: 2, author: 'Guide', content: 'Type a command or send a message below.' },
  ]

  return (
    <div class="w-1/4 h-full">
      <div class="flex h-full flex-col overflow-hidden rounded-lg border bg-card shadow-sm">
        <div class="flex-1 space-y-3 overflow-y-auto p-3">
          <For each={messages}>
            {(msg) => (
              <div class="rounded-md border border-border/60 bg-muted/50 p-2 text-sm">
                <div class="text-xs font-semibold text-muted-foreground">{msg.author}</div>
                <div>{msg.content}</div>
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
