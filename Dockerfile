FROM rust:latest AS builder

WORKDIR /app

# 拷贝项目文件
COPY . .

# 构建项目
RUN cargo build --release

# 构建最终镜像
FROM debian:buster-slim

# 安装所需的系统依赖
RUN apt-get update \
    && apt-get install -y ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 从构建阶段复制可执行文件
COPY --from=builder /app/target/release/webserver .

# 暴露服务器端口
EXPOSE 8080

# 运行服务器
CMD ["./webserver"]