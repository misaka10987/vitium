const { invoke } = window.__TAURI__.tauri;

const redirect = (url) => window.location.href = url;

const hello = async () => await invoke("hello");

const login = async (server, user, pass) => await invoke("login", { server, user, pass });

const user = async () => await invoke("user");

const server_addr = async () => await invoke("server_addr");

const recv_chat = async () => await invoke("recv_chat");

const send_chat = async (msg) => await invoke("send_chat", { msg });

const render_chat = async () => await invoke("render_chat");
