#[macro_use]
extern crate lazy_static;
#[path = "../src/lib/day1.rs"] mod day1;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::path::Path;

fn criterion_benchmark(c: &mut Criterion) {
    let input = day1::get_input(Path::new("input/day1.txt"));
    c.bench_function("day1 part1", |b| b.iter(|| day1::solve_part1(black_box(day1::get_input(Path::new("input/day1.txt"))))));

    let input = day1::get_input(Path::new("input/day1.txt"));
    c.bench_function("day1 part2", |b| b.iter(|| day1::solve_part2(black_box(day1::get_input(Path::new("input/day1.txt"))))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
