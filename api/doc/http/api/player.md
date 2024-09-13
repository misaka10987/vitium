# HTTP Route `/api/player`

## GET

List the username of all player.

### Response Body

JSON representation of the following Rust object:

```rust
type Response = Vec<String>;
```

which is a list of usernames.

## GET `/<name>`

Get player information of the specified username `name`.

### Response Body

JSON representation of the following Rust object:

```rust
type Response = Player;
```

which is the specified player information.

## POST `/<name>`

Edit the player `name`'s information.

### Request Header

`Cookie: token=` --- the token

### Request Body

JSON representation of the following Rust object:

```rust
pub struct EditPlayer(pub Player);
```

which contains the new player information to update.
