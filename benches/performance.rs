use criterion::{Criterion, criterion_group, criterion_main};
use pinyin::{PinyinConfig, convert, convert_with_config, match_word_pinyin};
use std::hint::black_box;

fn bench_convert_short_text(c: &mut Criterion) {
    let text = "中国人民";
    c.bench_function("convert_short_text", |b| {
        b.iter(|| convert(black_box(text)))
    });
}

fn bench_convert_medium_text(c: &mut Criterion) {
    let text = "中国人民喜欢在中国吃饭，中国人的口味，中国饭好吃";
    c.bench_function("convert_medium_text", |b| {
        b.iter(|| convert(black_box(text)))
    });
}

fn bench_convert_long_text(c: &mut Criterion) {
    let text = "中国人民喜欢在中国吃饭，中国人的口味，中国饭好吃".repeat(100);
    c.bench_function("convert_long_text", |b| {
        b.iter(|| convert(black_box(&text)))
    });
}

fn bench_convert_with_config_default(c: &mut Criterion) {
    let text = "中国人民喜欢在中国吃饭，中国人的口味，中国饭好吃".repeat(50);
    let config = PinyinConfig::new();

    c.bench_function("convert_with_config_default", |b| {
        b.iter(|| convert_with_config(black_box(&text), black_box(&config)).unwrap())
    });
}

fn bench_convert_with_config_no_long_words(c: &mut Criterion) {
    let text = "中国人民喜欢在中国吃饭，中国人的口味，中国饭好吃".repeat(50);
    let config = PinyinConfig::new().with_long_words(false);

    c.bench_function("convert_with_config_no_long_words", |b| {
        b.iter(|| convert_with_config(black_box(&text), black_box(&config)).unwrap())
    });
}

fn bench_match_word_pinyin(c: &mut Criterion) {
    let text = "中国人民共和国万岁";
    c.bench_function("match_word_pinyin", |b| {
        b.iter(|| match_word_pinyin(black_box(text)))
    });
}

fn bench_mixed_content(c: &mut Criterion) {
    let text = "Hello世界123！中国人民αβγ";
    c.bench_function("convert_mixed_content", |b| {
        b.iter(|| convert(black_box(text)))
    });
}

fn bench_polyphone_text(c: &mut Criterion) {
    let text = "重庆银行行长调调音乐数据";
    c.bench_function("convert_polyphone_text", |b| {
        b.iter(|| convert(black_box(text)))
    });
}

fn bench_surname_text(c: &mut Criterion) {
    let text = "曾国藩单于丹区志华仇大雄";
    c.bench_function("convert_surname_text", |b| {
        b.iter(|| convert(black_box(text)))
    });
}

fn bench_config_creation(c: &mut Criterion) {
    c.bench_function("config_creation", |b| {
        b.iter(|| {
            black_box(
                PinyinConfig::new()
                    .with_max_length(5000)
                    .with_long_words(true)
                    .with_polyphone(true),
            )
        })
    });
}

criterion_group!(
    benches,
    bench_convert_short_text,
    bench_convert_medium_text,
    bench_convert_long_text,
    bench_convert_with_config_default,
    bench_convert_with_config_no_long_words,
    bench_match_word_pinyin,
    bench_mixed_content,
    bench_polyphone_text,
    bench_surname_text,
    bench_config_creation
);

criterion_main!(benches);
