# Release Guide

## Package

- Cargo package: `pinyin-converter`
- Library import: `pinyin`
- Binary name: `pinyin`

## Local Checks

```bash
cargo fmt --all -- --check
cargo test --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo build --release
target/release/pinyin "你好世界"
```

## Tag Release

```bash
git tag v0.1.0
git push origin v0.1.0
```

The release workflow builds Linux, macOS, and Windows binaries, creates a GitHub release, and publishes to crates.io when the tag workflow has access to `CARGO_REGISTRY_TOKEN`.
