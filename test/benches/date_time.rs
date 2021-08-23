use chrono::{DateTime, Utc, MAX_DATETIME, MIN_DATETIME};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn time_format(dt: DateTime<Utc>) -> String {
    format!("{:?}", dt)
}

fn time_to_string(dt: DateTime<Utc>) -> String {
    dt.format("%+").to_string()
}

fn bench_formats(c: &mut Criterion) {
    let mut group = c.benchmark_group("DateTime formatting");
    for i in [MIN_DATETIME, MAX_DATETIME] {
        group.bench_with_input(BenchmarkId::new("Macro", i), &i, |b, i| {
            b.iter(|| time_format(*i))
        });
        group.bench_with_input(BenchmarkId::new("ToString", i), &i, |b, i| {
            b.iter(|| time_to_string(*i))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_formats);
criterion_main!(benches);
