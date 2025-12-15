import { createSignal, For, onMount, onCleanup } from 'solid-js'
import { Button } from '~/components/ui/button'
import { Send, Image, Code } from 'lucide-solid'
import { cn } from '~/lib/utils'

interface Message {
  id: string
  content: string
  sender: string
  timestamp: Date
  isHtml?: boolean
}

export const Chat = () => {
  const [messages, setMessages] = createSignal<Message[]>([])
  const [inputValue, setInputValue] = createSignal('')
  const [htmlEnabled, setHtmlEnabled] = createSignal(false)
  let messagesEndRef: HTMLDivElement | undefined
  let chatContainerRef: HTMLDivElement | undefined

  // Scroll to bottom when new messages arrive
  const scrollToBottom = () => {
    if (messagesEndRef) {
      messagesEndRef.scrollIntoView({ behavior: 'smooth' })
    }
  }

  // Mock function to add messages (replace with actual send logic)
  const handleSend = () => {
    const message = inputValue().trim()
    if (!message) return

    const newMessage: Message = {
      id: crypto.randomUUID(),
      content: message,
      sender: 'You',
      timestamp: new Date(),
      isHtml: htmlEnabled()
    }

    setMessages([...messages(), newMessage])
    setInputValue('')
    setTimeout(scrollToBottom, 0)
  }

  const handleKeyPress = (e: KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault()
      handleSend()
    }
  }

  const handleImageUpload = () => {
    // TODO: Implement image upload
    console.log('Image upload clicked')
  }

  const renderMessage = (message: Message) => {
    if (message.isHtml && htmlEnabled()) {
      return <div innerHTML={message.content} />
    }
    return <p class="whitespace-pre-wrap break-words">{message.content}</p>
  }

  return (
    <div
      ref={chatContainerRef}
      class="flex-col flex-1 bg-background border-l border-border"
      style={{ height: 'calc(100vh - 3.5rem)' }} // 3.5rem is approximate navbar height
    >
      {/* Messages Area */}
      <div class="flex-1 overflow-y-auto p-4 space-y-4">
        <For each={messages()}>
          {(message) => (
            <div class="flex flex-col gap-1">
              <div class="flex items-baseline gap-2">
                <span class="text-sm font-semibold text-primary">{message.sender}</span>
                <span class="text-xs text-muted-foreground">
                  {message.timestamp.toLocaleTimeString()}
                </span>
              </div>
              <div class="text-sm text-foreground bg-muted/50 rounded-lg p-3 max-w-[85%]">
                {renderMessage(message)}
              </div>
            </div>
          )}
        </For>
        <div ref={messagesEndRef} />
      </div>

      {/* Input Area */}
      <div class="border-t border-border p-4 bg-card">
        <div class="flex gap-2 items-end">
          {/* HTML Enable Button */}
          <Button
            variant={htmlEnabled() ? 'default' : 'outline'}
            size="icon"
            onClick={() => setHtmlEnabled(!htmlEnabled())}
            title={htmlEnabled() ? 'Disable HTML' : 'Enable HTML'}
            class="shrink-0"
          >
            <Code class="w-4 h-4" />
          </Button>

          {/* Input Box */}
          <div class="flex-1 relative">
            <textarea
              value={inputValue()}
              onInput={(e) => setInputValue(e.currentTarget.value)}
              onKeyPress={handleKeyPress}
              placeholder="Type a message..."
              class={cn(
                "w-full px-3 py-2 pr-10 rounded-md resize-none",
                "bg-input border border-input",
                "text-foreground placeholder:text-muted-foreground",
                "focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2",
                "disabled:opacity-50 disabled:cursor-not-allowed",
                "min-h-10 max-h-32"
              )}
              rows={1}
              style={{
                'field-sizing': 'content'
              }}
            />
          </div>

          {/* Picture Button */}
          <Button
            variant="outline"
            size="icon"
            onClick={handleImageUpload}
            title="Upload Image"
            class="shrink-0"
          >
            <Image class="w-4 h-4" />
          </Button>

          {/* Send Button */}
          <Button
            onClick={handleSend}
            disabled={!inputValue().trim()}
            title="Send Message"
            class="shrink-0"
          >
            <Send class="w-4 h-4" />
            <span class="hidden sm:inline">Send</span>
          </Button>
        </div>
      </div>
    </div>
  )
}
