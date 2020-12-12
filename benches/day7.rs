#[macro_use]
extern crate lazy_static;
#[path = "../src/lib/common.rs"]
mod common;
#[path = "../src/lib/day7.rs"]
mod day7;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::path::Path;

fn criterion_benchmark(c: &mut Criterion) {
    let input = day7::get_input(Path::new("input/day7.txt"));
    c.bench_function("day7 part1", |b| {
        b.iter(|| day7::solve_part1(black_box(&input), black_box("shiny gold".to_string())))
    });
    c.bench_function("day7 part2", |b| {
        b.iter(|| day7::solve_part2(black_box(&input), black_box("shiny gold".to_string())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
