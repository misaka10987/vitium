import { createSignal } from 'solid-js';

// Signal for the server address
export const [serverAddress, setServerAddress] = createSignal('localhost:3001');
export function getServerURL() {
  return `http://${serverAddress()}`;
}
export const [userName, setUserName] = createSignal('');