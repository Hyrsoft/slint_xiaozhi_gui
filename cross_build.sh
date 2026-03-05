#!/bin/bash
# 使用 cross 交叉编译到嵌入式 ARM Linux 设备 (musl 静态链接)
# 通过 DRM/KMS 后端 (linuxkms-noseat) 直接渲染到 /dev/dri/card0
#
# 前置条件:
#   1. 安装 Docker (cross 依赖 Docker 容器进行交叉编译)
#   2. 安装 cross:
#      cargo install cross --git https://github.com/cross-rs/cross
#   3. 安装 Rust 目标:
#      rustup target add armv7-unknown-linux-musleabihf
#
# 用法: ./cross_build.sh [debug|release]

set -e

BUILD_MODE="${1:-release}"
TARGET="armv7-unknown-linux-musleabihf"

echo "============================================"
echo "  使用 cross 交叉编译 slint_xiaozhi_gui"
echo "  目标: ${TARGET} (musl 静态链接)"
echo "  模式: ${BUILD_MODE}"
echo "============================================"

if [ "$BUILD_MODE" = "release" ]; then
    cross build --target ${TARGET} --features embedded --no-default-features --release
    BINARY_PATH="target/${TARGET}/release/slint_xiaozhi_gui"
else
    cross build --target ${TARGET} --features embedded --no-default-features
    BINARY_PATH="target/${TARGET}/debug/slint_xiaozhi_gui"
fi

echo ""
echo "============================================"
echo "  编译完成!"
echo "  产物: ${BINARY_PATH}"
echo "============================================"
echo ""
echo "部署到设备:"
echo "  scp ${BINARY_PATH} user@device:/path/to/"
echo ""
echo "在设备上运行:"
echo "  SLINT_FULLSCREEN=1 ./slint_xiaozhi_gui"
echo ""
echo "指定 DRM 设备:"
echo "  SLINT_FULLSCREEN=1 DRM_DEVICE=/dev/dri/card0 ./slint_xiaozhi_gui"
