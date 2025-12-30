import { createSignal } from 'solid-js';

// Signal for the server address
export const [serverAddress, setServerAddress] = createSignal<URL>(new URL('http://localhost:3001'));

export const [userName, setUserName] = createSignal('');