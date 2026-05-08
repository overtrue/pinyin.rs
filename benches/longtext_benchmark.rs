use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use pinyin::{Converter, Pinyin, PinyinConfig, convert, convert_with_config};
use std::fs;
use std::hint::black_box;

fn bench_longtext_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("longtext_performance");
    group.sample_size(10);

    // 读取长文本文件
    let longtext = fs::read_to_string("benches/longtext.txt").expect("Failed to read longtext.txt");

    let char_count = longtext.chars().count();
    let byte_count = longtext.len();

    println!("=== 长文本性能测试 ===");
    println!("文本长度: {} 字符, {} 字节", char_count, byte_count);

    // 设置吞吐量指标
    group.throughput(Throughput::Elements(char_count as u64));

    // 基本转换性能
    group.bench_function("convert_basic", |b| {
        b.iter(|| convert(black_box(&longtext)))
    });

    // 安全转换性能
    let safe_config = PinyinConfig::new().with_max_length(25000);
    group.bench_function("convert_safe", |b| {
        b.iter(|| convert_with_config(black_box(&longtext), &safe_config).unwrap())
    });

    group.finish();
}

fn bench_longtext_configs(c: &mut Criterion) {
    let mut group = c.benchmark_group("longtext_configs");
    group.sample_size(10);

    let longtext = fs::read_to_string("benches/longtext.txt").expect("Failed to read longtext.txt");

    let char_count = longtext.chars().count();
    group.throughput(Throughput::Elements(char_count as u64));

    // 测试不同配置的性能影响
    let configs = vec![
        (PinyinConfig::default().with_max_length(25000), "default"),
        (
            PinyinConfig::new()
                .with_long_words(true)
                .with_max_length(25000),
            "long_words_enabled",
        ),
        (
            PinyinConfig::new()
                .with_long_words(false)
                .with_max_length(25000),
            "long_words_disabled",
        ),
        (
            PinyinConfig::new()
                .with_polyphone(true)
                .with_max_length(25000),
            "polyphone_enabled",
        ),
        (
            PinyinConfig::new()
                .with_polyphone(false)
                .with_max_length(25000),
            "polyphone_disabled",
        ),
        (
            PinyinConfig::new()
                .with_long_words(false)
                .with_polyphone(false)
                .with_max_length(25000),
            "minimal_features",
        ),
    ];

    for (config, name) in configs {
        group.bench_with_input(BenchmarkId::new("config", name), &config, |b, config| {
            b.iter(|| convert_with_config(black_box(&longtext), black_box(config)).unwrap())
        });
    }

    group.finish();
}

fn bench_longtext_apis(c: &mut Criterion) {
    let mut group = c.benchmark_group("longtext_apis");
    group.sample_size(10);

    let longtext = fs::read_to_string("benches/longtext.txt").expect("Failed to read longtext.txt");

    let char_count = longtext.chars().count();
    group.throughput(Throughput::Elements(char_count as u64));

    // 测试不同API的性能
    group.bench_function("convert_function", |b| {
        b.iter(|| convert(black_box(&longtext)))
    });

    group.bench_function("pinyin_sentence", |b| {
        b.iter(|| Pinyin::sentence(black_box(&longtext)))
    });

    group.bench_function("converter_basic", |b| {
        b.iter(|| Converter::new(black_box(&longtext)).convert())
    });

    group.bench_function("converter_with_options", |b| {
        b.iter(|| {
            Converter::new(black_box(&longtext))
                .without_tone()
                .flatten()
                .only_hans()
                .convert()
        })
    });

    group.bench_function("converter_to_string", |b| {
        b.iter(|| Converter::new(black_box(&longtext)).to_string())
    });

    group.bench_function("converter_to_permalink", |b| {
        b.iter(|| Converter::new(black_box(&longtext)).to_permalink())
    });

    group.finish();
}

fn bench_longtext_chunked(c: &mut Criterion) {
    let mut group = c.benchmark_group("longtext_chunked");
    group.sample_size(10);

    let longtext = fs::read_to_string("benches/longtext.txt").expect("Failed to read longtext.txt");

    let char_count = longtext.chars().count();
    group.throughput(Throughput::Elements(char_count as u64));

    // 测试不同的分块策略
    let chunk_sizes = vec![100, 500, 1000, 2000, 5000];

    for chunk_size in chunk_sizes {
        let chunks: Vec<String> = longtext
            .chars()
            .collect::<Vec<_>>()
            .chunks(chunk_size)
            .map(|chunk| chunk.iter().collect())
            .collect();

        group.bench_with_input(
            BenchmarkId::new("chunked", chunk_size),
            &chunks,
            |b, chunks| {
                b.iter(|| {
                    let mut results = Vec::new();
                    for chunk in chunks {
                        results.extend(convert(black_box(chunk)));
                    }
                    results
                })
            },
        );
    }

    // 对比整体处理 vs 分块处理
    group.bench_function("whole_text", |b| b.iter(|| convert(black_box(&longtext))));

    let optimal_chunks: Vec<String> = longtext
        .chars()
        .collect::<Vec<_>>()
        .chunks(1000)
        .map(|chunk| chunk.iter().collect())
        .collect();

    group.bench_function("optimal_chunked", |b| {
        b.iter(|| {
            let mut results = Vec::new();
            for chunk in &optimal_chunks {
                results.extend(convert(black_box(chunk)));
            }
            results
        })
    });

    group.finish();
}

