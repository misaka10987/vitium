/* eslint react/jsx-key: 0 */

import { CommandRecord } from 'vitium-api'
import { match, P } from 'ts-pattern'
import ANSI from 'ansi-to-html'

export const CommandBubble = ({
  record: [line, status],
}: {
  record: CommandRecord
}) => {
  const ansi = new ANSI()
  const issuer = line.user ?? <span className="font-bold">Server</span>
  const [head, content] = match(status)
    .with({ Ok: P.string }, ({ Ok: status }) => [
      <span className="text-muted-foreground">
        {issuer}: <code>{line.line}</code>
      </span>,
      ansi.toHtml(status),
    ])
    .with({ Err: P.string }, ({ Err: status }) => [
      <span className="text-destructive">
        {issuer}: <code>{line.line}</code>
      </span>,
      ansi.toHtml(status),
    ])
    .exhaustive()
  return (
    <div className="flex-col text-sm items-center justify-center gap-2 w-full wrap-break-word m-1">
      <div className="align-middle">{head}</div>
      <div
        className="whitespace-pre-wrap"
        dangerouslySetInnerHTML={{ __html: content.toString() }}
      ></div>
    </div>
  )
}
