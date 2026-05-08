#!/bin/bash

# Pinyin.rs 本地构建脚本
# 用于测试二进制文件构建

set -e

echo "🚀 开始构建 Pinyin.rs 二进制文件..."

# 检查 Rust 环境
if ! command -v cargo &> /dev/null; then
    echo "❌ 错误: 未找到 cargo，请先安装 Rust"
    exit 1
fi

# 清理之前的构建
echo "🧹 清理之前的构建..."
cargo clean

# 构建 release 版本
echo "🔨 构建 release 版本..."
cargo build --release

# 检查构建结果
if [ -f "target/release/pinyin" ] || [ -f "target/release/pinyin.exe" ]; then
    echo "✅ 构建成功！"

    # 显示二进制文件信息
    if [ -f "target/release/pinyin" ]; then
        BINARY_PATH="target/release/pinyin"
    else
        BINARY_PATH="target/release/pinyin.exe"
    fi

    echo "📁 二进制文件位置: $BINARY_PATH"
    echo "📊 文件大小: $(du -h "$BINARY_PATH" | cut -f1)"

    # 测试基本功能
    echo "🧪 测试基本功能..."
    echo "测试 --version:"
    "$BINARY_PATH" --version

    echo ""
    echo "测试基本转换:"
    "$BINARY_PATH" "你好世界"

    echo ""
    echo "测试 permalink:"
    "$BINARY_PATH" --permalink "中华人民共和国"

    echo ""
    echo "测试缩写:"
    "$BINARY_PATH" --abbr "北京大学"

    echo ""
    echo "✅ 所有测试通过！"

else
    echo "❌ 构建失败！"
    exit 1
fi

echo ""
echo "🎉 构建完成！你可以使用以下命令运行："
echo "   $BINARY_PATH --help"
