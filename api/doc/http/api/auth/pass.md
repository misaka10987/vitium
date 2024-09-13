# HTTP Route `/api/auth/pass`

## POST

Modify a user's password.

### Request Header

`Authorization` --- password login credentials

### Request Body

JSON representation of the following Rust object:

```rust
pub struct EditPass(pub String);
```
