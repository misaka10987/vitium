"use client"

import React, { useState } from "react";
import { Textarea } from "@/components/ui/textarea";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Chatbubble } from "@/components/chatbubble";
import { Button } from "@/components/ui/button";
import { sendMessage, sendImage } from "@/lib/chat";
import { Send, Image } from "lucide-react";

export function Chatbox() {
    const [message, setMessage] = useState("");

    const handleSendMessage = (e: React.FormEvent) => {
        e.preventDefault();
        if (message.trim()) {
            sendMessage(message);
            setMessage("");
        }
    };

    const handleImageUpload = () => {
        sendImage();
    };

    return (
        <div className="flex flex-col h-screen w-full">
            <div className="flex-1 flex flex-col">
                <ScrollArea
                    className="h-[calc(100vh-180px)] flex w-full rounded-md border p-1"
                    type="auto"
                    scrollHideDelay={100}
                >
                    <div className="p-2">
                        <Chatbubble author="dev" timestamp={1672531199000} message="Hello, how are you?" variant="receive" />
                        <Chatbubble author="U" timestamp={1672531199000} message="I'm good, thanks!" variant="send" />
                    </div>
                </ScrollArea>
                <form onSubmit={handleSendMessage} className="flex gap-2 items-center">
                    <Textarea
                        className="h-[90px] w-full"
                        id="chatinput"
                        name="chatinput"
                        placeholder="Type your message here..."
                        required
                        value={message}
                        onChange={(e) => setMessage(e.target.value)}
                    />
                    <div className="flex flex-col gap-1">
                        <Button className="h-[40px] px-4" type="submit" aria-label="Send message">
                            <Send className="h-4 w-4" />
                        </Button>
                        <Button
                            className="h-[40px] px-4 mt-1"
                            type="button"
                            variant="outline"
                            aria-label="Upload image"
                            onClick={handleImageUpload}
                        >
                            <Image className="h-4 w-4" />
                        </Button>
                    </div>
                </form>
            </div>
        </div>
    )
}
