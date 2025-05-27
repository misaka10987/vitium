import { useHostStore } from '@/components/host'
import { useUserStore } from '@/components/user'
import { CommandLine } from 'vitium-api'
import { panic } from './util'
import { json } from 'typia'
import { parse } from 'shell-quote'
import parser from 'yargs-parser'
import { dnd } from './cmd/dnd'
import { sendImage } from './cmd/send-img'

/**
 * A client command instance.
 */
export interface Command {
  /**
   * Name of the command.
   */
  name: string

  /**
   * Validate the provided parameter.
   *
   * @param param some object
   * @returns if the parameter is valid
   */
  valid: (param: unknown) => boolean

  /**
   * Execute the command with provided parameter.
   *
   * The parameter is guaranteed valid, i.e. `.valid(param)`.
   *
   * @param param some object
   * @returns an async execution
   */
  // the type safety is manually guaranteed here
  /* eslint @typescript-eslint/no-explicit-any: 0 */
  exec: (param: any) => Promise<unknown>
}

/**
 * Register a command to the client command registry.
 *
 * @param cmd the command to register
 */
export const registerCommand = (cmd: Command) => {
  registry.set(cmd.name, cmd)
}

/**
 * Attempt to run a client command.
 *
 * @param name the command to run
 * @returns a function calling which with arguments would execute the command,
 * or `null` if the command does not exist
 */
export const runCommand = (name: string) => {
  const cmd = registry.get(name)
  if (!cmd) return null

  const exec = async (args: string[]) => {
    const parsed = parser(args, {
      configuration: {
        'parse-positional-numbers': false,
      },
    }) as any
    const valid = cmd.valid(parsed)
    if (!valid) {
      alert(`${cmd.name}: invalid arguments`)
      return
    }
    await cmd.exec(parsed)
  }
  return exec
}

const registry = new Map<string, Command>()

registerCommand(dnd)
registerCommand(sendImage)

/**
 * Handles a command.
 *
 * If the command exists in the client command registry,
 * would run the client command, falling back to sending the command to server otherwise.
 *
 * @param line the line of command
 */
export const handleCommand = async (line: string) => {
  const tokens = parse(line)
  const name = tokens[0]

  if (typeof name == 'string') {
    const run = runCommand(name)
    if (run) {
      console.info('Running client command:', line)
      const args = tokens
        .slice(1)
        .map((s) => (typeof s == 'string' ? s : panic()))
      await run(args)
      return
    }
  }

  console.info('Sending command:', line)
  await sendCommand(line)
}

/**
 * Send a chat message to the game server with current username.
 *
 * @param line the command
 */
export const sendCommand = async (line: string) => {
  const host = useHostStore.getState().host ?? panic('Missing hostname')
  const user = useUserStore.getState().user ?? panic('Missing username')

  const res = await fetch(`https://${host}/api/cmd`, {
    method: 'POST',
    credentials: 'include',
    headers: {
      'Content-Type': 'application/json',
    },
    body: json.assertStringify<CommandLine>({ user, line }),
  })
  if (!res.ok) panic('HTTP error', res)
}
