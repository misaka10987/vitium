import { CommandRecord } from 'vitium-api'
import { match, P } from 'ts-pattern'
import ANSI from 'ansi-to-html'

/**
 * A bubble for displaying a certain command record.
 *
 * @param record the command to display
 */
export const CommandBubble = ({
  record: [line, status],
}: {
  record: CommandRecord
}) => {
  const ansi = new ANSI()
  const issuer = line.user ? (
    <span className="italic">{line.user}</span>
  ) : (
    <span className="font-bold">Server</span>
  )
  // react/jsx-key incorrectly assume jsx elements in match branches are an array
  /* eslint react/jsx-key: 0 */
  const [head, content] = match(status)
    .with({ Ok: P.string }, ({ Ok }) => [
      <span className="text-muted-foreground">
        {issuer} : <code>{line.line}</code>
      </span>,
      ansi.toHtml(Ok),
    ] as const)
    .with({ Err: P.string }, ({ Err }) => [
      <span className="text-destructive">
        {issuer} : <code>{line.line}</code>
      </span>,
      ansi.toHtml(Err),
    ] as const)
    .exhaustive()
  return (
    <div className="text-sm gap-2 w-full p-0.5 m-0.5">
      <div className="align-middle break-all">{head}</div>
      <div
        className="whitespace-pre-wrap"
        dangerouslySetInnerHTML={{ __html: content }}
      />
    </div>
  )
}
