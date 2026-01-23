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

| 字段  | 标识符    | 描述           | 格式                |
| --- | ------ | ------------ | ----------------- |
| 用户名 | `user` | 对指定服务器的唯一标识符 | 不含空格的非空 ASCII 字符串 |
| 密码  | `pass` | 该用户的登录密码     | 非空 ASCII 字符串      |

同时，用户还可以带有以下可选信息：

| 字段   | 标识符        | 描述              | 格式        |
| ---- | ---------- | --------------- | --------- |
| 电子邮箱 | `email`    | 用户的电子邮箱，可用于身份验证 | 电子邮箱地址    |
| 显示名称 | `nickname` | 在界面中显示的用户名称     | UTF-8 字符串 |
| 头像   | `avatar`   | 在界面中显示的用户头像图片   | URL       |
| 自我介绍 | `intro`    | 自我介绍文本          | HTML      |

### 注册

服务端提供 `/signup` API 用于注册新账号。调用方式为通过 HTTP POST 提交表单。表单应带有正确编码的必要字段 `user` 和 `pass`, 即期望的用户名和密码。

如果输入合法，服务端在正常运作时，作出以下回复：

- HTTP 409, 如果试图注册的用户已经存在，并返回与 GET `https://server.vitium.dev/signup` 一致的页面；或

- HTTP 303 重定向至 `https://server.vitium.dev/` , 并 [初始化会话](#获取会话) 。

### 登录

服务端提供 `/login` API 用于用户登录。调用方式为通过 HTTP POST 提交表单。表单应带有正确编码的必要字段 `user` 和 `pass`, 即用户名和相对应的密码。

如果输入合法，服务端在正常运作时，作出以下回复：

- HTTP 401, 如果试图登录的用户不存在，或提供了错误的密码。返回与 GET `https://server.vitium.dev/login` 一致的页面；或

- HTTP 303 重定向至 `https://server.vitium.dev/` , 并 [初始化会话](#获取会话) 。

### 会话

会话功能通过 HTTP cookie 实现。

#### 获取会话

如果用户成功登录，服务端在成功的登录请求的响应中设置 cookie `token` . `token` 的值是由服务端实现定义的会话标识。同时，服务端应当设置 `HttpOnly` , `Secure` , `SameSite=None` 和 `Partitioned` 的 cookie 属性，并指定一个不超过 30 日的持久化保存寿命。

```http
Set-Cookie: token=0123456789abcdef; HttpOnly; Secure; SameSite=None; Partitioned; Path=/; Max-Age=86400
```

`token` 必须具有能够抵抗爆破攻击的密码学效力。

#### 使用会话

当客户端请求需要身份验证的服务端 API 时，应当在请求中发送 cookie `token` , 原则上浏览器将自动处理。在调用 `fetch` API 时，应当添加 `credentials: "include"` 参数。

:::danger[对抗 [跨站请求伪造攻击](https://developer.mozilla.org/zh-CN/docs/Glossary/CSRF)]

服务端 **必须** 对所有收到的跨域请求添加 `Access-Control-Allow-Origin` 头以禁止 Vitium 客户端以外的任何程序向服务端构造请求。

```http
Access-Control-Allow-Origin: https://client.vitium.dev
```

:::

同时，服务端应当在这些 API 上设置 `Access-Control-Allow-Credentials: true` 头，并根据具体 API 按需设置 `Access-Control-Allow-Methods` , `Access-Control-Allow-Headers` .
