# HTTP Route `/api/chat`

For out-game chats.

## GET

Synchronize recent chat messages.

### Response Body

JSON representation of the following Rust object:

```rust
type Response = Vec<(String, Chat)>;
```

which is a list of K-V pairs of usernames and chat messages.

## POST

Send a chat message.

### Request Header

`Cookie: token=` --- the token

### Request Body

JSON representation of the following Rust object:

```rust
pub struct SendChat(pub Chat);
```

which contains a `Chat` message to send.

### Response Body

JSON representation of the following Rust object:

```rust
type Response = SystemTime;
```

which is the time at which the message is received by server.
