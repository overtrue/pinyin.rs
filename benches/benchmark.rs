use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use pinyin::{Converter, Pinyin, PinyinConfig, convert, convert_safe, convert_with_config};
use std::fs;
use std::hint::black_box;

fn bench_convert_single_char(c: &mut Criterion) {
    let mut group = c.benchmark_group("convert_single_char");

    let test_chars = vec!["中", "国", "人", "民", "你", "好", "世", "界"];

    for char in test_chars {
        group.bench_with_input(BenchmarkId::new("convert", char), char, |b, char| {
            b.iter(|| convert(black_box(char)))
        });
    }

    group.finish();
}

fn bench_convert_words(c: &mut Criterion) {
    let mut group = c.benchmark_group("convert_words");

    let test_words = vec![
        "中国",
        "中华人民共和国",
        "北京大学",
        "清华大学",
        "你好世界",
        "带着希望去旅行",
    ];

    for word in test_words {
        group.bench_with_input(BenchmarkId::new("convert", word), word, |b, word| {
            b.iter(|| convert(black_box(word)))
        });

        group.bench_with_input(BenchmarkId::new("convert_safe", word), word, |b, word| {
            b.iter(|| convert_safe(black_box(word)).unwrap())
        });
    }

    group.finish();
}

fn bench_convert_sentences(c: &mut Criterion) {
    let mut group = c.benchmark_group("convert_sentences");

    let test_sentences = vec![
        "中国人民喜欢在中国吃饭，中国人的口味，中国饭好吃",
        "春眠不觉晓，处处闻啼鸟。夜来风雨声，花落知多少。",
        "床前明月光，疑是地上霜。举头望明月，低头思故乡。",
        "带着希望去旅行，比到达终点更美好。人生就像一场旅行，不必在乎目的地。",
    ];

    for sentence in test_sentences {
        group.bench_with_input(
            BenchmarkId::new("convert", sentence),
            sentence,
            |b, sentence| b.iter(|| convert(black_box(sentence))),
        );
    }

    group.finish();
}

fn bench_convert_long_text(c: &mut Criterion) {
    let mut group = c.benchmark_group("convert_long_text");

    let base_text = "中华人民共和国万岁，世界人民大团结万岁！";
    let test_cases = vec![
        (base_text.repeat(10), "small"),
        (base_text.repeat(50), "medium"),
        (base_text.repeat(100), "large"),
    ];

    for (text, size) in test_cases {
        group.bench_with_input(BenchmarkId::new("convert", size), &text, |b, text| {
            b.iter(|| convert(black_box(text)))
        });
    }

    group.finish();
}

fn bench_convert_with_configs(c: &mut Criterion) {
    let mut group = c.benchmark_group("convert_with_configs");

    let text = "中华人民共和国万岁";

    let configs = vec![
        (PinyinConfig::default(), "default"),
        (PinyinConfig::new().with_long_words(false), "no_long_words"),
        (PinyinConfig::new().with_polyphone(true), "polyphone"),
        (PinyinConfig::new().with_polyphone(false), "no_polyphone"),
    ];

    for (config, name) in configs {
        group.bench_with_input(
            BenchmarkId::new("convert_with_config", name),
            &config,
            |b, config| b.iter(|| convert_with_config(black_box(text), black_box(config)).unwrap()),
        );
    }

    group.finish();
}

fn bench_pinyin_static_methods(c: &mut Criterion) {
    let mut group = c.benchmark_group("pinyin_static_methods");

    let text = "带着希望去旅行";

    group.bench_function("sentence", |b| b.iter(|| Pinyin::sentence(black_box(text))));

    group.bench_function("permalink", |b| {
        b.iter(|| Pinyin::permalink(black_box(text)))
    });

    group.bench_function("abbr", |b| b.iter(|| Pinyin::abbr(black_box(text))));

    let name = "单某某";
    group.bench_function("name", |b| b.iter(|| Pinyin::name(black_box(name))));

    group.bench_function("name_abbr", |b| {
        b.iter(|| Pinyin::name_abbr(black_box(name)))
    });

    group.bench_function("passport_name", |b| {
        b.iter(|| Pinyin::passport_name(black_box("吕小布")))
    });

    group.finish();
}

