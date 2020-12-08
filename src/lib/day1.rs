use std::path::Path;
use super::common;

#[inline]
pub fn solve_part1(numbers: impl Iterator<Item = usize>) -> u32
{
    let numbers: Vec<usize> = numbers.collect();
    for number in numbers.iter() {
        for number_to_add in numbers.iter() {
           if number + number_to_add == 2020 {
                return (number * number_to_add) as u32;
            }
        }
    }

    0
}

#[inline]
pub fn solve_part2(numbers: impl Iterator<Item = usize>) -> usize
{
    let numbers: Vec<usize> = numbers.collect();

    for number in numbers.iter() {
        for number_to_add in numbers.iter() {
            for number_to_add_again in numbers.iter() {
                if number + number_to_add + number_to_add_again == 2020 {
                    return number * number_to_add * number_to_add_again;
                }
            }
        }
    }

    0
}

pub fn get_input(filename: &Path) -> impl Iterator<Item = usize> {
    common::read_input_as_integer(filename)
}


