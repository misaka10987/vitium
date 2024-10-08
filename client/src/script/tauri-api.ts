const TAURI_API = (<any>window).__TAURI__

const invoke: (cmd: string, arg?: any) => Promise<any> = TAURI_API.core.invoke
