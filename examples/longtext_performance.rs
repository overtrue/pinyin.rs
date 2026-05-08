use pinyin::{Converter, Pinyin, PinyinConfig, convert, convert_with_config};
use std::fs;
use std::time::Instant;

fn main() {
    println!("=== Pinyin.rs 长文本性能测试 ===\n");

    // 读取长文本文件
    let longtext = fs::read_to_string("benches/longtext.txt").expect("Failed to read longtext.txt");

    let char_count = longtext.chars().count();
    let byte_count = longtext.len();

    println!("文本信息:");
    println!("  字符数: {}", char_count);
    println!("  字节数: {}", byte_count);
    println!("  行数: {}", longtext.lines().count());
    println!();

    // 测试基本转换性能
    println!("1. 基本转换性能测试");
    let start = Instant::now();
    let result = convert(&longtext);
    let duration = start.elapsed();

    println!("  转换耗时: {:?}", duration);
    println!("  转换结果长度: {}", result.len());
    println!(
        "  字符/秒: {:.0}",
        char_count as f64 / duration.as_secs_f64()
    );
    println!(
        "  字节/秒: {:.0}",
        byte_count as f64 / duration.as_secs_f64()
    );
    println!();

    // 测试配置转换性能
    println!("2. 配置转换性能测试");
    let config = PinyinConfig::new().with_max_length(25000);

    let start = Instant::now();
    let result_config = convert_with_config(&longtext, &config).unwrap();
    let duration_config = start.elapsed();

    println!("  配置转换耗时: {:?}", duration_config);
    println!("  转换结果长度: {}", result_config.len());
    println!(
        "  字符/秒: {:.0}",
        char_count as f64 / duration_config.as_secs_f64()
    );
    println!();

    // 测试词组匹配 vs 单字转换
    println!("3. 词组匹配 vs 单字转换对比");

    let long_words_config = PinyinConfig::new()
        .with_long_words(true)
        .with_max_length(25000);

    let char_config = PinyinConfig::new()
        .with_long_words(false)
        .with_max_length(25000);

    let start = Instant::now();
    let _long_words_result = convert_with_config(&longtext, &long_words_config).unwrap();
    let long_words_duration = start.elapsed();

    let start = Instant::now();
    let _char_result = convert_with_config(&longtext, &char_config).unwrap();
    let char_duration = start.elapsed();

    println!("  词组匹配耗时: {:?}", long_words_duration);
    println!("  单字转换耗时: {:?}", char_duration);
    println!(
        "  耗时比例: {:.2}x",
        char_duration.as_secs_f64() / long_words_duration.as_secs_f64()
    );
    println!();

    // 测试不同API的性能
    println!("4. 不同API性能对比");

    // Pinyin::sentence
    let start = Instant::now();
    let _sentence_result = Pinyin::sentence(&longtext);
    let sentence_duration = start.elapsed();

    // Converter API
    let start = Instant::now();
    let _converter_result = Converter::new(&longtext).convert();
    let converter_duration = start.elapsed();

    // Converter with options
    let start = Instant::now();
    let _converter_opt_result = Converter::new(&longtext).without_tone().flatten().convert();
    let converter_opt_duration = start.elapsed();

    println!("  Pinyin::sentence: {:?}", sentence_duration);
    println!("  Converter::convert: {:?}", converter_duration);
    println!("  Converter with options: {:?}", converter_opt_duration);
    println!();

    // 测试分块处理性能
    println!("5. 分块处理性能测试");

    let chunk_sizes = vec![500, 1000, 2000, 5000];

    for chunk_size in chunk_sizes {
        let chunks: Vec<String> = longtext
            .chars()
            .collect::<Vec<_>>()
            .chunks(chunk_size)
            .map(|chunk| chunk.iter().collect())
            .collect();

        let start = Instant::now();
        let mut chunked_results = Vec::new();
        for chunk in &chunks {
            chunked_results.extend(convert(chunk));
        }
        let chunked_duration = start.elapsed();

        println!(
            "  分块大小 {}: {:?} ({} 块)",
            chunk_size,
            chunked_duration,
            chunks.len()
        );
    }

    // 对比整体处理
    let start = Instant::now();
    let _whole_result = convert(&longtext);
    let whole_duration = start.elapsed();

    println!("  整体处理: {:?}", whole_duration);
    println!();

    // 内存使用测试
    println!("6. 内存稳定性测试");

    let start = Instant::now();
    for i in 0..5 {
        let _result = convert(&longtext);
        println!("  第 {} 次转换完成", i + 1);
    }
    let repeated_duration = start.elapsed();

    println!("  5次重复转换总耗时: {:?}", repeated_duration);
    println!("  平均每次耗时: {:?}", repeated_duration / 5);
    println!();

    // 文本特征分析
    println!("7. 文本特征分析");

    let chinese_chars: String = longtext.chars().filter(|c| is_chinese_char(*c)).collect();

    let chinese_ratio = chinese_chars.len() as f64 / char_count as f64;

    println!("  中文字符数: {}", chinese_chars.len());
    println!("  中文字符比例: {:.2}%", chinese_ratio * 100.0);

    // 测试纯中文性能
    if chinese_chars.len() > 1000 {
        let chinese_sample = chinese_chars.chars().take(1000).collect::<String>();

        let start = Instant::now();
        let _chinese_result = convert(&chinese_sample);
        let chinese_duration = start.elapsed();

        println!("  1000个纯中文字符转换耗时: {:?}", chinese_duration);
        println!(
            "  纯中文字符/秒: {:.0}",
            1000.0 / chinese_duration.as_secs_f64()
        );
    }

    println!("\n=== 性能测试完成 ===");
}

/// 判断是否为中文字符
fn is_chinese_char(ch: char) -> bool {
    matches!(ch as u32,
        0x4E00..=0x9FFF |  // CJK Unified Ideographs
        0x3400..=0x4DBF |  // CJK Extension A
        0x20000..=0x2A6DF | // CJK Extension B
        0x2A700..=0x2B73F | // CJK Extension C
        0x2B740..=0x2B81F | // CJK Extension D
        0x2B820..=0x2CEAF | // CJK Extension E
        0xF900..=0xFAFF |   // CJK Compatibility Ideographs
        0x2F800..=0x2FA1F   // CJK Compatibility Supplement
    )
}
