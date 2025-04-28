import { Textarea } from '@/components/ui/textarea'
import { sendMessage } from '@/lib/chat'
import { useRef } from 'react'
import { Button } from '@/components/ui/button'
import {
  Send,
  // due to an eslint bug, it requires you to provide an `alt` prop
  // this is a walkaround
  Image as Photo,
} from 'lucide-react'
import { sendCommand } from '@/lib/cmd'
import { panic } from '@/lib/util'

export const TurboInput = ({}: {}) => {
  const msgForm = useRef<HTMLFormElement>(null)
  const msgInput = useRef<HTMLTextAreaElement>(null)
  return (
    <form
      ref={msgForm}
      className="flex gap-2 items-center w-full"
      onSubmit={(e) => {
        e.preventDefault()
        const form = new FormData(e.currentTarget)
        const content = form.get('msg')?.toString() ?? panic()
        sendMessage(content)
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
        <Button
          className="h-[40px] px-4"
          type="submit"
          aria-label="Send message"
        >
          <Send className="h-4 w-4" />
        </Button>
        <Button
          className="h-[40px] px-4 mt-1"
          type="button"
          variant="outline"
          aria-label="Upload image"
        >
          <Photo className="h-4 w-4" />
        </Button>
      </div>
    </form>
  )
}
