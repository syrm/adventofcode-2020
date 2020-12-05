#[macro_use]
extern crate lazy_static;
#[path = "../src/lib/day3.rs"] mod day3;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::path::Path;

fn criterion_benchmark(c: &mut Criterion) {
    let input = day3::get_input(Path::new("input/day3.txt")).unwrap();
    let moves = [
        [1, 1].to_vec(),
        [3, 1].to_vec(),
        [5, 1].to_vec(),
        [7, 1].to_vec(),
        [1, 2].to_vec()
    ].to_vec();
    c.bench_function("day3 part1", |b| b.iter(|| day3::solve_part1(black_box(&input))));
    c.bench_function("day3 part2", |b| b.iter(|| day3::solve_part2(black_box(&input), black_box(&moves))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
