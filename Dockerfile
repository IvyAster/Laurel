# 构建阶段
FROM rust:alpine3.22 AS builder

# Alpine下需要安装musl-dev来构建（如果你依赖一些C库的话）
#RUN apk add --no-cache musl-dev
#RUN apk add --no-cache protobuf-compiler
#    protoc \
#    protobuf-dev \

# 安装 protoc 和其他构建依赖
RUN apk add --no-cache \
    libgcc \
    build-base \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    postgresql-dev

# 验证安装
#RUN protoc --version
ARG APP_NAME

WORKDIR /app

COPY . .

RUN cargo build -p laurel-${APP_NAME} --release

# 运行时阶段
FROM alpine:latest

ARG APP_NAME

# 安装运行时依赖（例如，ca-certificates，如果你需要SSL）
RUN apk add --no-cache ca-certificates

# 创建非root用户
RUN addgroup -S app && adduser -S app -G app

USER app
WORKDIR /app
RUN mkdir config

COPY --from=builder /app/target/release/laurel-${APP_NAME} /app/app
COPY --from=builder /app/laurel-${APP_NAME}/config/app.toml ./config/
#COPY app.toml .

#EXPOSE 8080
#EXPOSE 18080

CMD ["./app"]