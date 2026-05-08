use pinyin::{PinyinConfig, PinyinError, convert, convert_with_config};
use std::collections::HashMap;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Pinyin.rs 高级使用示例 ===\n");

    // 1. 错误处理最佳实践
    println!("1. 错误处理最佳实践:");
    error_handling_examples();
    println!();

    // 2. 性能优化技巧
    println!("2. 性能优化技巧:");
    performance_optimization()?;
    println!();

    // 3. 批量处理
    println!("3. 批量处理:");
    batch_processing()?;
    println!();

    // 4. 自定义拼音处理器
    println!("4. 自定义拼音处理器:");
    custom_processors();
    println!();

    // 5. 多线程使用
    println!("5. 多线程使用:");
    multithreaded_usage();
    println!();

    // 6. 内存优化
    println!("6. 内存优化:");
    memory_optimization();
    println!();

    Ok(())
}

fn error_handling_examples() {
    // 创建一个安全的转换函数
    fn safe_convert_with_fallback(input: &str) -> Vec<String> {
        match pinyin::convert_safe(input) {
            Ok(result) => result,
            Err(PinyinError::InvalidInput(msg)) => {
                eprintln!("输入错误: {}", msg);
                vec!["[ERROR]".to_string()]
            }
            Err(PinyinError::ConfigError(msg)) => {
                eprintln!("配置错误: {}", msg);
                vec!["[CONFIG_ERROR]".to_string()]
            }
            Err(e) => {
                eprintln!("未知错误: {}", e);
                vec!["[UNKNOWN_ERROR]".to_string()]
            }
        }
    }

    let test_cases = vec!["中国", "", "测试"];
    for case in test_cases {
        let result = safe_convert_with_fallback(case);
        println!("  \"{}\" -> {:?}", case, result);
    }

    // 配置验证示例
    let invalid_config = PinyinConfig::new().with_max_length(0);
    match invalid_config.validate() {
        Ok(_) => println!("  配置有效"),
        Err(e) => println!("  配置无效: {}", e),
    }
}

fn performance_optimization() -> Result<(), Box<dyn std::error::Error>> {
    let test_text = "中国人民喜欢在中国吃饭，中国人的口味，中国饭好吃".repeat(50);

    // 1. 选择合适的配置
    println!("  配置对比:");

    // 小文本 - 默认配置
    let small_text = "中国人民";
    let default_config = PinyinConfig::new();
    let char_config = PinyinConfig::new().with_long_words(false);

    let start = Instant::now();
    let _result = convert_with_config(small_text, &default_config)?;
    let default_time = start.elapsed();

    let start = Instant::now();
    let _result = convert_with_config(small_text, &char_config)?;
    let char_time = start.elapsed();

    println!(
        "    小文本 - 默认: {:?}, 单字: {:?}",
        default_time, char_time
    );

    // 大文本 - 词组匹配通常更准确
    let start = Instant::now();
    let _result = convert_with_config(&test_text, &default_config)?;
    let default_time = start.elapsed();

    let start = Instant::now();
    let _result = convert_with_config(&test_text, &char_config)?;
    let char_time = start.elapsed();

    println!(
        "    大文本 - 默认: {:?}, 单字: {:?}",
        default_time, char_time
    );

    // 2. 预热优化
    println!("  预热优化:");
    let warmup_text = "预热";
    let _warmup = convert(warmup_text); // 预热数据加载

    let start = Instant::now();
    let _result = convert(&test_text);
    let warmed_time = start.elapsed();
    println!("    预热后转换时间: {:?}", warmed_time);

    Ok(())
}

fn batch_processing() -> Result<(), Box<dyn std::error::Error>> {
    let texts = [
        "北京大学",
        "清华大学",
        "中国科学院",
        "中华人民共和国",
        "人工智能技术",
    ];

    // 批量转换
    println!("  批量转换:");
    let config = PinyinConfig::new();

    let start = Instant::now();
    let results: Result<Vec<_>, _> = texts
        .iter()
        .map(|text| convert_with_config(text, &config))
        .collect();
    let batch_time = start.elapsed();

    match &results {
        Ok(results) => {
            for (text, result) in texts.iter().zip(results.iter()) {
                println!("    {} -> {}", text, result.join(" "));
            }
            println!("  批量处理时间: {:?}", batch_time);
        }
        Err(e) => println!("  批量处理失败: {}", e),
    }

    // 统计信息
    println!("  统计信息:");
    let total_chars: usize = texts.iter().map(|t| t.chars().count()).sum();
    let total_results: usize = results.as_ref().unwrap().iter().map(|r| r.len()).sum();
    println!("    总字符数: {}", total_chars);
    println!("    总拼音数: {}", total_results);
    println!(
        "    平均转换率: {:.2} 拼音/字符",
        total_results as f64 / total_chars as f64
    );

    Ok(())
}

