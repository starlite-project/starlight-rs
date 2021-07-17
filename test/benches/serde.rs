use bincode::{serialize, deserialize};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use serde::{Deserialize, Serialize};
use serde_cbor::{to_vec, from_slice};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
struct Coordinate(f64);

impl Display for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
struct Point(Coordinate, Coordinate);

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Grid(Vec<Point>);

impl Default for Grid {
    fn default() -> Self {
        let mut vec = Vec::with_capacity(10000);
        for i in 0..10000 {
            let coord = Coordinate(i.into());
            let point = Point(coord, coord);
            vec.push(point);
        }

        Self(vec)
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}..{}", self.0.first().unwrap(), self.0.last().unwrap())
    }
}

fn bench_serialize(c: &mut Criterion) {
    let mut group = c.benchmark_group("Serializers");
    let grid = Grid::default();
    group.bench_with_input(BenchmarkId::new("Cbor", &grid), &grid, |b, i| {
        b.iter(|| to_vec(i).unwrap())
    });
    group.bench_with_input(BenchmarkId::new("Bincode", &grid), &grid, |b, i| {
        b.iter(|| serialize(i).unwrap())
    });
    group.finish();
}

fn bench_deserialize(c: &mut Criterion) {
    let mut group = c.benchmark_group("Deserializers");
    let grid = Grid::default();
    let bincode_input = serialize(&grid).unwrap();
    let cbor_input = to_vec(&grid).unwrap();
    group.bench_with_input(BenchmarkId::new("Cbor", &grid), &cbor_input[..], |b, i| {
        b.iter(|| from_slice::<Grid>(i).unwrap())
    });
    group.bench_with_input(BenchmarkId::new("Bincode", &grid), &bincode_input[..], |b, i| {
        b.iter(|| deserialize::<Grid>(i).unwrap())
    });
    group.finish();
}

criterion_group!(benches, bench_serialize, bench_deserialize);
criterion_main!(benches);
