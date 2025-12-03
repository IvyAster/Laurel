#!/bin/bash

# 简化版 Docker 构建脚本
#set -e

# 配置
IMAGE_NAME="laurel/${1}"
IMAGE_TAG="${2:-usual}"
DOCKERFILE="Dockerfile-alpine"
FEATURES_PARAMS="${3}"

echo "🔨 构建 Docker 镜像..."
echo "镜像: ${IMAGE_NAME}:${IMAGE_TAG}"

docker rmi "${IMAGE_NAME}:${IMAGE_TAG}" --force
#cargo clean

# 构建镜像
docker build \
    -t "${IMAGE_NAME}:${IMAGE_TAG}" \
    -f "$DOCKERFILE" \
    --build-arg APP_NAME="${1}" \
    --build-arg FEATURES="${FEATURES_PARAMS}" \
    .

echo "✅ 构建完成!"
echo "镜像大小: $(docker images "${IMAGE_NAME}":"${IMAGE_TAG}" --format "table {{.Size}}" | tail -n 1)"

# 运行测试（可选）
#read -p "是否测试镜像? (y/n) " -n 1 -r
#echo
#if [[ $REPLY =~ ^[Yy]$ ]]; then
#    echo "🧪 测试镜像..."
#    docker run --rm -p 8080:8080 "${IMAGE_NAME}:${IMAGE_TAG}" &
#    sleep 5
#    if curl -f http://localhost:8080/api/id > /dev/null 2>&1; then
#        echo "✅ 测试通过"
#        pkill -f "docker run.*${IMAGE_NAME}"  # 停止测试容器
#    else
#        echo "❌ 测试失败"
#        exit 1
#    fi
#fi