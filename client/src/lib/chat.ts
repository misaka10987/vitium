import { serverAddress, userName } from '~/lib/auth'

export async function sendMessage(content: string, isHtml: boolean) {
  try {
    const res = await fetch(`${serverAddress()}/api/chat`,
      {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(
          {
            sender: userName(),
            time: Math.floor(Date.now() / 1000),
            content: content,
            html: isHtml
          }
        )
      }
    );
    if (!res.ok) {
      throw new Error(`Error sending message: ${res.statusText}`);
    }
  } catch (error) {
    console.error(error);
  }
}