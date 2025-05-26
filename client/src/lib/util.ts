/**
 * Throws an error with provided message and payload.
 *
 * This function never returns.
 */
export const panic = (msg: string = '', ...payload: unknown[]): never => {
  console.warn(msg, ...payload)
  throw Error(msg)
}

/**
 * Indicates unreachable code to make the compiler happy.
 *
 * This function {@link panic}s if actually called.
 */
export const unreachable = (..._: unknown[]): never =>
  panic('Unreachable code reached')

/**
 * Helper for formatting string with nullable arguments.
 *
 * This function would return `undefined` if any of arguments is `undefined`,
 * or format the string as original behaviour otherwise.
 *
 * @param template the template string
 * @param args arguments for template string
 * @returns the format result
 */
export const defined = (
  template: TemplateStringsArray,
  ...args: unknown[]
): string | undefined => {
  if (args.some((v) => v === undefined)) return undefined
  return String.raw(template, ...args)
}

/**
 * Helper for formatting string with nullable arguments.
 *
 * This function would return `null` if any of arguments is `null`,
 * or format the string as original behaviour otherwise.
 *
 * @param template the template string
 * @param args arguments for template string
 * @returns the format result
 */
export const nonnull = (
  template: TemplateStringsArray,
  ...args: unknown[]
): string | null => {
  if (args.some((v) => v === null)) return null
  return String.raw(template, ...args)
}

/**
 * Helper for formatting string with nullable arguments.
 *
 * This function would return `undefined` if any of arguments is `undefined`, `null` if any is `null`,
 * or format the string as original behaviour otherwise.
 *
 * @param template the template string
 * @param args arguments for template string
 * @returns the format result
 */
export const some = (
  template: TemplateStringsArray,
  ...args: unknown[]
): string | undefined | null => {
  if (args.some((v) => v === undefined)) return undefined
  if (args.some((v) => v === null)) return null
  return String.raw(template, ...args)
}

/**
 * The identity function, i.e. returns exactly its input.
 *
 * @param x any value
 * @returns the same as input
 */
export const identity = <T>(x: T): T => x

/**
 * Assert some expression values true and {@link panic}s if not so.
 * 
 * @param assertion some assertion
 */
export const assert = (assertion: boolean) => {
  console.assert(assertion)
  if (!assertion) panic('Assertion failed')
}

/**
 * Assert two values are strictly equal (i.e. `===`) and {@link panic}s if not so.
 * 
 * @param lhs some value to compare
 * @param rhs some value to compare
 */
export const assertEq = (lhs: unknown, rhs: unknown) => {
  const eq = lhs === rhs
  console.assert(eq)
  if (!eq) panic('Assertion failed: ', lhs, ' does not equal to ', rhs)
}
