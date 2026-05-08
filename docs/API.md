# API Reference

## Package Name

Cargo package: `pinyin-converter`

Rust version: 1.95 or newer

Rust library import:

```rust
use pinyin::Pinyin;
```

## Low-Level API

```rust
use pinyin::{convert, convert_safe, convert_with_config, PinyinConfig};

assert_eq!(convert("你好世界"), vec!["nǐ hǎo", "shì jiè"]);
assert_eq!(convert("Hi!"), vec!["H", "i", "!"]);

let config = PinyinConfig::new().with_max_length(100);
let result = convert_with_config("中国", &config).unwrap();
assert_eq!(result, vec!["zhōng guó"]);

let safe = convert_safe("中国").unwrap();
assert_eq!(safe, vec!["zhōng guó"]);
```

## High-Level API

```rust
use pinyin::Pinyin;

assert_eq!(Pinyin::sentence("你好，世界").to_string(), "nǐ hǎo ， shì jiè");
assert_eq!(Pinyin::phrase("你好，世界").to_string(), "nǐ hǎo shì jiè");
assert_eq!(Pinyin::permalink("带着希望去旅行"), "dai-zhe-xi-wang-qu-lv-xing");
assert_eq!(Pinyin::abbr("北京大学").to_string(), "b j d x");
assert_eq!(Pinyin::name("单某某").to_string(), "shàn mǒu mǒu");
assert_eq!(Pinyin::name_abbr("单某某").to_string(), "s m m");
assert_eq!(Pinyin::passport_name("吕秀才").to_string(), "lyu xiu cai");
assert_eq!(Pinyin::chars("重庆").to_string(), "zhòng qìng");
```

## Converter

```rust
use pinyin::{Converter, ToneStyle};

assert_eq!(Converter::new("旅行").to_string(), "lǚ xíng");
assert_eq!(Converter::new("旅行").without_tone().to_string(), "lv xing");
assert_eq!(
    Converter::new("你好")
        .with_tone_style(ToneStyle::Number)
        .to_string(),
    "ni3 hao3"
);
```
