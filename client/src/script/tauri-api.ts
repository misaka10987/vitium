const TAURI_API = (<any>window).__TAURI__

const invoke: (cmd: string, arg?: any) => Promise<any> = TAURI_API.core.invoke

const file_url: (path: string) => string = TAURI_API.core.convertFileSrc
