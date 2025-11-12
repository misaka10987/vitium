+++
title = '发现'
[sidebar]
order = 2
+++

为了一起工作，[Vitium 客户端](/app/structure/#客户端) 和 [服务端](/app/structure/#服务端) 必须先发现和识别对方。这可以被简单地实现。

当服务端选择在返回的页面中嵌入客户端时，它在 HTML 中添加一个指向客户端的 `<iframe>` 标签。进一步地，服务端应当设置 URL 查询参数 `server` 为自己的主机名，并将这个 `<iframe>` 标签的 `id` 置为 `"client"` .

```html
<iframe
    id="client"
    src="https://client.vitium.dev/?server=server.vitium.dev"
></iframe>
```

服务端应当尽可能让 `#client` 在返回的页面中获得尽可能大的显示空间。

在客户端工作时，它应当始终认为自己正在为当前 `server` 参数指定的服务端提供前端界面，不要混淆不同的 `server` . 如果 `server` 参数缺失，客户端应当以适当的方式向用户说明这一点。
