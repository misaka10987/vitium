/**
 * Throws an error with provided message and payload.
 *
 * This function never returns.
 */
export const panic = (msg: string = '', ...payload: object[]): never => {
  console.warn(msg, ...payload)
  throw Error(msg)
}
