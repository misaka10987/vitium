import { CommandRecord } from "vitium-api";

export const CommandBubble = (content: CommandRecord) => {
    if ('Ok' in content[1]) {
        return (
            <div className="flex-row text-sm text-muted-foreground items-center justify-center gap-2 w-full wrap-break-word">
                <div>
                    {content[0].user ?? 'System'}'s command executed successfully.
                </div>
                <div>
                    {content[1].Ok}
                </div>
            </div>
        )
    }
    else if ('Err' in content[1]) {
        return (
            <div className="flex-row text-sm text-destructive items-center justify-center gap-2 w-full wrap-break-word">
                <div>
                    {content[0].user ?? 'System'}'s command:"{content[0].line}" failed.
                </div>
                <div>
                    {content[1].Err}
                </div>
            </div>
        )
    }
}