import { createSignal } from 'solid-js';

// Signal for the server address
export const [serverAddress, setServerAddress] = createSignal<URL | null>(null);

export const [userName, setUserName] = createSignal('');

function base64url(bytes: Uint8Array): string {
    return btoa(String.fromCharCode(...bytes))
        .replace(/\+/g, '-')
        .replace(/\//g, '_')
        .replace(/=+$/g, '');
}

export function code_verifier_gen(): string {
    const array = new Uint8Array(64); // RFC7636 requests a length between 43 and 128 characters, used 64 here
    crypto.getRandomValues(array);
    return base64url(array);
}

export function code_challenge(code_verifier: string): Promise<string> {
    const encoder = new TextEncoder(); // utf8 is compatible with ascii, so the encode should be fine
    const data = encoder.encode(code_verifier);
    // Perform the SHA-256 hash
    return crypto.subtle.digest('SHA-256', data).then((hash) => {
        // Convert the hash to a base64url string
        return base64url(new Uint8Array(hash));
    });
}

