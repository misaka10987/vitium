import { Range, type SemVer } from 'semver'

export const upperBounded = (req: string | Range) =>
  !new Range(req, { loose: false, includePrerelease: false }).test(
    '1000000.1000000.1000000'
  )
