# HTTP Routes

Only successful requests (i.e. 200 OK) are documented.

If an item is not documented, it means that such item does not exist / is empty. For example, empty HTTP header/body, or JSON which values to `{}`, `[]`, `""` or `null`.

See [/http]() for more details.

---

`/` for customized homepage redirect

`/api` for the client

|--- `/auth` authentication related operations

|    |--- `/login` for password login

|    |--- `/pass` for password modification

|--- `/hello` for a "Hello, world!" message

|--- `/chat` for outside-game chats

|--- `/player` for player browsing and modification

|--- |--- `/<name>` for a specified player

|--- `/pc` for player-character browsing and modification

|--- |--- `/<name>` for a specified player character

|--- `/cmd` for command execution

|--- `/game` for in-game operations

|    |--- `/act` for submitting actions

|    |--- `/sync` for synchronization
