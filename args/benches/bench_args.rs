use criterion::{criterion_group, criterion_main, Criterion};
use star_args::Args;

fn single_with_one_delimiter(c: &mut Criterion) {
    c.bench_function(stringify!(single_with_one_delimiter), |b| {
        b.iter(|| {
            let mut args = Args::new("1,2", &[','.into()]);
            args.single::<String>().unwrap();
        })
    });
}

fn single_with_one_delimiter_and_long_string(c: &mut Criterion) {
    c.bench_function(stringify!(single_with_one_delimiter_and_long_string), |b| {
        b.iter(|| {
            let mut args = Args::new(
                "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25",
                &[','.into()],
            );

            args.single::<String>().unwrap();
        })
    });
}

fn single_with_three_delimiters(c: &mut Criterion) {
    c.bench_function(stringify!(single_with_three_delimiters), |b| {
        b.iter(|| {
            let mut args = Args::new("1,2 @3@4 5", &[','.into(), ' '.into(), '@'.into()]);
            args.single::<String>().unwrap();
        })
    });
}

fn single_with_three_delimiters_and_long_string(c: &mut Criterion) {
    c.bench_function(
        stringify!(single_with_three_delimiters_and_long_string),
        |b| {
            b.iter(|| {
                let mut args = Args::new(
                    "1,2 @3@4 5,1,2 @3@4 5,1,2 @3@4 5,1,2 @3@4 5,1,2 @3@4 5,1,2 @3@4 5,",
                    &[','.into(), ' '.into(), '@'.into()],
                );

                args.single::<String>().unwrap();
            })
        },
    );
}

fn single_quoted_with_one_delimiter(c: &mut Criterion) {
    c.bench_function(stringify!(single_quoted_with_one_delimiter), |b| {
        b.iter(|| {
            let mut args = Args::new(r#""1", "2""#, &[','.into()]);
            args.single_quoted::<String>().unwrap();
        })
    });
}

fn iter_with_one_delimiter(c: &mut Criterion) {
    c.bench_function(stringify!(iter_with_one_delimiter), |b| {
        b.iter(|| {
            let mut args = Args::new("1,2,3,4,5,6,7,8,9,10", &[','.into()]);
            args.iter::<String>()
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
        })
    });
}

fn iter_with_three_delimiters(c: &mut Criterion) {
    c.bench_function(stringify!(iter_with_three_delimiters), |b| {
        b.iter(|| {
            let mut args = Args::new(
                "1-2<3,4,5,6,7<8,9,10",
                &[','.into(), '-'.into(), '<'.into()],
            );

            args.iter::<String>()
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
        })
    });
}

criterion_group!(
    benches,
    single_with_one_delimiter,
    single_with_one_delimiter_and_long_string,
    single_with_three_delimiters,
    single_with_three_delimiters_and_long_string,
    single_quoted_with_one_delimiter,
    iter_with_one_delimiter,
    iter_with_three_delimiters
);
criterion_main!(benches);
