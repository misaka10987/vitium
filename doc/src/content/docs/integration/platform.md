+++
title = '平台支持'
[sidebar]
order = 1
+++

Vitium 希望尽可能地在所有现代桌面环境中运行。为了细化这一标准，我们在此定义一些代表性的支持环境。

## 浏览器

Vitium 使用的所有浏览器特性应当被 [MDN](https://developer.mozilla.org/zh-CN/docs/Glossary/Baseline/Compatibility) 至少标注为「基线最近可用」。如果未标注最近可用，Vitium 必须保证其有能力使用可用特性在不支持的浏览器上作为替代实现，否则必须搁置这一特性。

在此之上，我们额外要求支持开启了严格模式的最新版 Firefox.

## 操作系统

Vitium 必须保证提供在以下操作系统和硬件上正常运行的可执行文件：

- Windows 11 x86-64;

- MacOS 26 AArch64;

- Linux 6 x86-64.\*

\*发行版以最新 [Arch Linux](https://archlinux.org/) 为准。

我们不强制 Vitium 可执行文件可以在上述所有平台上构建（以及交叉构建）。但是，如果 Vitium 不支持某一平台上的构建，则该平台上的可执行文件必须能在 Linux 上被交叉构建。
