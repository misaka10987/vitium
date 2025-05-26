import { validateEquals } from 'typia'
import { Command } from '../cmd'

interface Args {
  _: [string]
}

/**
 * Command for searching an entry in the DnD ruleset.
 */
export const dnd: Command = {
  name: 'dnd',
  valid: (x: unknown) => validateEquals<Args>(x).success,
  exec: async (x: Args) => {
    const keyword = x._[0]
    if (window)
      window.open(`https://5e.dickytwister.org/search.html?${keyword}`)
  },
}
