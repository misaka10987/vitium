import { TextField, TextFieldTextArea } from '~/components/ui/text-field'
import { sendMessage } from '~/lib/chat'
import { createSignal, createEffect, onCleanup } from 'solid-js'
import { Button } from '~/components/ui/button'
import { Send, Code, ChevronsRight } from 'lucide-solid'
import { cn, panic } from '~/lib/utils'
import { handleCommand } from '~/lib/cmd'
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '~/components/ui/tooltip'
/**
 * A multifunction input area for chat message and command input.
 */
import { JSX } from 'solid-js'
export const ChatInput = () => {
  let msgForm: HTMLFormElement | undefined
  let msgInput: HTMLTextAreaElement | undefined
  const [isCommand, setIsCommand] = createSignal(false) // UI state for indicating the current input is a command
  const [enableHTML, setEnableHTML] = createSignal(false)

  const handleSubmit: JSX.EventHandlerUnion<HTMLFormElement, SubmitEvent> = (e) => {
    e.preventDefault()
    const form = new FormData(msgForm!)
    const content = form.get('content')?.toString() ?? panic('No content')
    if (content.trim().startsWith('/')) {
      handleCommand(content)
      setIsCommand(true)
    } else {
      sendMessage(content, enableHTML())
      setIsCommand(false)
    }
    if (!msgInput) panic('No msgInput')
    msgInput.value = ''
  }

  const handleKeyDown: JSX.EventHandlerUnion<HTMLTextAreaElement, KeyboardEvent> = (e) => {
    if (e.key == 'Enter' && !e.shiftKey) {
      e.preventDefault()
      msgForm?.requestSubmit()
    }
    // Dynamically update isCommand based on input value - used for the ui
    const value = (e.currentTarget as HTMLTextAreaElement).value + (e.key.length === 1 ? e.key : '');
    if (value.trim().startsWith('/')) {
      setIsCommand(true)
    } else {
      setIsCommand(false)
    }
  }

  return (
    <form
      ref={msgForm}
      class="flex flex-row-reverse gap-2 h-full w-full"
      onSubmit={handleSubmit}
    >
      <div class="flex flex-col gap-2">
        <div class="flex flex-1/2">
          <Tooltip>
            <TooltipTrigger>
              <Button class="h-10 w-10 ease-in-out" type="submit">
                {isCommand() ? <ChevronsRight /> : <Send />}
              </Button>
            </TooltipTrigger>
            <TooltipContent>
              <span class="select-none">
                {isCommand() ? 'Send Command' : 'Send Message'}
              </span>
            </TooltipContent>
          </Tooltip>

        </div>
        <div class="flex-1/2">
          <Tooltip>
            <TooltipTrigger>
              <Button
                class="h-10 w-10 ease-in-out"
                type="button"
                variant={enableHTML() ? 'default' : 'secondary'}
                onClick={(e) => {
                  e.preventDefault();
                  e.stopPropagation();
                  setEnableHTML((v) => !v);
                  if (msgInput) msgInput.focus();
                }}
              >
                <Code />
              </Button>
            </TooltipTrigger>
            <TooltipContent>
              <span class="select-none">Enable HTML</span>
            </TooltipContent>
          </Tooltip>
        </div>
      </div>

      <TextField class="flex grow w-0 h-full">
        <TextFieldTextArea
          ref={msgInput}
          name="content"
          class={cn(
            'resize-none overflow-auto h-full',
            isCommand() && 'font-bold font-mono'
          )}
          placeholder={isCommand() ? 'Type Command' : 'Type Message'}
          spellcheck={!isCommand()}
          required
          onKeyDown={handleKeyDown}
        />
      </TextField>
    </form>
  )
}
