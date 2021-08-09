use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};
use rayon::prelude::*;

fn sum_of_squares_sync(input: &[i32]) -> i32 {
    input.iter().map(|&i| i * i).sum()
}

fn sum_of_squares_rayon(input: &[i32]) -> i32 {
    input.par_iter().map(|&i| i * i).sum()
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Rayon");
    let mut input = Vec::with_capacity(100);
    for i in 0..100 {
        input.push(i);
    }

    group.bench_with_input(BenchmarkId::new("Std", "Vector"), &black_box(input.clone()), |b, i| {
        b.iter(|| sum_of_squares_sync(&i[..]))
    });
    group.bench_with_input(BenchmarkId::new("Rayon", "Vector"), &black_box(input), |b, i| {
        b.iter(|| sum_of_squares_rayon(&i[..]))
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
