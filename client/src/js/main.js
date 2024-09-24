const { invoke } = window.__TAURI__.tauri;

const redirect = (url) => window.location.href = url;

const hello = () => invoke("hello");

const connect = (server, user, pass) => invoke("connect", { server, user, pass });
