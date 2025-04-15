"use client"

import React, { useState, useEffect, useRef } from "react";
import { Textarea } from "@/components/ui/textarea";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Chatbubble } from "@/components/chatbubble";
import { Button } from "@/components/ui/button";
import { sendMessage, sendImage, SetSSEListener } from "@/lib/chat";
import { Send, Image } from "lucide-react";
import { hostStore } from "@/components/host";
import { useUsername } from "@/components/user";

export function Chatbox() {
    const [message, setMessage] = useState("");
    const [messages, setMessages] = useState<any[]>([]);
    const { hostname } = hostStore.getState();
    const connectAttempts = useRef(0);
    const name = useUsername();

    useEffect(() => {
        if (!hostname) return;

        let eventSource: EventSource;

        const connectToSSE = () => {
            // The browser automatically sets Accept: text/event-stream for EventSource connections
            eventSource = new EventSource(`https://${hostname}/chat`, {
                withCredentials: true
            });

            // Log connection attempt
            console.log("Establishing SSE connection to", hostname);

            // Set up the SSE listener
            SetSSEListener(eventSource, setMessages);

            // Reset attempts counter on successful connection
            eventSource.addEventListener("open", () => {
                connectAttempts.current = 0;
            });

            // Handle reconnection if needed
            eventSource.addEventListener("error", () => {
                if (eventSource.readyState === EventSource.CLOSED) {
                    connectAttempts.current += 1;
                    console.log(`Connection closed. Attempt ${connectAttempts.current}/3`);

                    if (connectAttempts.current >= 3) {
                        console.log("Maximum connection attempts reached. Redirecting to login.");
                        window.location.href = "/login";
                        return;
                    }

                    setTimeout(connectToSSE, 3000); // Reconnect after 3 seconds
                }
            });
        };

        connectToSSE();

        // Clean up when component unmounts
        return () => {
            if (eventSource) {
                console.log("Closing SSE connection");
                eventSource.close();
            }
        };
    }, [hostname]);

    const sendCurrentMessage = () => {
        if (message.trim()) {

            //     // Add the sent message to the UI immediately
            //     setMessages(prev => [...prev, {
            //         author: name,
            //         timestamp: Date.now(),
            //         message: message,
            //         variant: "send",
            //         html: false
            //     }]);

            sendMessage(message);
            setMessage("");
        }
    };

    const handleSendMessage = (e: React.FormEvent) => {
        e.preventDefault();
        sendCurrentMessage();
    };

    const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
        if (e.key === 'Enter' && !e.shiftKey) {
            e.preventDefault(); // Prevent default Enter behavior (new line)
            sendCurrentMessage();
        }
    };

    const handleImageUpload = () => {
        sendImage();
    };

    return (
        <div className="flex flex-col h-screen w-full">
            <div className="flex-1 flex flex-col">
                <ScrollArea
                    className="h-[calc(100vh-180px)] flex w-full rounded-md border p-1 mb-2"
                    type="auto"
                    scrollHideDelay={100}
                >
                    <div className="p-2">
                        {messages.map((msg, index) => (
                            <Chatbubble
                                key={index}
                                author={msg.author}
                                timestamp={msg.timestamp}
                                message={msg.html ? <iframe sandbox=""><div dangerouslySetInnerHTML={{ __html: msg.message }} /></iframe> : msg.message}
                                variant={msg.variant} // use iframe above to create a 'sandbox'
                            />
                        ))}
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
                        onKeyDown={handleKeyDown}
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
