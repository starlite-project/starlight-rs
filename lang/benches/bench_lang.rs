use criterion::{criterion_group, criterion_main, Criterion};
use star_lang::I18nMap;

fn langmap_benchmark(c: &mut Criterion) {
    let map = I18nMap::from_dir("./test").unwrap();
    c.bench_function("LangMap get", |b| b.iter(|| map.get("en_us")));
}

criterion_group!(benches, langmap_benchmark);
criterion_main!(benches);
