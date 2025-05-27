use pinyin::convert;

/// 基本用法示例 - 参考 PHP 版本的功能演示
fn main() {
    println!("=== Pinyin.rs 基本用法示例 ===");
    println!("参考 PHP 版本 overtrue/pinyin 的功能");
    println!();

    // 1. 基本拼音转换
    println!("1. 基本拼音转换");
    println!("----------------------------------------");

    let text = "你好，世界！";
    println!("原文: {}", text);

    // 当前版本只支持带音调的拼音
    let result = convert(text);
    println!("拼音: {}", result.join(" "));
    println!();

    // 2. 句子转换
    println!("2. 句子转换");
    println!("----------------------------------------");

    let sentences = vec![
        "带着希望去旅行，比到达终点更美好",
        "春眠不觉晓，处处闻啼鸟",
        "床前明月光，疑是地上霜",
    ];

    for sentence in sentences {
        println!("原文: {}", sentence);
        let pinyin = convert(sentence);
        println!("拼音: {}", pinyin.join(" "));
        println!();
    }

    // 3. 多音字处理
    println!("3. 多音字处理");
    println!("----------------------------------------");

    let polyphones = vec![
        "重庆",
        "银行",
        "单独",
        "中国",
        "长城",
    ];

    for word in polyphones {
        println!("词语: {}", word);
        let pinyin = convert(word);
        println!("拼音: {}", pinyin.join(" "));
        println!();
    }

    // 4. 混合内容处理
    println!("4. 混合内容处理");
    println!("----------------------------------------");

    let mixed_texts = vec![
        "Hello 世界 123",
        "iPhone15 很棒！",
        "2024年春节快乐🎉",
        "AI人工智能 vs 传统算法",
    ];

    for text in mixed_texts {
        println!("原文: {}", text);
        let pinyin = convert(text);
        println!("拼音: {}", pinyin.join(" "));
        println!();
    }

    // 5. 不同格式输出
    println!("5. 不同格式输出");
    println!("----------------------------------------");

    let demo_text = "中华人民共和国";
    println!("原文: {}", demo_text);

    // 空格分隔
    let pinyin = convert(demo_text);
    println!("空格分隔: {}", pinyin.join(" "));

    // 连字符分隔
    println!("连字符分隔: {}", pinyin.join("-"));

    // 下划线分隔
    println!("下划线分隔: {}", pinyin.join("_"));

    // 无分隔符
    println!("无分隔符: {}", pinyin.join(""));

    // JSON 格式
    println!("JSON 格式: {:?}", pinyin);
    println!();

    // 6. 性能演示
    println!("6. 性能演示");
    println!("----------------------------------------");

    let long_text = "这是一个用来测试性能的长文本，包含了很多中文字符。".repeat(10);
    println!("长文本字符数: {}", long_text.chars().count());

    let start = std::time::Instant::now();
    let result = convert(&long_text);
    let duration = start.elapsed();

    println!("转换耗时: {:?}", duration);
    println!("输出拼音数: {}", result.len());
    println!("处理速度: {:.2} 字符/毫秒",
             long_text.chars().count() as f64 / duration.as_millis() as f64);
    println!();

    // 7. 特殊字符处理
    println!("7. 特殊字符处理");
    println!("----------------------------------------");

    let special_cases = vec![
        "标点符号：！@#￥%……&*（）",
        "数字123456789",
        "英文English",
        "emoji😀😃😄😁",
        "制表符\t换行符\n",
        "全角　空格",
    ];

    for text in special_cases {
        println!("输入: {:?}", text);
        let pinyin = convert(text);
        println!("输出: {}", pinyin.join(" "));
        println!();
    }

    println!("=== 示例演示完成 ===");
    println!("更多功能请参考文档和测试用例");
}
