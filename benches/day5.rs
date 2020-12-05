#[macro_use]
extern crate lazy_static;
#[path = "../src/lib/day5.rs"] mod day5;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::path::Path;

fn criterion_benchmark(c: &mut Criterion) {
    let input = day5::get_input(Path::new("input/day5.txt")).unwrap();
    c.bench_function("day5 part1", |b| b.iter(|| day5::solve_part1(black_box(&input))));
    c.bench_function("day5 part2", |b| b.iter(|| day5::solve_part2(black_box(&input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
