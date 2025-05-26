import { is } from 'typia'
import { CommandRecord, Message } from 'vitium-api'
import { MessageBubble } from './message-bubble'
import { CommandBubble } from './command-bubble'
import { match, P } from 'ts-pattern'
import { unreachable } from '@/lib/util'

/// The entry to display in a bubble.
export type Entry = Message | CommandRecord

/**
 * A bubble component for displaying chat entries (either chat message or a command).
 *
 * This component determines which type of entry to render runtime based on the type of parameter passed to it.
 *
 * @param entry the entry to display
 * @returns either a {@link MessageBubble} or a {@link CommandBubble}
 */
export const Bubble = ({ entry: entry }: { entry: Entry }) => {
  return match(entry)
    .with(
      P.when((x) => is<Message>(x)),
      (msg) => <MessageBubble {...msg} />
    )
    .with(
      P.when((x) => is<CommandRecord>(x)),
      (record) => <CommandBubble record={record} />
    )
    .otherwise(unreachable)
}
