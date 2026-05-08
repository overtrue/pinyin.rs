# GitHub Actions 自动构建配置

本文档说明了 Pinyin.rs 项目的 GitHub Actions 自动构建流水线配置。

## 🚀 功能概述

GitHub Actions 工作流会自动为多个平台构建二进制文件，并在发布时自动上传到 GitHub Releases。
构建工具链固定为 Rust 1.95.0。

## 📦 支持的平台

### Linux

- `x86_64-unknown-linux-gnu` - Linux x64 (glibc)
- `x86_64-unknown-linux-musl` - Linux x64 (musl, 静态链接)
- `aarch64-unknown-linux-gnu` - Linux ARM64 (glibc)
- `aarch64-unknown-linux-musl` - Linux ARM64 (musl, 静态链接)

### macOS

- `x86_64-apple-darwin` - macOS Intel
- `aarch64-apple-darwin` - macOS Apple Silicon (M1/M2)

### Windows

- `x86_64-pc-windows-msvc` - Windows x64 (MSVC)

## 🔧 触发方式

### 1. 标签发布（推荐）

当推送以 `v` 开头的标签时自动触发：

```bash
# 创建并推送标签
git tag v0.1.0
git push origin v0.1.0
```

### 2. 手动触发

在 GitHub 网页上手动触发：

1. 进入 GitHub 仓库页面
2. 点击 "Actions" 标签
3. 选择 "Release" 工作流
4. 点击 "Run workflow"
5. 输入要发布的标签名

## 📁 文件结构

```
.github/
└── workflows/
    └── release.yml          # 发布工作流
scripts/
├── build.sh                 # 本地构建脚本
└── benchmark.sh             # 性能测试脚本
Makefile                     # 开发工具
RELEASE_GUIDE.md             # 发布指南
```

## 🛠️ 本地测试

在推送标签之前，可以本地测试构建：

```bash
# 使用 Makefile
make release-local

# 或直接运行脚本
./scripts/build.sh

# 测试二进制文件
target/release/pinyin --help
target/release/pinyin "测试文本"
```

## 📋 工作流程

1. **代码检查**: 运行 `cargo fmt --check` 和 `cargo clippy`
2. **测试**: 运行完整的测试套件
3. **构建**: 为所有目标平台交叉编译
4. **打包**: 创建压缩包（Linux/macOS: tar.gz, Windows: zip）
5. **发布**: 上传到 GitHub Releases

## 🔍 构建产物

每个平台的构建产物包含：

- 二进制可执行文件 (`pinyin` 或 `pinyin.exe`)
- README.md
- LICENSE 文件

## ⚡ 性能特性

构建的二进制文件具有以下特性：

- **高性能**: 基本转换 13,000+ 字符/秒
- **优化配置**: 配置优化后 44,000+ 字符/秒
- **分块处理**: 大文本处理 206,000+ 字符/秒
- **多平台**: 支持 7 个主要平台
- **静态链接**: musl 版本无需额外依赖

## 🧪 质量保证

每次构建都会：

- ✅ 运行 110+ 个测试用例
- ✅ 通过 clippy 代码检查
- ✅ 验证代码格式
- ✅ 测试基本功能
- ✅ 验证性能指标

## 📖 使用示例

下载后可以直接使用：

```bash
# 基本转换
./pinyin "你好世界"

# 数字声调
./pinyin -t number "你好"

# 生成 permalink
./pinyin -p "中华人民共和国"

# 姓名转换
./pinyin -n "单某某"

# 护照格式
./pinyin --passport "吕小布"

# 管道输入
echo "中国" | ./pinyin
```

## 🔧 故障排除

### 构建失败

1. 检查代码是否通过所有测试
2. 确保 `Cargo.toml` 版本号正确
3. 验证标签格式（必须以 `v` 开头）

### 发布失败

1. 检查 GitHub token 权限
2. 确保仓库有 Releases 权限
3. 验证工作流文件语法

### 下载问题

1. 检查网络连接
2. 确认平台架构匹配
3. 验证文件完整性

## 📚 相关文档

- [发布指南](../RELEASE_GUIDE.md)
- [性能报告](../PERFORMANCE_REPORT.md)
- [开发指南](../README.md)
- [API 文档](https://docs.rs/pinyin-converter)

## 🤝 贡献

如需改进构建流程，请：

1. Fork 仓库
2. 修改 `.github/workflows/release.yml`
3. 测试本地构建
4. 提交 Pull Request

---

*最后更新: 2026年5月*
