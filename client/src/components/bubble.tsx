import { is } from 'typia'
import { CommandRecord, Message } from 'vitium-api'
import { MessageBubble } from './message-bubble'
import { CommandBubble } from './command-bubble'

export const Bubble = ({ content }: { content: Message | CommandRecord }) => {
  if (is<Message>(content)) {
    return <MessageBubble {...content} />
  }
  if (is<CommandRecord>(content)) {
    return <CommandBubble {...content} />
  }
}
