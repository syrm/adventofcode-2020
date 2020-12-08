#[macro_use]
extern crate lazy_static;
#[path = "../src/lib/day8.rs"] mod day8;
#[path = "../src/lib/common.rs"] mod common;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::path::Path;

fn criterion_benchmark(c: &mut Criterion) {
    let input = day8::get_input(Path::new("input/day8.txt")).unwrap();
    c.bench_function("day8 part1", |b| b.iter(|| day8::solve_part1(black_box(&input))));
    c.bench_function("day8 part2", |b| b.iter(|| day8::solve_part2(black_box(&input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
