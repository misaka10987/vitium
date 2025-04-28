import { Textarea } from '@/components/ui/textarea'
import { sendMessage } from '@/lib/chat'
import { useRef, useState } from 'react'
import { Button } from '@/components/ui/button'
import { Send, Code2 } from 'lucide-react'
import { panic } from '@/lib/util'

export const TurboInput = () => {
  const msgForm = useRef<HTMLFormElement>(null)
  const msgInput = useRef<HTMLTextAreaElement>(null)
  const [enableHTML, setEnableHTML] = useState(false)
  return (
    <form
      ref={msgForm}
      className="flex gap-2 items-center w-full"
      onSubmit={(e) => {
        e.preventDefault()
        const form = new FormData(e.currentTarget)
        const content = form.get('msg')?.toString() ?? panic()
        sendMessage(content, enableHTML)
        const curr = msgInput?.current ?? panic()
        curr.value = ''
      }}
    >
      <Textarea
        ref={msgInput}
        name="msg"
        className="h-full resize-none overflow-auto py-2"
        placeholder="Type your message here..."
        required
        onKeyDown={(e) => {
          if (e.key == 'Enter' && !e.shiftKey) {
            e.preventDefault()
            msgForm.current?.requestSubmit()
          }
        }}
      />
      <div className="flex flex-col gap-1">
        <Button className="h-[40px]" type="submit" aria-label="Send message">
          <Send className="h-4 w-4" />
        </Button>
        <Button
          className="h-[40px] mt-1 ease-in-out"
          type="button"
          variant={enableHTML ? 'default' : 'secondary'}
          onClick={() => setEnableHTML((enabled) => !enabled)}
        >
          <Code2 className="h-4 w-4" />
        </Button>
      </div>
    </form>
  )
}
