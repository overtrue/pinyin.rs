# Pinyin.rs Makefile
# 参考 PHP 版本的 overtrue/pinyin 项目结构

.PHONY: help build test bench clean install format lint doc benchmark-test

# 默认目标
help:
	@echo "Pinyin.rs 项目管理工具"
	@echo ""
	@echo "可用命令:"
	@echo "  build          - 构建项目"
	@echo "  test           - 运行所有测试"
	@echo "  bench          - 运行基准测试"
	@echo "  benchmark-test - 运行自定义基准测试脚本"
	@echo "  clean          - 清理构建文件"
	@echo "  install        - 安装依赖"
	@echo "  format         - 格式化代码"
	@echo "  lint           - 代码检查"
	@echo "  doc            - 生成文档"
	@echo "  release        - 发布版本"
	@echo ""

# 构建项目
build:
	@echo "构建 Pinyin.rs..."
	cargo build --release

# 运行测试
test:
	@echo "运行单元测试..."
	cargo test --verbose

# 运行基准测试
bench:
	@echo "运行基准测试..."
	cargo bench

# 运行自定义基准测试脚本
benchmark-test:
	@echo "运行自定义基准测试脚本..."
	@if [ -f "benchmark_test.rs" ]; then \
		echo "编译基准测试脚本..."; \
		rustc --edition 2021 -L target/release/deps benchmark_test.rs -o benchmark_test --extern pinyin=target/release/libpinyin.rlib; \
		echo "运行基准测试..."; \
		./benchmark_test; \
		rm -f benchmark_test; \
	else \
		echo "基准测试脚本不存在，请先创建 benchmark_test.rs"; \
	fi

# 清理构建文件
clean:
	@echo "清理构建文件..."
	cargo clean
	rm -f benchmark_test

# 安装依赖
install:
	@echo "安装依赖..."
	cargo fetch

# 格式化代码
format:
	@echo "格式化代码..."
	cargo fmt

# 代码检查
lint:
	@echo "运行代码检查..."
	cargo clippy -- -D warnings

# 生成文档
doc:
	@echo "生成文档..."
	cargo doc --open

# 发布版本
release: clean format lint test
	@echo "准备发布版本..."
	cargo build --release
	@echo "发布完成！"

# 完整测试套件（参考 PHP 版本）
test-all: format lint test bench benchmark-test
	@echo "所有测试完成！"

# 性能分析
profile:
	@echo "运行性能分析..."
	cargo build --release
	@if command -v perf >/dev/null 2>&1; then \
		echo "使用 perf 进行性能分析..."; \
		perf record --call-graph=dwarf cargo test --release; \
		perf report; \
	else \
		echo "perf 工具未安装，跳过性能分析"; \
	fi

# 内存检查
memory-check:
	@echo "运行内存检查..."
	@if command -v valgrind >/dev/null 2>&1; then \
		echo "使用 valgrind 进行内存检查..."; \
		cargo build; \
		valgrind --tool=memcheck --leak-check=full cargo test; \
	else \
		echo "valgrind 工具未安装，跳过内存检查"; \
	fi

# 代码覆盖率
coverage:
	@echo "生成代码覆盖率报告..."
	@if command -v cargo-tarpaulin >/dev/null 2>&1; then \
		cargo tarpaulin --out Html; \
		echo "覆盖率报告已生成到 tarpaulin-report.html"; \
	else \
		echo "cargo-tarpaulin 未安装，请运行: cargo install cargo-tarpaulin"; \
	fi

# 安全审计
audit:
	@echo "运行安全审计..."
	@if command -v cargo-audit >/dev/null 2>&1; then \
		cargo audit; \
	else \
		echo "cargo-audit 未安装，请运行: cargo install cargo-audit"; \
	fi

# 依赖更新
update:
	@echo "更新依赖..."
	cargo update

# 检查过时依赖
outdated:
	@echo "检查过时依赖..."
	@if command -v cargo-outdated >/dev/null 2>&1; then \
		cargo outdated; \
	else \
		echo "cargo-outdated 未安装，请运行: cargo install cargo-outdated"; \
	fi

# 开发环境设置
dev-setup:
	@echo "设置开发环境..."
	rustup component add rustfmt clippy
	cargo install cargo-tarpaulin cargo-audit cargo-outdated
	@echo "开发环境设置完成！"

# 快速测试（类似 PHP 版本的快速验证）
quick-test:
	@echo "快速功能测试..."
	@echo "测试基本转换功能..."
	@cargo run --example basic_usage 2>/dev/null || echo "示例程序不存在"
	@echo "运行核心测试..."
	cargo test --lib --quiet
	@echo "快速测试完成！"
