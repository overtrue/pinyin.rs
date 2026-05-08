use pinyin::{PinyinConfig, convert, convert_safe, convert_with_config, match_word_pinyin};

/// 基本用法示例 - 参考 PHP 版本的功能演示
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Pinyin.rs 基本使用示例 ===\n");

    // 1. 基本转换
    println!("1. 基本转换:");
    basic_conversion();
    println!();

    // 2. 安全转换（带错误处理）
    println!("2. 安全转换:");
    safe_conversion();
    println!();

    // 3. 配置化转换
    println!("3. 配置化转换:");
    config_conversion()?;
    println!();

    // 4. 词汇匹配
    println!("4. 词汇匹配:");
    word_matching();
    println!();

    // 5. 特殊用例
    println!("5. 特殊用例:");
    special_cases();
    println!();

    // 6. 性能测试
    println!("6. 性能测试:");
    performance_test()?;

    Ok(())
}

fn basic_conversion() {
    let examples = vec![
        "中国",
        "你好世界",
        "中华人民共和国",
        "Hello世界123",
        "中国人民喜欢在中国吃饭",
    ];

    for text in examples {
        let result = convert(text);
        println!("  {} -> {}", text, result.join(" "));
    }
}

fn safe_conversion() {
    let examples = vec![
        "中国人民",
        "", // 空字符串
        "测试文本",
    ];

    for text in examples {
        match convert_safe(text) {
            Ok(result) => {
                if result.is_empty() {
                    println!("  \"{}\" -> (空结果)", text);
                } else {
                    println!("  \"{}\" -> {}", text, result.join(" "));
                }
            }
            Err(e) => {
                println!("  \"{}\" -> 错误: {}", text, e);
            }
        }
    }
}

fn config_conversion() -> Result<(), Box<dyn std::error::Error>> {
    let text = "中华人民共和国万岁";

    // 默认配置
    let default_config = PinyinConfig::new().with_max_length(1000);

    let result = convert_with_config(text, &default_config)?;
    println!("  默认配置: {} -> {}", text, result.join(" "));

    // 关闭词组匹配，只按单字转换
    let char_config = PinyinConfig::new()
        .with_long_words(false)
        .with_max_length(1000);

    let result = convert_with_config(text, &char_config)?;
    println!("  单字配置: {} -> {}", text, result.join(" "));

    // 自定义配置
    let custom_config = PinyinConfig::new()
        .with_polyphone(true)
        .with_long_words(true)
        .with_max_length(5000);

    let result = convert_with_config(text, &custom_config)?;
    println!("  自定义配置: {} -> {}", text, result.join(" "));

    Ok(())
}

fn word_matching() {
    let text = "中国人民";
    let matches = match_word_pinyin(text);

    println!("  输入: {}", text);
    println!("  匹配结果:");
    for (word, pinyin) in matches.iter().take(8) {
        // 只显示前8个结果
        println!("    {} -> {}", word, pinyin);
    }
    if matches.len() > 8 {
        println!("    ... 还有 {} 个匹配结果", matches.len() - 8);
    }
}

fn special_cases() {
    // 姓氏处理
    println!("  姓氏处理:");
    let surnames = vec!["曾国藩", "单于丹", "区志华", "仇大雄"];
    for name in surnames {
        let result = convert(name);
        println!("    {} -> {}", name, result.join(" "));
    }

    // 多音字处理
    println!("  多音字处理:");
    let polyphones = vec!["银行", "行走", "重庆", "重量", "调调"];
    for word in polyphones {
        let result = convert(word);
        println!("    {} -> {}", word, result.join(" "));
    }

    // 混合内容
    println!("  混合内容:");
    let mixed = vec!["Hello世界", "2024年春节", "αβγ中文"];
    for text in mixed {
        let result = convert(text);
        println!("    {} -> {:?}", text, result);
    }

    // 生成缩写
    println!("  拼音缩写:");
    let texts = vec!["中华人民共和国", "北京大学", "清华大学"];
    for text in &texts {
        let abbr = generate_abbreviation(text);
        println!("    {} -> {}", text, abbr);
    }

    // URL 友好格式
    println!("  URL 友好格式:");
    for text in &texts {
        let permalink = to_permalink(text);
        println!("    {} -> {}", text, permalink);
    }
}

fn performance_test() -> Result<(), Box<dyn std::error::Error>> {
    use std::time::Instant;

    let short_text = "中国人民";
    let medium_text = "中国人民喜欢在中国吃饭，中国人的口味，中国饭好吃";
    let long_text = medium_text.repeat(100);

    // 短文本测试
    let start = Instant::now();
    let _result = convert(short_text);
    let short_duration = start.elapsed();
    println!(
        "  短文本 ({} 字符): {:?}",
        short_text.chars().count(),
        short_duration
    );

    // 中等文本测试
    let start = Instant::now();
    let _result = convert(medium_text);
    let medium_duration = start.elapsed();
    println!(
        "  中等文本 ({} 字符): {:?}",
        medium_text.chars().count(),
        medium_duration
    );

    // 长文本配置转换
    let config = PinyinConfig::new().with_max_length(long_text.len() + 1);
    let start = Instant::now();
    let _result = convert_with_config(&long_text, &config)?;
    let configured_duration = start.elapsed();
    println!(
        "  长文本配置转换 ({} 字符): {:?}",
        long_text.chars().count(),
        configured_duration
    );

    // 长文本单字转换
    let char_config = PinyinConfig::new()
        .with_long_words(false)
        .with_max_length(long_text.len() + 1);
    let start = Instant::now();
    let _result = convert_with_config(&long_text, &char_config)?;
    let char_duration = start.elapsed();
    println!(
        "  长文本单字转换 ({} 字符): {:?}",
        long_text.chars().count(),
        char_duration
    );

    Ok(())
}

// 辅助函数：生成拼音缩写
fn generate_abbreviation(text: &str) -> String {
    convert(text)
        .iter()
        .filter_map(|pinyin| {
            pinyin
                .chars()
                .find(|c| c.is_ascii_alphabetic())
                .map(|c| c.to_lowercase().to_string())
        })
        .collect::<Vec<_>>()
        .join("")
}

// 辅助函数：生成 URL 友好格式
fn to_permalink(text: &str) -> String {
    convert(text)
        .iter()
        .map(|s| remove_tones(s))
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

// 辅助函数：移除声调符号
fn remove_tones(pinyin: &str) -> String {
    pinyin
        .replace("ā", "a")
        .replace("á", "a")
        .replace("ǎ", "a")
        .replace("à", "a")
        .replace("ē", "e")
        .replace("é", "e")
        .replace("ě", "e")
        .replace("è", "e")
        .replace("ī", "i")
        .replace("í", "i")
        .replace("ǐ", "i")
        .replace("ì", "i")
        .replace("ō", "o")
        .replace("ó", "o")
        .replace("ǒ", "o")
        .replace("ò", "o")
        .replace("ū", "u")
        .replace("ú", "u")
        .replace("ǔ", "u")
        .replace("ù", "u")
        .replace("ǖ", "v")
        .replace("ǘ", "v")
        .replace("ǚ", "v")
        .replace("ǜ", "v")
        .replace(" ", "")
}
