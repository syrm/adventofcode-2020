#[macro_use]
extern crate lazy_static;
#[path = "../src/lib/common.rs"]
mod common;
#[path = "../src/lib/day9.rs"]
mod day9;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::path::Path;

fn criterion_benchmark(c: &mut Criterion) {
    let input = day9::get_input(Path::new("input/day9.txt"));
    c.bench_function("day9 part1", |b| {
        b.iter(|| day9::solve_part1(black_box(&input), black_box(25)))
    });
    c.bench_function("day9 part2", |b| {
        b.iter(|| day9::solve_part2(black_box(&input), black_box(10884537)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
