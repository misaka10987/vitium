import { is } from 'typia'
import { CommandRecord, Message } from 'vitium-api'
import { MessageBubble } from './message-bubble'
import { CommandBubble } from './command-bubble'
import { match, P } from 'ts-pattern'
import { panic } from '@/lib/util'

export const Bubble = ({ content }: { content: Message | CommandRecord }) => {
  return match(content)
    .with(
      P.when((x) => is<Message>(x)),
      (content) => <MessageBubble {...content} />
    )
    .with(
      P.when((x) => is<CommandRecord>(x)),
      (content) => <CommandBubble record={content} />
    )
    .otherwise(() => panic())
}
