use std::time::Instant;
use pinyin::convert;

/// 基准测试脚本 - 参考 PHP 版本的 overtrue/pinyin
///
/// 这个脚本模拟 PHP 版本的基准测试，测试各种拼音转换功能的性能
fn main() {
    println!("=== Pinyin.rs 基准测试 ===");
    println!("参考 PHP 版本 overtrue/pinyin 的基准测试");
    println!();

    // 测试数据 - 参考 PHP 版本的测试内容
    let test_data = vec![
        // 短文本测试
        "你好",
        "世界",
        "中国",
        "北京",

        // 中等长度文本测试
        "带着希望去旅行，比到达终点更美好",
        "春眠不觉晓，处处闻啼鸟",
        "床前明月光，疑是地上霜",

        // 长文本测试
        "在那遥远的地方，有一位好姑娘，人们走过了她的帐房，都要回头留恋地张望",
        "我爱北京天安门，天安门上太阳升，伟大领袖毛主席，指引我们向前进",

        // 多音字测试
        "重庆重工重点重视重量重复重新重要",
        "银行行走行程行业行为行政行动",
        "单独单位单纯单调单身单薄单据",

        // 混合内容测试
        "Hello 世界 123 你好！",
        "2024年春节快乐🎉",
        "iPhone15 Pro Max 很棒！",

        // 复杂句子测试
        "人工智能技术的发展日新月异，深度学习、机器学习、自然语言处理等领域都取得了重大突破",
        "中华人民共和国成立于1949年10月1日，经过70多年的发展，已经成为世界第二大经济体",
    ];

    // 1. 基本转换性能测试
    println!("1. 基本拼音转换性能测试");
    println!("----------------------------------------");

        for (i, text) in test_data.iter().enumerate() {
        let start = Instant::now();
        let result = convert(text);
        let duration = start.elapsed();

        println!("测试 {}: {} 字符", i + 1, text.chars().count());
        println!("输入: {}", text);
        println!("输出: {}", result.join(" "));
        println!("耗时: {:?}", duration);
        println!();
    }

        // 2. 重复转换性能测试
    println!("2. 重复转换性能测试");
    println!("----------------------------------------");

    let test_text = "带着希望去旅行，比到达终点更美好";

    // 测试多次转换的性能
    let iterations = 100;
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = convert(test_text);
    }
    let total_duration = start.elapsed();
    let avg_duration = total_duration / iterations;

    println!("测试文本: {}", test_text);
    println!("重复次数: {} 次", iterations);
    println!("总耗时: {:?}", total_duration);
    println!("平均耗时: {:?}", avg_duration);
    println!("每秒处理: {:.2} 次", 1_000_000.0 / avg_duration.as_micros() as f64);
    println!();

    // 3. 批量处理性能测试
    println!("3. 批量处理性能测试");
    println!("----------------------------------------");

    let batch_sizes = vec![10, 50, 100, 500, 1000];
    let sample_text = "中华人民共和国";

    for &size in &batch_sizes {
        let start = Instant::now();
        for _ in 0..size {
            let _ = convert(sample_text);
        }
        let duration = start.elapsed();
        let avg_duration = duration / size;

        println!("批量大小: {} 次", size);
        println!("总耗时: {:?}", duration);
        println!("平均耗时: {:?}", avg_duration);
        println!("每秒处理: {:.2} 次", 1_000_000.0 / avg_duration.as_micros() as f64);
        println!();
    }

    // 4. 内存使用测试（模拟）
    println!("4. 不同长度文本处理测试");
    println!("----------------------------------------");

    let super_long_text = "中华人民共和国是世界上历史最悠久的国家之一。".repeat(10);
    let lengths = vec![
        ("短文本", "你好"),
        ("中文本", "春眠不觉晓，处处闻啼鸟。夜来风雨声，花落知多少。"),
        ("长文本", "在那遥远的地方，有一位好姑娘，人们走过了她的帐房，都要回头留恋地张望。她那粉红的笑脸，好像红太阳，她那美丽动人的眼睛，好像晚上明媚的月亮。"),
        ("超长文本", &super_long_text),
    ];

    for (desc, text) in lengths {
        let char_count = text.chars().count();
        let start = Instant::now();
        let result = convert(text);
        let duration = start.elapsed();

        println!("{}: {} 字符", desc, char_count);
        println!("耗时: {:?}", duration);
        println!("每字符耗时: {:?}", duration / char_count as u32);
        println!("输出长度: {} 拼音", result.len());
        println!();
    }

    // 5. 特殊字符处理测试
    println!("5. 特殊字符处理测试");
    println!("----------------------------------------");

    let special_tests = vec![
        "数字123测试",
        "English中文Mixed",
        "标点符号！@#￥%……&*（）",
        "emoji😀😃😄😁测试",
        "换行\n制表\t测试",
        "　　全角空格测试",
    ];

    for text in special_tests {
        let start = Instant::now();
        let result = convert(text);
        let duration = start.elapsed();

        println!("输入: {:?}", text);
        println!("输出: {}", result.join(" "));
        println!("耗时: {:?}", duration);
        println!();
    }

    // 6. 并发性能测试（模拟）
    println!("6. 并发处理模拟测试");
    println!("----------------------------------------");

    let concurrent_text = "并发处理测试文本";
    let thread_counts = vec![1, 2, 4, 8];

    for &thread_count in &thread_counts {
        let start = Instant::now();

        // 模拟并发处理
        let handles: Vec<_> = (0..thread_count).map(|_| {
            std::thread::spawn(move || {
                for _ in 0..100 {
                    let _ = convert(concurrent_text);
                }
            })
        }).collect();

        for handle in handles {
            handle.join().unwrap();
        }

        let duration = start.elapsed();
        println!("线程数: {}", thread_count);
        println!("总耗时: {:?}", duration);
        println!("平均每线程: {:?}", duration / thread_count);
        println!();
    }

    // 7. 性能总结
    println!("7. 性能测试总结");
    println!("----------------------------------------");

    // 综合性能测试
    let comprehensive_test = "这是一个综合性能测试，包含了各种常见的中文字符、标点符号、数字123、英文English，以及一些特殊情况的处理。";
    let iterations = 1000;

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = convert(comprehensive_test);
    }
    let total_duration = start.elapsed();
    let avg_duration = total_duration / iterations;

    println!("综合测试文本: {}", comprehensive_test);
    println!("测试次数: {} 次", iterations);
    println!("总耗时: {:?}", total_duration);
    println!("平均耗时: {:?}", avg_duration);
    println!("每秒处理: {:.2} 次", 1_000_000.0 / avg_duration.as_micros() as f64);
    println!("字符处理速度: {:.2} 字符/秒",
             comprehensive_test.chars().count() as f64 * 1_000_000.0 / avg_duration.as_micros() as f64);

    println!();
    println!("=== 基准测试完成 ===");
    println!("注意：此测试结果仅供参考，实际性能可能因硬件环境而异");
}

