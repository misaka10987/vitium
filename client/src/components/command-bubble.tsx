/* eslint react/jsx-key: 0 */

import { CommandRecord } from 'vitium-api'
import { match, P } from 'ts-pattern'

export const CommandBubble = ({
  record: [line, status],
}: {
  record: CommandRecord
}) => {
  const issuer = line.user ?? <span className="font-bold">Server</span>
  const [head, content] = match(status)
    .with({ Ok: P.string }, ({ Ok: status }) => [
      <span className="text-muted-foreground">
        {issuer}: <code>{line.line}</code>
      </span>,
      status,
    ])
    .with({ Err: P.string }, ({ Err: status }) => [
      <span className="text-destructive">
        {issuer}: <code>{line.line}</code>
      </span>,
      status,
    ])
    .exhaustive()
  return (
    <div className="flex-col text-sm items-center justify-center gap-2 w-full wrap-break-word m-1">
      <div className="align-middle">{head}</div>
      <div>{content}</div>
    </div>
  )
}
