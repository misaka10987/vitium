import { Textarea } from '@/components/ui/textarea'
import { sendMessage } from '@/lib/chat'
import { useRef, useState } from 'react'
import { Button } from '@/components/ui/button'
import { Send, Code2, ChevronsRight } from 'lucide-react'
import { panic } from '@/lib/util'
import { cn } from '@/lib/utils'
import { sendCommand } from '@/lib/cmd'
import { HoverCard, HoverCardContent, HoverCardTrigger } from './ui/hover-card'

export const TurboInput = () => {
  const msgForm = useRef<HTMLFormElement>(null)
  const msgInput = useRef<HTMLTextAreaElement>(null)
  const [isCommand, setIsCommand] = useState(false)
  const [enableHTML, setEnableHTML] = useState(false)
  return (
    <form
      ref={msgForm}
      className="flex gap-2 items-center w-full"
      onSubmit={(e) => {
        e.preventDefault()
        const form = new FormData(e.currentTarget)
        const content = form.get('content')?.toString() ?? panic()
        if (isCommand) {
          sendCommand(content.substring(1))
        } else {
          sendMessage(content, enableHTML)
        }
        const curr = msgInput?.current ?? panic()
        curr.value = ''
        setIsCommand(false)
      }}
    >
      <Textarea
        ref={msgInput}
        name="content"
        className={cn(
          'h-full resize-none overflow-auto py-2',
          isCommand && 'font-bold font-mono'
        )}
        placeholder="Type your message here..."
        required
        onChange={(e) => setIsCommand(e.currentTarget.value.startsWith(':'))}
        onKeyDown={(e) => {
          if (e.key == 'Enter' && !e.shiftKey) {
            e.preventDefault()
            msgForm.current?.requestSubmit()
          }
        }}
      />
      <div className="flex flex-col gap-2">
        <div className="flex flex-1/2">
          <HoverCard>
            <HoverCardTrigger asChild>
              <Button
                className="h-10 w-10 ease-in-out"
                type="submit"
                aria-label="Send message"
              >
                {isCommand ? <ChevronsRight /> : <Send />}
              </Button>
            </HoverCardTrigger>
            <HoverCardContent className="w-fit h-fit py-2 px-3">
              <span className="text-sm">
                {isCommand ? 'Send Command' : 'Send Message'}
              </span>
            </HoverCardContent>
          </HoverCard>
        </div>
        <div className="flex">
          <HoverCard>
            <HoverCardTrigger asChild>
              <Button
                className="h-10 w-10 ease-in-out"
                type="button"
                variant={enableHTML ? 'default' : 'secondary'}
                onClick={() => setEnableHTML((enabled) => !enabled)}
              >
                <Code2 />
              </Button>
            </HoverCardTrigger>
            <HoverCardContent className="w-fit h-fit py-2 px-3">
              <span className="text-sm">Enable HTML</span>
            </HoverCardContent>
          </HoverCard>
        </div>
      </div>
    </form>
  )
}
