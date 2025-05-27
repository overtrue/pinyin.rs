use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

const DATA_PATH: &str = "data";

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=sources/");

    cleanup()?;
    generate_chars()?;
    generate_words()?;
    generate_surnames()?;
    generate_heteronyms()?;

    println!("Data generation completed successfully");
    Ok(())
}

/// 清理并创建数据目录
fn cleanup() -> Result<()> {
    if Path::new(DATA_PATH).exists() {
        std::fs::remove_dir_all(DATA_PATH)
            .with_context(|| format!("Failed to remove directory: {}", DATA_PATH))?;
    }
    std::fs::create_dir(DATA_PATH)
        .with_context(|| format!("Failed to create directory: {}", DATA_PATH))?;
    Ok(())
}

/// 生成字符数据
fn generate_chars() -> Result<()> {
    println!("Generating character data...");
    let mut data = Vec::new();

    // 加载原始数据和补丁数据
    for path in [
        Path::new("sources/chars.txt"),
        Path::new("sources/patches/chars.txt"),
    ] {
        if !path.exists() {
            println!("Warning: {} does not exist, skipping", path.display());
            continue;
        }

        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {}", path.display()))?;

        for (line_num, line) in content.lines().enumerate() {
            if let Some(item) = parse_line(line) {
                data.push(item);
            } else if !line.trim().is_empty() && !line.trim().starts_with('#') {
                println!("Warning: Failed to parse line {} in {}: {}",
                    line_num + 1, path.display(), line);
            }
        }
    }

    if data.is_empty() {
        return Err(anyhow::anyhow!("No character data found"));
    }

    let chunk_size = div_ceil(data.len(), 10);
    println!("Processing {} characters in chunks of {}", data.len(), chunk_size);

    for (count, (unicode, pinyin)) in data.iter().enumerate() {
        // unicode: "U+4E00"
        let code_point = u32::from_str_radix(&unicode[2..], 16)
            .with_context(|| format!("Invalid Unicode code point: {}", unicode))?;

        let chunk_file_name = format!("chars_{}.txt", count / chunk_size);
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(Path::new(DATA_PATH).join(&chunk_file_name))
            .with_context(|| format!("Failed to open file: {}", chunk_file_name))?;

        if let Some(chinese) = char::from_u32(code_point) {
            writeln!(file, "{}: {}", chinese, pinyin)
                .with_context(|| format!("Failed to write to file: {}", chunk_file_name))?;
        } else {
            println!("Warning: Invalid Unicode code point: {}", unicode);
        }
    }

    println!("Generated {} character files", 10);
    Ok(())
}

/// 生成词汇数据
fn generate_words() -> Result<()> {
    println!("Generating word data...");
    let mut data = HashMap::new();

    // 加载原始数据和补丁数据，补丁数据优先级更高
    for path in [
        Path::new("sources/words.txt"),
        Path::new("sources/patches/words.txt"),
    ] {
        if !path.exists() {
            println!("Warning: {} does not exist, skipping", path.display());
            continue;
        }

        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {}", path.display()))?;

        for (line_num, line) in content.lines().enumerate() {
            if let Some((chinese, pinyin)) = parse_line(line) {
                data.insert(chinese, pinyin);
            } else if !line.trim().is_empty() && !line.trim().starts_with('#') {
                println!("Warning: Failed to parse line {} in {}: {}",
                    line_num + 1, path.display(), line);
            }
        }
    }

    if data.is_empty() {
        return Err(anyhow::anyhow!("No word data found"));
    }

    let chunk_size = div_ceil(data.len(), 10);
    println!("Processing {} words in chunks of {}", data.len(), chunk_size);

    for (count, (chinese, pinyin)) in hashmap_to_sorted_vec(data).iter().enumerate() {
        let chunk_file_name = format!("words_{}.txt", count / chunk_size);
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(Path::new(DATA_PATH).join(&chunk_file_name))
            .with_context(|| format!("Failed to open file: {}", chunk_file_name))?;

        writeln!(file, "{}: {}", chinese, pinyin)
            .with_context(|| format!("Failed to write to file: {}", chunk_file_name))?;
    }

    println!("Generated {} word files", 10);
    Ok(())
}

