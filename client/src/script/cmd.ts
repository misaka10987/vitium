const hello: () => Promise<void> = async () => await invoke("hello")

const login: (server: string, user: string, pass: string) => Promise<void>
   = async (server, user, pass) => await invoke("login", { server, user, pass })

const user: () => Promise<string> = async () => await invoke("user")

const server_addr: () => Promise<string> = async () => await invoke("server_addr")

const recv_chat: () => Promise<void> = async () => await invoke("recv_chat")

const send_chat: (msg: string) => Promise<void> = async (msg) => await invoke("send_chat", { msg })

const render_chat: () => Promise<string> = async () => await invoke("render_chat")

const chat_modified: () => Promise<boolean> = async () => await invoke("chat_modified")