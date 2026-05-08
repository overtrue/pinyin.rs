# Pinyin.rs

[![CI](https://github.com/overtrue/pinyin.rs/actions/workflows/ci.yml/badge.svg)](https://github.com/overtrue/pinyin.rs/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/pinyin-converter.svg)](https://crates.io/crates/pinyin-converter)
[![Documentation](https://docs.rs/pinyin-converter/badge.svg)](https://docs.rs/pinyin-converter)
[![License](https://img.shields.io/crates/l/pinyin-converter.svg)](LICENSE)

中文转拼音 Rust 库和命令行工具，词语优先匹配，支持多音字、姓氏、护照姓名、slug、缩写和多种声调格式。词库来自 [mozillazg/pinyin-data](https://github.com/mozillazg/pinyin-data) 及项目补丁数据。

## Features

- 词语优先：最长词匹配，`你好世界` 会输出 `nǐ hǎo shì jiè`，不会逐字泄露 `好` 的多音字。
- 常用 API：句子、短语、姓名、护照姓名、permalink、首字母缩写、单字和 heteronym。
- 格式完整：支持声调符号、数字声调、无声调，支持 `ü` / `v` / `yu` / `u` 输出。
- CLI 可用：支持参数输入和 stdin 管道输入。
- 构建干净：词库在 Cargo `OUT_DIR` 生成，构建过程不会改写源码目录。

## Install

Requires Rust 1.95 or newer.

Library:

```toml
[dependencies]
pinyin-converter = "0.1"
```

The package is named `pinyin-converter`; the Rust library name is `pinyin`:

```rust
use pinyin::Pinyin;
```

CLI:

```bash
cargo install pinyin-converter
```

## Quick Start

```rust
use pinyin::{convert, Converter, Pinyin, ToneStyle};

fn main() {
    assert_eq!(convert("你好世界"), vec!["nǐ hǎo", "shì jiè"]);

    assert_eq!(Pinyin::sentence("你好，世界").to_string(), "nǐ hǎo ， shì jiè");
    assert_eq!(Pinyin::phrase("你好，世界").to_string(), "nǐ hǎo shì jiè");
    assert_eq!(Pinyin::permalink("带着希望去旅行"), "dai-zhe-xi-wang-qu-lv-xing");
    assert_eq!(Pinyin::abbr("北京大学").to_string(), "b j d x");
    assert_eq!(Pinyin::name("单某某").to_string(), "shàn mǒu mǒu");
    assert_eq!(Pinyin::passport_name("吕秀才").to_string(), "lyu xiu cai");

    let numbered = Converter::new("你好")
        .with_tone_style(ToneStyle::Number)
        .to_string();
    assert_eq!(numbered, "ni3 hao3");
}
```

## CLI

```bash
pinyin "你好世界"
# nǐ hǎo shì jiè

pinyin -t number "你好"
# ni3 hao3

pinyin -t none "旅行"
# lv xing

pinyin --permalink "带着希望去旅行"
# dai-zhe-xi-wang-qu-lv-xing

pinyin --abbr "北京大学"
# b j d x

pinyin --name "单某某"
# shàn mǒu mǒu

pinyin --passport "吕秀才"
# lyu xiu cai

echo "中国" | pinyin
# zhōng guó
```

Run `pinyin --help` for all options.

## API

### Low-Level Conversion

`convert(input)` returns matched pinyin segments and keeps unmatched characters:

```rust
use pinyin::convert;

assert_eq!(convert("中国人喜欢中国吃饭"), vec![
    "zhōng guó rén",
    "xǐ huan",
    "zhōng guó",
    "chī fàn",
]);
assert_eq!(convert("Hi!"), vec!["H", "i", "!"]);
```

Use `convert_safe` or `convert_with_config` when input limits should be validated:

```rust
use pinyin::{convert_with_config, PinyinConfig};

let config = PinyinConfig::new().with_max_length(100);
let result = convert_with_config("中国", &config).unwrap();
assert_eq!(result, vec!["zhōng guó"]);
```

### Pinyin

```rust
use pinyin::Pinyin;

Pinyin::sentence("你好，世界");          // nǐ hǎo ， shì jiè
Pinyin::phrase("你好，世界");            // nǐ hǎo shì jiè
Pinyin::permalink("带着希望去旅行");     // dai-zhe-xi-wang-qu-lv-xing
Pinyin::permalink_with("你好世界", "_").unwrap(); // ni_hao_shi_jie
Pinyin::abbr("北京大学");                // b j d x
Pinyin::name("单某某");                  // shàn mǒu mǒu
Pinyin::name_abbr("单某某");             // s m m
Pinyin::passport_name("吕秀才");          // lyu xiu cai
Pinyin::chars("重庆");                   // zhòng qìng
Pinyin::heteronym("重");                 // [('重', ["zhòng", "chóng", ...])]
```

### Converter

```rust
use pinyin::{Converter, ToneStyle};

assert_eq!(Converter::new("旅行").to_string(), "lǚ xíng");
assert_eq!(Converter::new("旅行").without_tone().to_string(), "lv xing");
assert_eq!(Converter::new("旅行").without_tone().yu_to_yu().to_string(), "lyu xing");
assert_eq!(Converter::new("你好").with_tone_style(ToneStyle::Number).to_string(), "ni3 hao3");
assert_eq!(Converter::new("Hello世界123").only_hans().to_string(), "shì jiè");
```

## Development

```bash
cargo fmt -- --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -- "你好世界"
```

The build script reads files under `sources/` and writes generated data into Cargo's `OUT_DIR`. Do not edit generated files under `target/`.

## License

MIT
