import { createSignal } from 'solid-js';

// Signal for the server address
export const [serverAddress, setServerAddress] = createSignal<URL>(new URL('http://server.vitium.dev'));

export const [userName, setUserName] = createSignal('');