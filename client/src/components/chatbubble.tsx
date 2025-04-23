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
        'flex flex-row gap-2 w-full',
        variant == 'send' && 'flex-row-reverse'
      )}
    >
      <div className="flex">
        <Avatar className="m-1">
          <AvatarFallback className="text-sm select-none">
            {sender?.substring(0, 2).toUpperCase()}
          </AvatarFallback>
        </Avatar>
      </div>
      <div className="flex-grow">
        <div
          className={cn(
            'flex flex-row gap-1',
            variant === 'send' && 'flex-row-reverse'
          )}
        >
          <Badge
            variant="outline"
            className="flex text-[10px] py-0 px-1 h-auto mt-1 bg-transparent text-muted-foreground border-border flex-shrink-0 select-none"
          >
            {new Date(Number(time)).toLocaleTimeString([], {
              hour: '2-digit',
              minute: '2-digit',
            })}
          </Badge>
          <span className="flex text-sm font-medium mt-1 text-muted-foreground truncate select-none">
            {sender}
          </span>
        </div>
        <div
          className={cn(
            'flex',
            variant == 'send' && 'justify-end',
            variant == 'receive' && 'justify-start'
          )}
        >
          <div
            className={cn(
              'text-sm p-1 pl-2 pr-2 mt-1 rounded-lg max-w-md w-fit',
              variant == 'default' && 'bg-secondary text-secondary-foreground',
              variant == 'send' && 'bg-primary text-primary-foreground',
              variant == 'receive' && 'bg-secondary text-secondary-foreground'
            )}
          >
            {renderedContent}
          </div>
        </div>
      </div>
    </div>
  )
}