fn bench_converter_api(c: &mut Criterion) {
    let mut group = c.benchmark_group("converter_api");

    let text = "你好世界";

    group.bench_function("basic_convert", |b| {
        b.iter(|| Converter::new(black_box(text)).convert())
    });

    group.bench_function("with_options", |b| {
        b.iter(|| {
            Converter::new(black_box(text))
                .without_tone()
                .flatten()
                .convert()
        })
    });

    group.bench_function("to_string", |b| {
        b.iter(|| Converter::new(black_box(text)).to_string())
    });

    group.bench_function("to_permalink", |b| {
        b.iter(|| Converter::new(black_box(text)).to_permalink())
    });

    group.finish();
}

fn bench_mixed_content(c: &mut Criterion) {
    let mut group = c.benchmark_group("mixed_content");

    let test_cases = vec![
        ("Hello世界123", "mixed_simple"),
        ("你好2024年春节快乐！", "mixed_with_numbers"),
        ("CGV电影院@北京朝阳区", "mixed_complex"),
        ("🌟中国🇨🇳加油💪", "mixed_with_emoji"),
    ];

    for (text, name) in test_cases {
        group.bench_with_input(BenchmarkId::new("convert", name), text, |b, text| {
            b.iter(|| convert(black_box(text)))
        });
    }

    group.finish();
}

fn bench_special_cases(c: &mut Criterion) {
    let mut group = c.benchmark_group("special_cases");

    // 测试多音字
    group.bench_function("polyphone", |b| {
        b.iter(|| convert(black_box("银行行走重庆重量")))
    });

    // 测试姓氏
    group.bench_function("surnames", |b| {
        b.iter(|| convert(black_box("单曾区仇朴盖")))
    });

    // 测试生僻字
    group.bench_function("rare_chars", |b| b.iter(|| convert(black_box("䴙䴘龘𠮷"))));

    // 测试纯英文
    group.bench_function("pure_english", |b| {
        b.iter(|| convert(black_box("Hello World")))
    });

    // 测试纯数字
    group.bench_function("pure_numbers", |b| {
        b.iter(|| convert(black_box("1234567890")))
    });

    // 测试标点符号
    group.bench_function("punctuation", |b| {
        b.iter(|| convert(black_box("，。！？：；")))
    });

    group.finish();
}

fn bench_memory_intensive(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_intensive");
    group.sample_size(10); // 减少样本数量，因为这些测试比较耗时

    // 测试大量重复字符
    let large_text = "中".repeat(1000);
    group.bench_function("large_repeated_char", |b| {
        b.iter(|| convert(black_box(&large_text)))
    });

    // 测试大量重复词汇
    let large_words = "中华人民共和国".repeat(100);
    group.bench_function("large_repeated_words", |b| {
        b.iter(|| convert(black_box(&large_words)))
    });

    // 测试大量混合内容
    let large_mixed = "中国China123！".repeat(200);
    group.bench_function("large_mixed_content", |b| {
        b.iter(|| convert(black_box(&large_mixed)))
    });

    group.finish();
}

fn bench_concurrent_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_access");

    use std::sync::Arc;
    use std::thread;

    let text = Arc::new("中华人民共和国".to_string());

    group.bench_function("single_thread", |b| {
        let text = Arc::clone(&text);
        b.iter(|| convert(black_box(&text)))
    });

    group.bench_function("multi_thread", |b| {
        b.iter(|| {
            let handles: Vec<_> = (0..4)
                .map(|_| {
                    let text = Arc::clone(&text);
                    thread::spawn(move || convert(&text))
                })
                .collect();

            for handle in handles {
                handle.join().unwrap();
            }
        })
    });

    group.finish();
}

