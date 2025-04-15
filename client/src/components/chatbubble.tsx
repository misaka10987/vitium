import React, { JSX } from "react";
import { Avatar, AvatarFallback } from "@/components/ui/avatar";
import { Badge } from "@/components/ui/badge";
import { cn } from "@/lib/utils";

interface ChatbubbleProps {
    author: string;
    timestamp: number;
    message: string | JSX.Element;
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
                        <AvatarFallback className="text-sm select-none">
                            {author.length > 1 ? author.charAt(0).toUpperCase() + author.charAt(1) : author.charAt(0).toUpperCase()}
                        </AvatarFallback>
                    </Avatar>
                </div>
                <div className="flex-grow">
                    <div className={cn(
                        "flex items-center gap-1",
                        variant === "send" && "justify-end"
                    )}>
                        <span className="text-sm font-medium mt-1 text-muted-foreground truncate select-none">{author}</span>
                        <Badge variant="outline" className="text-[10px] py-0 px-1 h-auto mt-1 bg-transparent text-muted-foreground border-border flex-shrink-0 select-none">
                            {new Date(timestamp).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
                        </Badge>
                    </div>
                    <div className={cn(
                        "text-sm p-1 pl-2 pr-2 mt-1 rounded-lg max-w-md break-all overflow-hidden w-fit",
                        variant === "default" && "bg-secondary text-secondary-foreground",
                        variant === "send" && "bg-primary text-primary-foreground ml-auto rounded-tr-none",
                        variant === "receive" && "bg-secondary text-secondary-foreground mr-auto rounded-tl-none"
                    )}>
                        {message}
                    </div>
                </div>
            </div>
        </div>
    );
}