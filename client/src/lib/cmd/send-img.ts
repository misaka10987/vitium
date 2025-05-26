import { validateEquals } from 'typia'
import { Command } from '../cmd'
import { sendMessage } from '../chat'

interface Args {
  _: [string]
}

/**
 * Command for sending an image with its URL.
 */
export const sendImage: Command = {
  name: 'send-img',
  valid: (param: unknown) => {
    const res = validateEquals<Args>(param)
    if (!res.success) return false
    const url = res.data._[0]
    try {
      new URL(url)
      return true
    } catch (_) {
      return false
    }
  },
  exec: async (param: Args) => {
    const url = new URL(param._[0])
    const html = `<img src="${url}" alt="user sent image" />`
    sendMessage(html, true)
  },
}
