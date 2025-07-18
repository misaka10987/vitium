import { Avatar, AvatarFallback } from '@/components/ui/avatar'
import { Badge } from '@/components/ui/badge'
import { cn } from '@/lib/utils'
import { Message } from 'vitium-api'
import DOMPurify from 'dompurify'
import { useUserStore } from './user'

/**
 * A bubble for displaying a certain chat message.
 *
 * See {@link Message} for parameter definitions.
 * @param sender
 * @param time
 * @param content
 * @param html
 */
export const MessageBubble = ({ sender, time, content, html }: Message) => {
  const { user } = useUserStore()
  const send = user == sender

  const renderedContent = html ? (
    <p
      className="text-sm text-justify"
      dangerouslySetInnerHTML={{ __html: DOMPurify.sanitize(content) }}
    />
  ) : (
    <p className="text-sm text-justify whitespace-pre-wrap">{content}</p>
  )

  return (
    <div
      className={cn('flex flex-row gap-2 w-full', send && 'flex-row-reverse')}
    >
      <div className="flex">
        <Avatar className="m-1">
          <AvatarFallback className="text-sm font-bold select-none">
            {sender?.substring(0, 2).toUpperCase()}
          </AvatarFallback>
        </Avatar>
      </div>
      <div className="flex-grow">
        <div className="flex flex-col gap-1">
          <div
            className={cn('flex flex-row gap-1', send && 'flex-row-reverse')}
          >
            <Badge
              variant="outline"
              className="flex text-xs py-0 px-1 bg-transparent text-muted-foreground select-none"
            >
              {new Date(time).toLocaleTimeString([], {
                hour: '2-digit',
                minute: '2-digit',
              })}
            </Badge>
            <span className="flex text-sm font-medium align-middle text-muted-foreground select-none">
              {sender}
            </span>
          </div>
          <div className={cn('flex', send ? 'justify-end' : 'justify-start')}>
            <div
              className={cn(
                'py-1.5 px-2.5 rounded-lg',
                send
                  ? 'bg-accent text-accent-foreground'
                  : 'bg-secondary text-secondary-foreground'
              )}
            >
              {renderedContent}
            </div>
          </div>
        </div>
      </div>
      <div className="mx-2" />
    </div>
  )
}
