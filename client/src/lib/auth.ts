import { createSignal } from 'solid-js';

// Signal for the server address
export const [serverAddress, setServerAddress] = createSignal('');
export const [userName, setUserName] = createSignal('');