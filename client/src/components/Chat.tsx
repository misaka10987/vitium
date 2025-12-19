import { For } from 'solid-js'
import { ChatInput } from '~/components/ChatInput'
import { PureDOM } from '~/components/PureDOM'

type Message = {
  id: number
  author: string
  content: string
  htmlEnabled?: boolean
}

export const Chat = () => {
  const messages: Message[] = [
    { id: 1, author: 'System', content: 'Welcome to the realm.' },
    { id: 2, author: 'Guide', content: 'Type a command or send a message below.' },
    // Example HTML-enabled message:
    { id: 3, author: 'Admin', content: '<b>Hello</b> <i>world</i>!', htmlEnabled: true },
    { id: 4, author: 'User', content: 'This is a plain text message.' },
    { id: 5, author: 'Moderator', content: '<script>alert("XSS Attack!")</script> This should be safe.', htmlEnabled: true },
    { id: 6, author: 'User2', content: 'Just another message.' },
    { id: 7, author: 'User3', content: '<div style="color: red;">This is red text.</div>', htmlEnabled: true },
    { id: 8, author: 'User4', content: 'Goodbye!' },
    { id: 9, author: 'System', content: 'yet another system message' },
    { id: 10, author: 'Guide', content: 'Notice that the commands start with \'/\' and is not documented' },
    { id: 11, author: 'Admin', content: '<u>Underlined text</u> is also supported.', htmlEnabled: true },
  ]

  return (
    <div class="w-1/4 h-full">
      <div class="flex h-full flex-col overflow-hidden rounded-lg border bg-card shadow-sm">
        <div class="flex-1 space-y-3 overflow-y-auto p-3">
          <For each={messages}>
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
