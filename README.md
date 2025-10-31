# 鱼排徽章服务

摸鱼派网站所使用的徽章服务，支持自定义图片，渐变颜色及方向等。

> 参考自 [unv-shield](https://github.com/RimoChan/unv-shield) ，基于 Rust 重写并增加支持渐变颜色指定与 Gif 支援。

<a href="https://fishpi.cn/gen?ver=0.1&scale=1&border=5&txt=%E6%91%B8%E9%B1%BC%E8%80%85%E4%BA%8B%E7%AB%9F%E6%88%90&url=https://file.fishpi.cn/logo_raw.png&backcolor=ed8f25&fontcolor=ffffff" target="_blank">查看示例</a>

## 参数列表

| 参数名 | 说明 | 允许值/范围 | 示例值 |
|-------------|--------------------------------|-----------------------------------------------------------------------------------------------|---------------------------------------------|
| ver | 接口版本号 | 字符串（建议数字或版本号格式） | `0.1` |
| scale | 缩放比例 | 数字（整数或小数，建议范围0.1~10） | `0.79` |
| txt | 显示文本 | 任意字符串（建议URL编码） | `00后tes` |
| url | 图片地址 | 合法URL字符串 | `https://file.fishpi.cn/2024/03/zhuanquanquan-3e16db97.gif` |
| backcolor | 背景色（支持多色渐变） | 多个6位16进制色值（用英文逗号分隔），或单独`auto`，不能与颜色混用 | `ffffff,000000,ffa500,ff0000` 或 `auto` |
| fontcolor | 字体颜色（支持多色渐变） | 多个6位16进制色值（用英文逗号分隔），或单独`auto`，不能与颜色混用 | `ffffff,000000` 或 `auto` |
| shadow | 背景阴影浓度 | 数字（整数或小数，建议范围0~1，非法值默认0） | `0.8` |
| anime | 动画时间（秒） | 数字（整数或小数，建议范围0.1~10，非法值默认0） | `5` |
| way | 渐变方向 | 方向字符串（`top`、`bottom`、`left`、`right`、`top-left`、`top-right`、`bottom-left`、`bottom-right`），或角度（`0deg`~`359deg`），非法值默认`bottom` | `top-left` 或 `45deg` |
| fontway | 字体渐变方向 | 同`way` | `bottom` 或 `120deg` |

---

### 详细说明

- **backcolor/fontcolor**：
 - 允许多个6位16进制色值（如`ffffff,000000`），用英文逗号分隔。
 - 允许单独`auto`，但不能与颜色混用（如`auto,ffffff`非法）。
- **shadow/anime**：
 - 仅允许数字（整数或小数），非法值自动转为`0`。
- **way/fontway**：
 - 允许方向字符串或角度（如`45deg`），非法值自动转为`bottom`。
- **其它参数**：
 - 建议做URL编码，防止特殊字符导致解析异常。

---

## 调试与发布

执行 `cargo run`，访问 http://127.0.0.1:3000/gen?...

执行 `cargo build --release` 可发布二进制。

## 参考仓库

- [unv-shield](https://github.com/RimoChan/unv-shield)