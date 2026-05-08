#!/bin/bash

# Pinyin.rs 性能基准测试脚本

echo "=== Pinyin.rs 性能基准测试 ==="
echo

# 检查是否存在长文本文件
if [ ! -f "benches/longtext.txt" ]; then
    echo "错误: 找不到 benches/longtext.txt 文件"
    echo "请确保在项目根目录运行此脚本，并且存在测试文件"
    exit 1
fi

echo "1. 运行长文本性能测试..."
cargo run --example longtext_performance --release

echo
echo "2. 运行基准测试（如果可用）..."
if cargo bench --bench benchmark -- --test > /dev/null 2>&1; then
    echo "运行基本基准测试..."
    cargo bench --bench benchmark convert_single_char convert_words
else
    echo "基准测试不可用或有错误，跳过..."
fi

echo
echo "3. 运行单元测试以确保功能正确性..."
cargo test --release

echo
echo "=== 基准测试完成 ==="
echo
echo "性能报告已保存在 PERFORMANCE_REPORT.md"
echo "如需详细的基准测试，请运行: cargo bench"
