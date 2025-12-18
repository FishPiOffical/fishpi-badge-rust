# Docker 部署指南

## 快速开始

### 使用 Docker 构建和运行

```bash
# 构建镜像
docker build -t fishpi-badge-rust:latest .

# 运行容器
docker run -d \
  --name fishpi-badge \
  -p 3001:3001 \
  -e RUST_LOG=info \
  fishpi-badge-rust:latest
```

### 使用 Docker Compose

```bash
# 启动服务
docker-compose up -d

# 查看日志
docker-compose logs -f

# 停止服务
docker-compose down
```

## 配置说明

### 端口

- **3001**: 应用服务端口（HTTP）

### 环境变量

| 变量名     | 默认值 | 说明     | 可选值                                    |
| ---------- | ------ | -------- | ----------------------------------------- |
| `RUST_LOG` | `info` | 日志级别 | `trace`, `debug`, `info`, `warn`, `error` |

### 文件和目录

#### 必需文件（已包含在镜像中）

- `static/` - 静态资源目录
  - `404.html` - 404 错误页面
  - `index.html` - 首页
  - `assets/` - CSS、JS 等资源
    - `daisyui.min.css`
    - `font.js`
    - `main.js`
    - `style.css`
- `templates/` - 模板文件目录
  - `badge.svg` - SVG 徽章模板

#### 可选映射（如需修改静态资源）

```bash
docker run -d \
  --name fishpi-badge \
  -p 3001:3001 \
  -v $(pwd)/static:/app/static:ro \
  -v $(pwd)/templates:/app/templates:ro \
  fishpi-badge-rust:latest
```

## 多架构支持

### 构建多架构镜像

使用 Docker Buildx 构建支持多架构的镜像：

```bash
# 创建构建器
docker buildx create --name multiarch --use

# 构建并推送多架构镜像
docker buildx build --platform linux/amd64,linux/arm64 \
  -t your-registry/fishpi-badge-rust:latest \
  --push .
```

支持的架构：
- `linux/amd64` (x86_64)
- `linux/arm64` (ARM64/aarch64)

## 高级配置

### 自定义日志级别

```bash
# 开发环境：详细日志
docker run -d -p 3001:3001 -e RUST_LOG=debug fishpi-badge-rust:latest

# 生产环境：仅警告和错误
docker run -d -p 3001:3001 -e RUST_LOG=warn fishpi-badge-rust:latest
```

### 资源限制

```bash
docker run -d \
  --name fishpi-badge \
  -p 3001:3001 \
  --memory="512m" \
  --cpus="1.0" \
  fishpi-badge-rust:latest
```

### 健康检查

容器内置健康检查，每 30 秒检查一次服务状态：

```bash
# 查看健康状态
docker inspect --format='{{.State.Health.Status}}' fishpi-badge
```

## 访问应用

服务启动后，可通过以下地址访问：

- 徽章生成器: `http://localhost:3001/gen/maker/`
- API 端点: `http://localhost:3001/gen?参数`

### API 参数示例

```
http://localhost:3001/gen?ver=1&url=https://example.com/avatar.jpg&txt=Hello&size=32&border=3
```

支持的参数：
- `ver` - 版本
- `url` - 头像 URL
- `txt` - 文本内容
- `size` - 尺寸 (默认: 32)
- `border` - 边框宽度 (默认: 3)
- `fontsize` - 字体大小 (默认: 15)
- `barradius` - 圆角半径
- `scale` - 缩放比例 (默认: 1.0)
- `fontcolor` - 字体颜色 (支持 auto 或 hex 颜色)
- `backcolor` - 背景颜色 (hex 或 HSL)
- `shadow` - 阴影强度 (默认: 0.5)
- `anime` - 动画速度 (默认: 0.5)
- `barlen` - 条形长度
- `font` - 字体
- `way` - 颜色模式
- `fontway` - 字体颜色模式

## 故障排查

### 查看容器日志

```bash
docker logs fishpi-badge
```

### 进入容器调试

```bash
docker exec -it fishpi-badge /bin/bash
```

### 重启容器

```bash
docker restart fishpi-badge
```

## 安全建议

1. ✅ 容器以非 root 用户（appuser, UID 1000）运行
2. ✅ 使用最小化的基础镜像（debian:bookworm-slim）
3. ✅ 多阶段构建，减小镜像体积
4. ✅ 包含健康检查
5. ⚠️ 建议在生产环境中使用反向代理（如 Nginx）并启用 HTTPS

## 镜像信息

- 基础镜像: `debian:bookworm-slim`
- Rust 版本: `1.75`
- 最终镜像大小: 约 100-150 MB（取决于架构）
