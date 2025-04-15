import { Avatar, AvatarFallback } from '@/components/ui/avatar'
import { Badge } from '@/components/ui/badge'
import { cn } from '@/lib/utils'
import { Message } from 'vitium-api'

export const Chatbubble = ({
  sender,
  time,
  content,
  html,
  variant = 'default',
}: Message & { variant?: 'default' | 'send' | 'receive' }) => {
  const renderedContent = html ? (
    <iframe sandbox="">
      <div dangerouslySetInnerHTML={{ __html: content }} />
    </iframe>
  ) : (
    content
  )
  return (
    <div
      className={cn(
        'flex flex-col gap-2',
        variant === 'send' && 'self-end',
        variant === 'receive' && 'self-start'
      )}
    >
      <div className="flex items-start gap-2">
        <div className={cn('flex-shrink-0', variant === 'send' && 'order-2')}>
          <Avatar className="m-1">
            <AvatarFallback className="text-sm select-none">
              {sender.substring(0, 2)}
            </AvatarFallback>
          </Avatar>
        </div>
        <div className="flex-grow">
          <div
            className={cn(
              'flex items-center gap-1',
              variant === 'send' && 'justify-end'
            )}
          >
            <span className="text-sm font-medium mt-1 text-muted-foreground truncate select-none">
              {sender}
            </span>
            <Badge
              variant="outline"
              className="text-[10px] py-0 px-1 h-auto mt-1 bg-transparent text-muted-foreground border-border flex-shrink-0 select-none"
            >
              {new Date(Number(time)).toLocaleTimeString([], {
                hour: '2-digit',
                minute: '2-digit',
              })}
            </Badge>
          </div>
          <div
            className={cn(
              'text-sm p-1 pl-2 pr-2 mt-1 rounded-lg max-w-md break-all overflow-hidden w-fit',
              variant === 'default' && 'bg-secondary text-secondary-foreground',
              variant === 'send' &&
                'bg-primary text-primary-foreground ml-auto rounded-tr-none',
              variant === 'receive' &&
                'bg-secondary text-secondary-foreground mr-auto rounded-tl-none'
            )}
          >
            {renderedContent}
          </div>
        </div>
      </div>
    </div>
  )
}
