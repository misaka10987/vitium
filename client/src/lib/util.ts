/**
 * Throws an error with provided message and payload.
 *
 * This function never returns.
 */
export const panic = <T>(msg: string = '', ...payload: T[]): never => {
  console.warn(msg, ...payload)
  throw Error(msg)
}

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
export const defined = (template: TemplateStringsArray, ...args: unknown[]): string | undefined => {
  if (args.some(v => v === undefined))
    return undefined
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
export const nonnull = (template: TemplateStringsArray, ...args: unknown[]): string | null => {
  if (args.some(v => v === null))
    return null
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
export const some = (template: TemplateStringsArray, ...args: unknown[]): string | undefined | null => {
  if (args.some(v => v === undefined))
    return undefined
  if (args.some(v => v === null))
    return null
  return String.raw(template, ...args)
}
