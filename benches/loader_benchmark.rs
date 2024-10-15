use criterion::{criterion_group, criterion_main, Criterion};
use pinyin::loader::{WordsLoader, CharsLoader};

fn words_loader_benchmark(c: &mut Criterion) {
    c.bench_function("words_loader_benchmark", |b| {
        b.iter(|| {
            WordsLoader::new();
        })
    });
}

fn chars_loader_benchmark(c: &mut Criterion) {
    c.bench_function("chars_loader_benchmark", |b| {
        b.iter(|| {
            CharsLoader::new();
        })
    });
}

criterion_group!(benches, words_loader_benchmark, chars_loader_benchmark);
criterion_main!(benches);
