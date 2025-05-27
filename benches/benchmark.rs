use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pinyin::{convert, match_word_pinyin};

fn benchmark_convert_short(c: &mut Criterion) {
    c.bench_function("convert_short", |b| {
        b.iter(|| convert(black_box("中国人民")))
    });
}

fn benchmark_convert_medium(c: &mut Criterion) {
    let text = "中国人民喜欢在中国吃饭，中国人的口味，中国饭好吃";
    c.bench_function("convert_medium", |b| {
        b.iter(|| convert(black_box(text)))
    });
}

fn benchmark_convert_long(c: &mut Criterion) {
    let text = "中国人民喜欢在中国吃饭，中国人的口味，中国饭好吃".repeat(10);
    c.bench_function("convert_long", |b| {
        b.iter(|| convert(black_box(&text)))
    });
}

fn benchmark_match_word_pinyin(c: &mut Criterion) {
    c.bench_function("match_word_pinyin", |b| {
        b.iter(|| match_word_pinyin(black_box("中国人民喜欢在中国吃饭")))
    });
}

criterion_group!(
    benches,
    benchmark_convert_short,
    benchmark_convert_medium,
    benchmark_convert_long,
    benchmark_match_word_pinyin
);
criterion_main!(benches);
