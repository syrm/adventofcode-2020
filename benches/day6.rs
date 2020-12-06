#[macro_use]
extern crate lazy_static;
#[path = "../src/lib/day6.rs"] mod day6;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::path::Path;

fn criterion_benchmark(c: &mut Criterion) {
    let input = day6::get_input(Path::new("input/day6.txt")).unwrap();
    c.bench_function("day6 part1", |b| b.iter(|| day6::solve_part1(black_box(&input))));
    c.bench_function("day6 part2", |b| b.iter(|| day6::solve_part2(black_box(&input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