fn bench_longtext_file(c: &mut Criterion) {
    let mut group = c.benchmark_group("longtext_file");
    group.sample_size(10); // 减少样本数量，因为长文本测试比较耗时

    // 读取长文本文件
    let longtext = fs::read_to_string("benches/longtext.txt").expect("Failed to read longtext.txt");

    println!(
        "Long text length: {} chars, {} bytes",
        longtext.chars().count(),
        longtext.len()
    );

    // 基本转换测试
    group.bench_function("convert_full_text", |b| {
        b.iter(|| convert(black_box(&longtext)))
    });

    // 安全转换测试
    let longtext_config = PinyinConfig::new().with_max_length(longtext.len() + 1);
    group.bench_function("convert_safe_full_text", |b| {
        b.iter(|| convert_with_config(black_box(&longtext), &longtext_config).unwrap())
    });

    // 不同配置的性能对比
    let configs = vec![
        (PinyinConfig::new().with_max_length(20000), "default"),
        (
            PinyinConfig::new()
                .with_long_words(false)
                .with_max_length(20000),
            "no_long_words",
        ),
        (
            PinyinConfig::new()
                .with_polyphone(true)
                .with_max_length(20000),
            "polyphone",
        ),
        (
            PinyinConfig::new()
                .with_polyphone(false)
                .with_max_length(20000),
            "no_polyphone",
        ),
    ];

    for (config, name) in configs {
        group.bench_with_input(
            BenchmarkId::new("convert_with_config", name),
            &config,
            |b, config| {
                b.iter(|| convert_with_config(black_box(&longtext), black_box(config)).unwrap())
            },
        );
    }

    // 测试不同的API
    group.bench_function("pinyin_sentence", |b| {
        b.iter(|| Pinyin::sentence(black_box(&longtext)))
    });

    group.bench_function("converter_api", |b| {
        b.iter(|| Converter::new(black_box(&longtext)).convert())
    });

    group.bench_function("converter_with_options", |b| {
        b.iter(|| {
            Converter::new(black_box(&longtext))
                .without_tone()
                .flatten()
                .convert()
        })
    });

    // 分块处理测试
    let chunk_size = 1000;
    let longtext_string = longtext.chars().collect::<String>();
    let chunks: Vec<&str> = longtext_string
        .as_bytes()
        .chunks(chunk_size)
        .map(|chunk| std::str::from_utf8(chunk).unwrap_or(""))
        .filter(|s| !s.is_empty())
        .collect();

    group.bench_function("chunked_processing", |b| {
        b.iter(|| {
            for chunk in &chunks {
                convert(black_box(chunk));
            }
        })
    });

    // 内存使用测试（通过重复转换来观察内存稳定性）
    group.bench_function("memory_stability", |b| {
        b.iter(|| {
            for _ in 0..3 {
                let _result = convert(black_box(&longtext));
            }
        })
    });

    group.finish();
}

fn bench_longtext_subsets(c: &mut Criterion) {
    let mut group = c.benchmark_group("longtext_subsets");

    // 读取长文本文件
    let longtext = fs::read_to_string("benches/longtext.txt").expect("Failed to read longtext.txt");

    let lines: Vec<&str> = longtext.lines().collect();

    // 测试不同大小的文本片段
    let test_sizes = vec![
        (1, "single_line"),
        (5, "small_chunk"),
        (10, "medium_chunk"),
        (20, "large_chunk"),
        (lines.len(), "full_text"),
    ];

    for (line_count, size_name) in test_sizes {
        let text = lines
            .iter()
            .take(line_count.min(lines.len()))
            .cloned()
            .collect::<Vec<_>>()
            .join("\n");

        group.bench_with_input(BenchmarkId::new("convert", size_name), &text, |b, text| {
            b.iter(|| convert(black_box(text)))
        });

        let config = PinyinConfig::new().with_max_length(text.len() + 1);
        group.bench_with_input(
            BenchmarkId::new("convert_safe", size_name),
            &text,
            |b, text| b.iter(|| convert_with_config(black_box(text), &config).unwrap()),
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_convert_single_char,
    bench_convert_words,
    bench_convert_sentences,
    bench_convert_long_text,
    bench_convert_with_configs,
    bench_pinyin_static_methods,
    bench_converter_api,
    bench_mixed_content,
    bench_special_cases,
    bench_memory_intensive,
    bench_concurrent_access,
    bench_longtext_file,
    bench_longtext_subsets
);

criterion_main!(benches);
