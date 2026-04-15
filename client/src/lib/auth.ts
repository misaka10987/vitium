const base64url = (bytes: Uint8Array) => {
    return btoa(String.fromCharCode(...bytes))
        .replace(/\+/g, '-')
        .replace(/\//g, '_')
        .replace(/=+$/g, '');
}

export const code_verifier_gen = () => {
    const array = new Uint8Array(64); // RFC7636 requests a length between 43 and 128 characters, used 64 here
    crypto.getRandomValues(array);
    return base64url(array);
}

export const code_challenge = async (code_verifier: string) => {
    const encoder = new TextEncoder(); // utf8 is compatible with ascii, so the encode should be fine
    const data = encoder.encode(code_verifier);
    const hash = await crypto.subtle.digest('SHA-256', data);
    const code = base64url(new Uint8Array(hash));
    return code;
}
