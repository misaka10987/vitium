import { CommandRecord } from "vitium-api";

export const CommandBubble = ([content_line, content_status]: CommandRecord) => {
    if ('Ok' in content_status) {
        return (
            <div className="flex-row text-sm text-muted-foreground items-center justify-center gap-2 w-full wrap-break-word">
                <div>
                    {content_line.user ?? 'System'}'s command executed successfully.
                </div>
                <div>
                    {content_status.Ok}
                </div>
            </div>
        )
    }
    else if ('Err' in content_status) {
        return (
            <div className="flex-row text-sm text-destructive items-center justify-center gap-2 w-full wrap-break-word">
                <div>
                    {content_line.user ?? 'System'}'s command:"{content_line.line}" failed.
                </div>
                <div>
                    {content_status.Err}
                </div>
            </div>
        )
    }
}