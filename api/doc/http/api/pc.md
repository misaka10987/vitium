# HTTP Route `/api/pc`

## GET

List the name of all player characters.

### Response Body

JSON representation of the following Rust object:

```rust
type Response = Vec<String>;
```

which is a list of character names.

## GET `/<name>`

Get player character of the specified `name`.

### Response Body

JSON representation of the following Rust object:

```rust
type Response = PC;
```

which is the specified player character.

## POST

Edit a player character's information.

### Request Header

`Cookie: token=` --- the token

### Request Body

JSON representation of the following Rust object:

```rust
pub struct EditPlayer(pub Player);
```

which contains the new player information to update.
