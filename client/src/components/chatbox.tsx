import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import { ScrollArea } from "@/components/ui/scroll-area";

export function Chatbox() {
    return (
        <div className="flex flex-col gap-6">
            <div className="grid gap-3">
                <Label htmlFor="chatbox">Chat</Label>
                <ScrollArea 
                    className="h-full w-full rounded-md border"
                    type="auto"
                    scrollHideDelay={100}
                >
                </ScrollArea>
                <Textarea
                    id="chatinput"
                    name="chatinput"
                    placeholder="Type your message here..."
                    required
                />
            </div>
        </div>
    )
}