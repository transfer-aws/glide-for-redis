/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export const enum RequestType {
  /** Type of a get string request. */
  GetString = 1,
  /** Type of a set string request. */
  SetString = 2,
}
export const enum ResponseType {
  /** Type of a response that returns a null. */
  Null = 0,
  /** Type of a response that returns a string. */
  String = 1,
}
export const HEADER_LENGTH_IN_BYTES: number
export const SOCKET_FILE_PATH: string
export function StartSocketConnection(
  connectionAddress: string,
  readSocketName: string,
  writeSocketName: string,
  startCallback: (err: null | Error) => void,
  closeCallback: (err: null | Error) => void,
): void
export class AsyncClient {
  static CreateConnection(connectionAddress: string): AsyncClient
  get(key: string): Promise<string | null>
  set(key: string, value: string): Promise<void>
}
