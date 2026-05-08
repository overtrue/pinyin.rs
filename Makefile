# Pinyin.rs Makefile
# 简化常用的开发和构建任务

.PHONY: help build test clean fmt clippy bench install release-local check-all

# 默认目标
help:
	@echo "Pinyin.rs 开发工具"
	@echo ""
	@echo "可用命令:"
	@echo "  build          构建项目"
	@echo "  test           运行测试"
	@echo "  bench          运行基准测试"
	@echo "  clean          清理构建文件"
	@echo "  fmt            格式化代码"
	@echo "  clippy         运行 clippy 检查"
	@echo "  install        安装到系统"
	@echo "  release-local  本地构建 release 版本"
	@echo "  check-all      运行所有检查（测试、格式、clippy）"
	@echo "  help           显示此帮助信息"

# 构建项目
build:
	@echo "🔨 构建项目..."
	cargo build

# 运行测试
test:
	@echo "🧪 运行测试..."
	cargo test

# 运行基准测试
bench:
	@echo "📊 运行基准测试..."
	cargo bench

# 清理构建文件
clean:
	@echo "🧹 清理构建文件..."
	cargo clean

# 格式化代码
fmt:
	@echo "✨ 格式化代码..."
	cargo fmt

# 检查代码格式
fmt-check:
	@echo "🔍 检查代码格式..."
	cargo fmt --check

# 运行 clippy
clippy:
	@echo "📎 运行 clippy 检查..."
	cargo clippy --all-targets --all-features -- -D warnings

# 安装到系统
install:
	@echo "📦 安装到系统..."
	cargo install --path .

# 本地构建 release 版本
release-local:
	@echo "🚀 本地构建 release 版本..."
	./scripts/build.sh

# 运行所有检查
check-all: fmt-check clippy test
	@echo "✅ 所有检查完成！"

# 快速开发循环
dev: fmt clippy test
	@echo "🔄 开发检查完成！"

# 准备发布
prepare-release: check-all release-local
	@echo "🎯 发布准备完成！"
	@echo ""
	@echo "下一步："
	@echo "1. 更新版本号: Cargo.toml"
	@echo "2. 提交更改: git commit -am 'Bump version to x.y.z'"
	@echo "3. 创建标签: git tag vx.y.z"
	@echo "4. 推送标签: git push origin vx.y.z"

# 性能测试
perf:
	@echo "⚡ 运行性能测试..."
	cargo run --example longtext_performance --release

# 运行长文本基准测试
longtext-bench:
	@echo "📈 运行长文本基准测试..."
	cargo bench --bench longtext_benchmark

# 查看帮助
usage:
	@echo "📚 查看命令行工具用法..."
	cargo run -- --help

# 测试命令行工具
test-cli:
	@echo "🔧 测试命令行工具..."
	@echo "基本转换:"
	cargo run -- "你好世界"
	@echo ""
	@echo "数字声调:"
	cargo run -- -t number "你好"
	@echo ""
	@echo "permalink:"
	cargo run -- --permalink "中华人民共和国"
	@echo ""
	@echo "缩写:"
	cargo run -- --abbr "北京大学"

# 文档生成
docs:
	@echo "📖 生成文档..."
	cargo doc --open

# 检查依赖更新
update-deps:
	@echo "🔄 检查依赖更新..."
	cargo update

# 安全审计
audit:
	@echo "🔒 运行安全审计..."
	cargo audit

# 覆盖率测试（需要安装 tarpaulin）
coverage:
	@echo "📊 运行覆盖率测试..."
	cargo tarpaulin --out Html

# 交叉编译测试（需要安装 cross）
cross-build:
	@echo "🌐 交叉编译测试..."
	cross build --target x86_64-unknown-linux-musl --release