fn custom_processors() {
    // 自定义拼音处理器
    struct PinyinProcessor {
        _config: PinyinConfig,
    }

    impl PinyinProcessor {
        fn new() -> Self {
            Self {
                _config: PinyinConfig::new(),
            }
        }

        fn to_initials(&self, text: &str) -> String {
            convert(text)
                .iter()
                .filter_map(|pinyin| {
                    pinyin
                        .chars()
                        .find(|c| c.is_ascii_alphabetic())
                        .map(|c| c.to_uppercase().to_string())
                })
                .collect::<Vec<_>>()
                .join("")
        }

        fn to_toneless(&self, text: &str) -> Vec<String> {
            convert(text)
                .iter()
                .map(|pinyin| self.remove_tones(pinyin))
                .collect()
        }

        fn to_numbered_tones(&self, text: &str) -> Vec<String> {
            convert(text)
                .iter()
                .map(|pinyin| self.convert_to_numbered(pinyin))
                .collect()
        }

        fn remove_tones(&self, pinyin: &str) -> String {
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
        }

        fn convert_to_numbered(&self, pinyin: &str) -> String {
            let tone_map = HashMap::from([
                ("ā", "a1"),
                ("á", "a2"),
                ("ǎ", "a3"),
                ("à", "a4"),
                ("ē", "e1"),
                ("é", "e2"),
                ("ě", "e3"),
                ("è", "e4"),
                ("ī", "i1"),
                ("í", "i2"),
                ("ǐ", "i3"),
                ("ì", "i4"),
                ("ō", "o1"),
                ("ó", "o2"),
                ("ǒ", "o3"),
                ("ò", "o4"),
                ("ū", "u1"),
                ("ú", "u2"),
                ("ǔ", "u3"),
                ("ù", "u4"),
                ("ǖ", "v1"),
                ("ǘ", "v2"),
                ("ǚ", "v3"),
                ("ǜ", "v4"),
            ]);

            let mut result = pinyin.to_string();
            for (tone_char, numbered) in tone_map {
                result = result.replace(tone_char, numbered);
            }
            result
        }
    }

    let processor = PinyinProcessor::new();
    let test_text = "中华人民共和国";

    println!("  原文: {}", test_text);
    println!("  标准拼音: {}", convert(test_text).join(" "));
    println!("  首字母: {}", processor.to_initials(test_text));
    println!("  无声调: {}", processor.to_toneless(test_text).join(" "));
    println!(
        "  数字声调: {}",
        processor.to_numbered_tones(test_text).join(" ")
    );
}

fn multithreaded_usage() {
    use std::sync::Arc;
    use std::thread;

    let texts = Arc::new(vec![
        "北京", "上海", "广州", "深圳", "杭州", "南京", "武汉", "成都",
    ]);

    println!("  多线程转换:");
    let start = Instant::now();

    let handles: Vec<_> = (0..4)
        .map(|thread_id| {
            let texts = Arc::clone(&texts);
            thread::spawn(move || {
                let start_idx = thread_id * 2;
                let end_idx = std::cmp::min(start_idx + 2, texts.len());

                let mut results = Vec::new();
                for i in start_idx..end_idx {
                    let text = &texts[i];
                    let pinyin = convert(text);
                    results.push(((*text).to_string(), pinyin.join(" ")));
                }
                (thread_id, results)
            })
        })
        .collect();

    let mut all_results = Vec::new();
    for handle in handles {
        let (thread_id, results) = handle.join().unwrap();
        println!("    线程 {}: 处理了 {} 个文本", thread_id, results.len());
        all_results.extend(results);
    }

    let total_time = start.elapsed();
    println!("  总处理时间: {:?}", total_time);

    for (text, pinyin) in all_results {
        println!("    {} -> {}", text, pinyin);
    }
}

fn memory_optimization() {
    println!("  内存使用优化:");

    // 1. 避免不必要的字符串分配
    fn efficient_join(parts: &[String], separator: &str) -> String {
        if parts.is_empty() {
            return String::new();
        }

        let total_len =
            parts.iter().map(|s| s.len()).sum::<usize>() + separator.len() * (parts.len() - 1);
        let mut result = String::with_capacity(total_len);

        for (i, part) in parts.iter().enumerate() {
            if i > 0 {
                result.push_str(separator);
            }
            result.push_str(part);
        }
        result
    }

    let text = "中华人民共和国万岁";
    let pinyin_parts = convert(text);

    // 高效拼接
    let joined = efficient_join(&pinyin_parts, " ");
    println!("    高效拼接: {}", joined);

    // 2. 复用配置对象
    let config = PinyinConfig::new();
    let texts = vec!["中国", "美国", "英国", "法国", "德国"];

    println!("    复用配置转换:");
    for text in texts {
        if let Ok(result) = convert_with_config(text, &config) {
            println!("      {} -> {}", text, result.join(" "));
        }
    }

    // 3. 预分配容器
    fn convert_batch_optimized(texts: &[&str]) -> Vec<Vec<String>> {
        let mut results = Vec::with_capacity(texts.len());
        for text in texts {
            results.push(convert(text));
        }
        results
    }

    let batch_texts = vec!["北京", "上海", "广州"];
    let batch_results = convert_batch_optimized(&batch_texts);
    println!("    批量转换结果数: {}", batch_results.len());
}
