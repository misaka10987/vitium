# HTTP Route `/api/auth/pass`

## POST

Signup a new user.

### Request Body

JSON representation of the following Rust object:

```rust
pub struct SignUp {
    pub user: String,
    pub pass: String,
}
```