#[cfg(test)]
mod benchmark_tests {
    use super::*;

    #[test]
    fn test_basic_conversion_performance() {
        let text = "性能测试";
        let start = Instant::now();
        let result = convert(text);
        let duration = start.elapsed();

        assert!(!result.is_empty());
        assert!(duration.as_millis() < 100); // 应该在100ms内完成
    }

    #[test]
    fn test_batch_processing_performance() {
        let text = "批量处理测试";
        let batch_size = 100;

        let start = Instant::now();
        for _ in 0..batch_size {
            let _ = convert(text);
        }
        let duration = start.elapsed();

        // 平均每次转换应该在10ms内完成
        assert!(duration.as_millis() / batch_size < 10);
    }

    #[test]
    fn test_long_text_performance() {
        let long_text = "这是一个很长的测试文本，用来测试长文本的处理性能。".repeat(50);

        let start = Instant::now();
        let result = convert(&long_text);
        let duration = start.elapsed();

        assert!(!result.is_empty());
        // 长文本处理应该在1秒内完成
        assert!(duration.as_secs() < 1);
    }

    #[test]
    fn test_repeated_conversion_performance() {
        let text = "重复转换测试";

        // 测试重复转换的性能
        let iterations = 10;

        let start = Instant::now();
        for _ in 0..iterations {
            let _ = convert(text);
        }
        let duration = start.elapsed();

        // 平均每次转换应该在50ms内完成
        assert!(duration.as_millis() / iterations < 50);
    }

    #[test]
    fn test_special_characters_performance() {
        let special_text = "特殊字符123ABC！@#测试";

        let start = Instant::now();
        let result = convert(special_text);
        let duration = start.elapsed();

        assert!(!result.is_empty());
        assert!(duration.as_millis() < 100);
    }
}
