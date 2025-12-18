# 多阶段构建 Dockerfile for fishpi-badge-rust
# 支持多架构: linux/amd64, linux/arm64

# ============ 构建阶段 ============
FROM rust:1.91.0-slim AS builder

# 安装构建依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    binutils \
    && rm -rf /var/lib/apt/lists/*

# 设置工作目录
WORKDIR /app

# 复制依赖文件
COPY Cargo.toml ./

# 创建虚拟 src 用于缓存依赖
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# 复制源代码
COPY src ./src

# 构建应用（清除虚拟构建缓存后重新编译，并优化大小）
RUN rm -rf target/release/deps/fishpi_badge_rust* && \
    cargo build --release && \
    strip target/release/fishpi-badge-rust

# ============ 运行阶段 ============
FROM gcr.io/distroless/cc-debian12

# 设置工作目录
WORKDIR /app

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/fishpi-badge-rust /app/fishpi-badge-rust

# 复制静态资源和模板
COPY static ./static
COPY templates ./templates

# 暴露端口
EXPOSE 3001

# 设置默认环境变量
ENV RUST_LOG=info

# 启动应用（distroless 默认使用 nonroot 用户）
CMD ["/app/fishpi-badge-rust"]
