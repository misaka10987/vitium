import { Avatar, AvatarFallback } from "@/components/ui/avatar";
import { Badge } from "@/components/ui/badge";
import { cn } from "@/lib/utils";

interface ChatbubbleProps {
    author: string;
    timestamp: number;
    message: string;
    variant?: "default" | "send" | "receive";
    className?: string;
}

export function Chatbubble({
    author,
    timestamp,
    message,
    variant = "default",
    className,
}: ChatbubbleProps) {
    return (
        <div className={cn(
            "flex flex-col gap-2",
            variant === "send" && "self-end",
            variant === "receive" && "self-start",
            className
        )}>
            <div className="flex items-start gap-2">
                <div className={cn(
                    "flex-shrink-0",
                    variant === "send" && "order-2"
                )}>
                    <Avatar className="m-1">
                        <AvatarFallback className="text-sm">
                            {author.length > 1 ? author.charAt(0).toUpperCase() + author.charAt(1) : author.charAt(0).toUpperCase()}
                        </AvatarFallback>
                    </Avatar>
                </div>
                <div className="flex-grow">
                    <div className={cn(
                        "flex items-center gap-1",
                        variant === "send" && "justify-end"
                    )}>
                        <span className="text-sm font-medium mt-1 dark:text-gray-300 truncate">{author}</span>
                        <Badge variant="outline" className="text-[10px] py-0 px-1 h-auto mt-1 dark:bg-transparent dark:text-gray-400 dark:border-gray-700 flex-shrink-0">
                            {new Date(timestamp).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
                        </Badge>
                    </div>
                    <p className={cn(
                        "text-sm p-1 pl-2 pr-2 mt-1 rounded-lg max-w-md break-words overflow-wrap-anywhere overflow-hidden",
                        variant === "default" && "bg-gray-100 text-gray-700 dark:bg-gray-800 dark:text-gray-300",
                        variant === "send" && "bg-gray-500 text-white ml-auto rounded-tr-none dark:bg-gray-600",
                        variant === "receive" && "bg-gray-200 text-gray-800 mr-auto rounded-tl-none dark:bg-gray-700 dark:text-gray-200"
                    )}>
                        {message}
                    </p>
                </div>
            </div>
        </div>
    );
}