/// 生成姓氏数据
fn generate_surnames() -> Result<()> {
    println!("Generating surname data...");
    let mut data = Vec::new();

    let path = Path::new("sources/surnames.txt");
    if !path.exists() {
        return Err(anyhow::anyhow!("Surnames file not found: {}", path.display()));
    }

    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;

    for (line_num, line) in content.lines().enumerate() {
        if let Some(item) = parse_line(line) {
            data.push(item);
        } else if !line.trim().is_empty() && !line.trim().starts_with('#') {
            println!("Warning: Failed to parse line {} in {}: {}",
                line_num + 1, path.display(), line);
        }
    }

    if data.is_empty() {
        return Err(anyhow::anyhow!("No surname data found"));
    }

    // 将结果写入文件
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(Path::new(DATA_PATH).join("surnames.txt"))
        .context("Failed to create surnames.txt")?;

    for (chinese, pinyin) in data.iter() {
        writeln!(file, "{}: {}", chinese, pinyin)
            .context("Failed to write surnames to file")?;
    }

    println!("Generated surnames file with {} entries", data.len());
    Ok(())
}

/// 生成多音字数据
fn generate_heteronyms() -> Result<()> {
    println!("Generating heteronym data...");

    let path = Path::new("sources/heteronyms.txt");
    if !path.exists() {
        return Err(anyhow::anyhow!("Heteronyms file not found: {}", path.display()));
    }

    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;

    let data: Vec<&str> = content.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();

    if data.is_empty() {
        return Err(anyhow::anyhow!("No heteronym data found"));
    }

    let data_len = data.len(); // 保存长度用于后续打印

    // 将结果写入文件
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(Path::new(DATA_PATH).join("heteronyms.txt"))
        .context("Failed to create heteronyms.txt")?;

    for item in data {
        writeln!(file, "{}", item)
            .context("Failed to write heteronyms to file")?;
    }

    println!("Generated heteronyms file with {} entries", data_len);
    Ok(())
}

/// 将 HashMap 转换为按键排序的 Vec
fn hashmap_to_sorted_vec(map: HashMap<String, String>) -> Vec<(String, String)> {
    let mut vec: Vec<(String, String)> = map.into_iter().collect();
    vec.sort_by(|a, b| a.0.cmp(&b.0));
    vec
}

/// 解析数据行
///
/// 支持格式：
/// - "字符: 拼音"
/// - "U+4E00: 拼音 # 注释"
fn parse_line(line: &str) -> Option<(String, String)> {
    let line = line.trim();

    // 跳过空行和注释行
    if line.is_empty() || line.starts_with('#') {
        return None;
    }

    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() < 2 {
        return None;
    }

    let chinese = parts[0].trim();
    let pinyin_part = parts[1].trim();

    // 移除注释部分
    let pinyin = pinyin_part
        .split_whitespace()
        .take_while(|s| !s.starts_with('#'))
        .collect::<Vec<&str>>()
        .join(" ");

    if chinese.is_empty() || pinyin.is_empty() {
        return None;
    }

    // 验证拼音格式（基本检查）
    if !is_valid_pinyin(&pinyin) {
        println!("Warning: Invalid pinyin format: {}", pinyin);
        return None;
    }

    Some((chinese.to_string(), pinyin))
}

/// 验证拼音格式
fn is_valid_pinyin(pinyin: &str) -> bool {
    // 基本验证：只包含字母、空格、声调符号
    pinyin.chars().all(|c| {
        c.is_alphabetic() || c.is_whitespace() ||
        "āáǎàēéěèīíǐìōóǒòūúǔùǖǘǚǜ".contains(c)
    })
}

/// 向上取整除法
fn div_ceil(num: usize, denom: usize) -> usize {
    assert!(denom > 0, "Denominator must be greater than 0");
    (num + denom - 1) / denom
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        // 正常格式
        assert_eq!(
            parse_line("中国: zhōng guó"),
            Some(("中国".to_string(), "zhōng guó".to_string()))
        );

        // Unicode 格式
        assert_eq!(
            parse_line("U+4E2D: zhōng # 中"),
            Some(("U+4E2D".to_string(), "zhōng".to_string()))
        );

        // 空行
        assert_eq!(parse_line(""), None);
        assert_eq!(parse_line("   "), None);

        // 注释行
        assert_eq!(parse_line("# 这是注释"), None);

        // 格式错误
        assert_eq!(parse_line("中国"), None);
        assert_eq!(parse_line("中国:"), None);
    }

    #[test]
    fn test_is_valid_pinyin() {
        assert!(is_valid_pinyin("zhōng guó"));
        assert!(is_valid_pinyin("nǐ hǎo"));
        assert!(is_valid_pinyin("a"));

        assert!(!is_valid_pinyin("123"));
        assert!(!is_valid_pinyin("hello@world"));
    }

    #[test]
    fn test_div_ceil() {
        assert_eq!(div_ceil(10, 3), 4);
        assert_eq!(div_ceil(9, 3), 3);
        assert_eq!(div_ceil(1, 1), 1);
        assert_eq!(div_ceil(0, 5), 0);
    }

    #[test]
    #[should_panic]
    fn test_div_ceil_zero_denom() {
        div_ceil(10, 0);
    }
}
