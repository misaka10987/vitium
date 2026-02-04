+++
title = '用户'
[sidebar]
order = 2
+++

一个 **用户** 是现实中的某人在特定 [Vitium 服务端](/concept/structure/#服务端) 的身份。用户可以作为 TRPG 玩家或游戏管理员参与游戏。

用户可以通过网页进行注册和登录。用户名是唯一的。用户可以通过密码进行登录。一旦用户登录，会话将保留直到浏览器关闭，或达到指定的超时期限。

<details>
<summary>为什么我们不采用单点登录？</summary>
Vitium 开发者团队并不想投入额外成本，维护一个集中式的身份验证服务。同时，我们希望自托管服务器的拥有者具有对用户数据的完全控制，而不依赖于第三方服务。这样，服务器管理员将更具有能力，且更方便地操作特定的用户政策，例如迁移和封禁。
</details>

---

## 实现

我们如下编码用户的必要身份证明凭据：

| 字段   | 标识符 | 描述                     | 格式                        |
| ------ | ------ | ------------------------ | --------------------------- |
| 用户名 | `user` | 对指定服务器的唯一标识符 | 不含空格的非空 ASCII 字符串 |
| 密码   | `pass` | 该用户的登录密码         | 非空 ASCII 字符串           |

同时，用户还可以带有以下可选信息：

| 字段     | 标识符     | 描述                           | 格式         |
| -------- | ---------- | ------------------------------ | ------------ |
| 电子邮箱 | `email`    | 用户的电子邮箱，可用于身份验证 | 电子邮箱地址 |
| 显示名称 | `nickname` | 在界面中显示的用户名称         | UTF-8 字符串 |
| 头像     | `avatar`   | 在界面中显示的用户头像图片     | URL          |
| 自我介绍 | `intro`    | 自我介绍文本                   | HTML         |

### 注册

服务端提供 `/signup` API 用于注册新账号。调用方式为通过 HTTP POST 提交表单。表单应带有正确编码的必要字段 `user` 和 `pass`, 即期望的用户名和密码。

如果输入合法，服务端在正常运作时，作出以下回复：

- HTTP 409, 如果试图注册的用户已经存在，并返回与 GET `https://server.vitium.dev/signup` 一致的页面；或

- HTTP 303 重定向至 `https://server.vitium.dev/` , 并 [初始化会话](#获取会话) 。

### 登录与会话

我们采用字符串令牌来标识某个用户的会话。此令牌由服务端生成，具有密码学效力，保存于客户端的 `localStorage` 中。在调用需要用户会话的 API 时，客户端通过 HTTP `Authorization` 头提供令牌。

```http
Authorization: Bearer <token>
```

相应地，服务端应当允许在跨域上下文中使用令牌。

```http
Access-Control-Allow-Headers: Authorization
```

令牌的寿命不作详细规定，但服务端应当向运维人员提供吊销令牌的功能。

我们通过执行 [PKCE](https://datatracker.ietf.org/doc/html/rfc7636) 策略来登录和获取会话。服务端提供 `/login` API 用于用户登录。

为开始用户登录流程，客户端向此路由 HTTP POST 提交表单。表单应带有正确编码的必要字段 `user` 和 `pass`, 即用户名和相对应的密码，用于用户认证。同时，请求还应带有客户端已预先准备的 [`code_challenge`](https://datatracker.ietf.org/doc/html/rfc7636#section-4.2) , 作为查询参数。

我们只支持 SHA256 作为 PKCE 的验证算法。

如果输入合法，服务端在正常运作时，作出以下回复：

- HTTP 401, 如果试图登录的用户不存在，或提供了错误的密码。此时，服务端应当返回与 GET `https://server.vitium.dev/login` 一致的页面；或

- HTTP 303 重定向至 `https://client.vitium.dev/` , 如果登录是成功的。服务端应当按照 [RFC 6749](https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.2) 在查询参数中设置验证码 `code`.

客户端通过检查页面的地址栏获取验证码。一旦获取，客户端应当尽快从 `/access-token` API 获取会话令牌。这通过向其 HTTP POST 提交表单实现。表单应带有 `code` 与 `code_verifier` , 参见 [RFC 6749](https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.3) .

如果输入合法，服务端 [按相应标准](https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.4) 作出回复，在验证成功时发放令牌。客户端应当读取回复获得会话令牌，并将其存储于浏览器 `localStorage` 中。
