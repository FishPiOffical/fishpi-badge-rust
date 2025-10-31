# 鱼排徽章服务

摸鱼派网站所使用的徽章服务，支持自定义图片，渐变颜色及方向等。

> 参考自 [unv-shield](https://github.com/RimoChan/unv-shield) ，基于 Rust 重写并增加支持渐变颜色指定与 Gif 支援。

<a href="https://fishpi.cn/gen?ver=0.1&scale=1&txt=%E6%91%B8%E9%B1%BC%E6%B4%BE&url=https://file.fishpi.cn/logo_raw.png&backcolor=ed8f25&fontcolor=000000" target="_blank">查看示例</a>

## 参数列表

|参数名|含义|示例|默认值|
|---|---|---|---|
| ver | 版本号 | 0.1 |--|
| url | 图片地址 | https://file.fishpi.cn/logo_raw.png |--|
| txt | 徽章文本 | 摸鱼者事竟成|--|
| size | 徽章尺寸 | 32 | 32 |
| border | 边距和阴影扩散范围。 | 3 | 3 |
| barlen | 徽章的文字条的长度。| 100 | 由文字长度决定 |
| fontsize | 字体大小 | 30 | 15 |
| barradius | 文字条圆角大小 | 15 | size / 2 |
| scale | 等比放大倍数 | 2.18 | 1.0 |
| fontcolor | 文字颜色，可以多个实现颜色渐变 | ffffff | 根据背景自动计算 |
| backcolor | 背景色，指定渐变可使用多个颜色值，之间用`,`隔开|ffffff,000000|39241e|
| shadow | 背景阴影的浓度。| 0.9 | 0.5 |
| anime | 动画时间 | 5 | 0.5 |
| way | 渐变方向 | top-left | bottom |

## 调试与发布

执行 `cargo run`，访问 http://127.0.0.1:3000/gen?...

执行 `cargo build --release` 可发布二进制。

## 参考仓库

- [unv-shield](https://github.com/RimoChan/unv-shield)