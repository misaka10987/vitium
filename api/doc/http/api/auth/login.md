# HTTP Route `/api/auth/login`

## GET

Returns a newly generated token using as cookie `token`, if successfully logged in with [HTTP Basic Authentication](https://developer.mozilla.org/en-US/docs/Web/HTTP/Authentication).

### Request Header
`Authorization` --- password login credentials

### Response Header
`Set-Cookie` --- the token returned
