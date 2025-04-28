/**
 * Throws an error with provided message and payload.
 *
 * This function never returns.
 */
export const panic = <T>(msg: string = '', ...payload: T[]): never => {
  console.warn(msg, ...payload)
  throw Error(msg)
}
