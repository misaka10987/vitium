import { is } from 'typia'
import { CommandLine, Message } from 'vitium-api'
import { MessageBubble } from './message-bubble'

export const Bubble = ({ content }: { content: Message | CommandLine }) => {
  if (is<Message>(content)) {
    return <MessageBubble {...content} />
  }
  if (is<CommandLine>(content)) {
    return 'umimplemented'
  }
}
