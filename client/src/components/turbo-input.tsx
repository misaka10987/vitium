import { Textarea } from '@/components/ui/textarea'
import { sendMessage } from '@/lib/chat'
import { useRef, useState } from 'react'
import { Button } from '@/components/ui/button'
import { Send, Code2, ChevronsRight } from 'lucide-react'
import { panic } from '@/lib/util'
import { cn } from '@/lib/utils'
import { handleCommand } from '@/lib/cmd'
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from './ui/tooltip'

/**
 * A multifunction input area for chat message and command input.
 */
export const TurboInput = () => {
  const msgForm = useRef<HTMLFormElement>(null)
  const msgInput = useRef<HTMLTextAreaElement>(null)
  const [isCommand, setIsCommand] = useState(false)
  const [enableHTML, setEnableHTML] = useState(false)
  return (
    <form
      ref={msgForm}
      className="flex flex-row-reverse gap-2 h-full w-full"
      onSubmit={(e) => {
        e.preventDefault()
        const form = new FormData(e.currentTarget)
        const content = form.get('content')?.toString() ?? panic()
        if (isCommand) {
          handleCommand(content)
        } else {
          sendMessage(content, enableHTML)
        }
        const curr = msgInput?.current ?? panic()
        curr.value = ''
        setIsCommand(false)
      }}
    >
      <div className="flex flex-col gap-2">
        <div className="flex flex-1/2">
          <TooltipProvider>
            <Tooltip>
              <TooltipTrigger asChild>
                <Button className="h-10 w-10 ease-in-out" type="submit">
                  {isCommand ? <ChevronsRight /> : <Send />}
                </Button>
              </TooltipTrigger>
              <TooltipContent>
                <span className="select-none">
                  {isCommand ? 'Send Command' : 'Send Message'}
                </span>
              </TooltipContent>
            </Tooltip>
          </TooltipProvider>
        </div>
        <div className="flex flex-1/2">
          <TooltipProvider>
            <Tooltip>
              <TooltipTrigger asChild>
                <Button
                  className="h-10 w-10 ease-in-out"
                  type="button"
                  variant={enableHTML ? 'default' : 'secondary'}
                  onClick={() => setEnableHTML((enabled) => !enabled)}
                >
                  <Code2 />
                </Button>
              </TooltipTrigger>
              <TooltipContent>
                <span className="select-none">Enable HTML</span>
              </TooltipContent>
            </Tooltip>
          </TooltipProvider>
        </div>
      </div>

      <div className="flex flex-grow w-0 h-full">
        <Textarea
          ref={msgInput}
          name="content"
          className={cn(
            'resize-none overflow-auto py-2',
            isCommand && 'font-bold font-mono'
          )}
          placeholder={isCommand ? 'Type Command' : 'Type Message'}
          required
          onKeyDown={(e) => {
            if (e.key == 'Enter' && !e.shiftKey) {
              e.preventDefault()
              msgForm.current?.requestSubmit()
            }
            if (e.currentTarget.value == '') {
              if (e.key == 'Backspace' && isCommand) {
                e.preventDefault()
                setIsCommand(false)
              }
              if (e.key == ':' && !isCommand) {
                e.preventDefault()
                setIsCommand(true)
              }
            }
          }}
        />
      </div>
    </form>
  )
}
