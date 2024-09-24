const { invoke } = window.__TAURI__.tauri;

const redirect = (url) => window.location.href = url;

const hello = async () => await invoke("hello");

const login = async (server, user, pass) => await invoke("login", { server, user, pass });

const user = async () => await invoke("user");

const server_addr = async () => await invoke("server_addr");