fn bench_longtext_memory(c: &mut Criterion) {
    let mut group = c.benchmark_group("longtext_memory");
    group.sample_size(10); // 减少样本数量，因为内存测试比较耗时

    let longtext = fs::read_to_string("benches/longtext.txt").expect("Failed to read longtext.txt");

    // 内存稳定性测试 - 重复转换
    group.bench_function("repeated_conversion", |b| {
        b.iter(|| {
            for _ in 0..10 {
                let _result = convert(black_box(&longtext));
            }
        })
    });

    // 内存累积测试 - 保存结果
    group.bench_function("accumulate_results", |b| {
        b.iter(|| {
            let mut all_results = Vec::new();
            for _ in 0..5 {
                let result = convert(black_box(&longtext));
                all_results.push(result);
            }
            all_results
        })
    });

    // 大量小文本 vs 少量大文本
    let small_texts: Vec<String> = longtext.lines().map(|line| line.to_string()).collect();

    group.bench_function("many_small_texts", |b| {
        b.iter(|| {
            for text in &small_texts {
                convert(black_box(text));
            }
        })
    });

    group.bench_function("single_large_text", |b| {
        b.iter(|| convert(black_box(&longtext)))
    });

    group.finish();
}

fn bench_longtext_concurrent(c: &mut Criterion) {
    let mut group = c.benchmark_group("longtext_concurrent");
    group.sample_size(10);

    let longtext = fs::read_to_string("benches/longtext.txt").expect("Failed to read longtext.txt");

    use std::sync::Arc;
    use std::thread;

    let shared_text = Arc::new(longtext);

    // 单线程基准
    group.bench_function("single_thread", |b| {
        let text = Arc::clone(&shared_text);
        b.iter(|| convert(black_box(&text)))
    });

    // 多线程并发
    let thread_counts = vec![2, 4, 8];

    for thread_count in thread_counts {
        group.bench_with_input(
            BenchmarkId::new("concurrent", thread_count),
            &thread_count,
            |b, &thread_count| {
                b.iter(|| {
                    let handles: Vec<_> = (0..thread_count)
                        .map(|_| {
                            let text = Arc::clone(&shared_text);
                            thread::spawn(move || convert(&text))
                        })
                        .collect();

                    for handle in handles {
                        handle.join().unwrap();
                    }
                })
            },
        );
    }

    group.finish();
}

fn bench_longtext_analysis(c: &mut Criterion) {
    let mut group = c.benchmark_group("longtext_analysis");
    group.sample_size(10);

    let longtext = fs::read_to_string("benches/longtext.txt").expect("Failed to read longtext.txt");

    // 分析文本特征对性能的影响
    let lines: Vec<&str> = longtext.lines().collect();

    // 短行 vs 长行
    let short_lines: String = lines
        .iter()
        .filter(|line| line.len() < 100)
        .cloned()
        .collect::<Vec<_>>()
        .join("\n");

    let long_lines: String = lines
        .iter()
        .filter(|line| line.len() >= 100)
        .cloned()
        .collect::<Vec<_>>()
        .join("\n");

    if !short_lines.is_empty() {
        group.bench_function("short_lines", |b| {
            b.iter(|| convert(black_box(&short_lines)))
        });
    }

    if !long_lines.is_empty() {
        group.bench_function("long_lines", |b| b.iter(|| convert(black_box(&long_lines))));
    }

    // 密集汉字 vs 稀疏汉字
    let dense_chinese: String = longtext
        .chars()
        .filter(|c| {
            matches!(*c as u32,
                0x4E00..=0x9FFF |  // CJK Unified Ideographs
                0x3400..=0x4DBF |  // CJK Extension A
                0x20000..=0x2A6DF | // CJK Extension B
                0x2A700..=0x2B73F | // CJK Extension C
                0x2B740..=0x2B81F | // CJK Extension D
                0x2B820..=0x2CEAF | // CJK Extension E
                0xF900..=0xFAFF |   // CJK Compatibility Ideographs
                0x2F800..=0x2FA1F   // CJK Compatibility Supplement
            )
        })
        .take(1000) // 限制长度以便测试
        .collect();

    if !dense_chinese.is_empty() {
        group.bench_function("dense_chinese", |b| {
            b.iter(|| convert(black_box(&dense_chinese)))
        });
    }

    group.finish();
}

criterion_group!(
    longtext_benches,
    bench_longtext_performance,
    bench_longtext_configs,
    bench_longtext_apis,
    bench_longtext_chunked,
    bench_longtext_memory,
    bench_longtext_concurrent,
    bench_longtext_analysis
);

criterion_main!(longtext_benches);
