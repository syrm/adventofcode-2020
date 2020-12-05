#[macro_use]
extern crate lazy_static;

mod lib;
use lib::*;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let default = day5;

    match &args.get(1) {
        None => default(),
        Some(arg) => match arg.parse() {
            Ok(2) => day2(),
            Ok(3) => day3(),
            Ok(4) => day4(),
            _ => default()
        }
    }
}

fn day2() {
    let input = day2::get_input(Path::new("input/day2.txt")).unwrap();

    println!("Day 2");
    println!("Part 1 answer is {}", day2::solve_part1(&input));
    println!("Part 2 answer is {}", day2::solve_part2(&input));
}

fn day3() {
    let input = day3::get_input(Path::new("input/day3.txt")).unwrap();

    println!("Day 3");
    println!("Part 1 answer is {}", day3::solve_part1(&input));
    println!("Part 2 answer is {}", day3::solve_part2(
        &input,
        &[
            [1, 1].to_vec(),
            [3, 1].to_vec(),
            [5, 1].to_vec(),
            [7, 1].to_vec(),
            [1, 2].to_vec()
        ].to_vec()
    ));
}

fn day4() {
    let input = day4::get_input(Path::new("input/day4.txt")).unwrap();
    let answer1 = day4::solve_part1(&input);
    let answer2 = day4::solve_part2(&input);

    println!("Day 4");
    println!("Part 1 answer is {}", answer1);
    assert_eq!(200, answer1);
    println!("Part 2 answer is {}", answer2);
    assert_eq!(116, answer2);
}

fn day5() {
    let input = day5::get_input(Path::new("input/day5.txt")).unwrap();
    let answer1 = day5::solve_part1(&input);
    let answer2 = day5::solve_part2(&input);

    println!("Day 5");
    println!("Part 1 answer is {}", answer1);
    assert_eq!(989, answer1);
    println!("Part 2 answer is {}", answer2);
    assert_eq!(548, answer2);
}