import { createSignal } from 'solid-js';

// Signal for the server address
export const [serverAddress, setServerAddress] = createSignal<URL | null>(null);

export const [userName, setUserName] = createSignal('');