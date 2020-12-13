use super::common;
use std::path::Path;

type Xmas = Vec<usize>;

#[inline]
pub fn solve_part1(xmas: &Xmas, preamble: usize) -> usize {
    for (index, number) in xmas.iter().enumerate() {
        if index < preamble {
            continue;
        }

        if find_numbers(index, number, &xmas, preamble) == false {
            return *number;
        }
    }

    0
}

pub fn find_numbers(
    number_index: usize,
    number: &usize,
    numbers: &Vec<usize>,
    preamble: usize,
) -> bool {
    for (first_index, first_number) in numbers.iter().enumerate() {
        if first_index >= number_index
            || (number_index >= preamble && first_index < number_index - preamble)
        {
            continue;
        }

        for (second_index, second_number) in numbers.iter().enumerate() {
            if second_index >= first_index
                || (number_index >= preamble as usize && second_index < number_index - preamble)
            {
                continue;
            }

            if first_number != second_number && first_number + second_number == *number {
                return true;
            }
        }
    }

    false
}

#[inline]
pub fn solve_part2(xmas: &Xmas, sum_to_found: usize) -> usize {
    let mut lower_bound: usize = 0;

    'try_again: loop {
        let mut smallest_number: usize = usize::MAX;
        let mut largest_number: usize = usize::MIN;
        let mut current_sum: usize = 0;
        let mut contiguous_size: usize = 0;

        for (index, number) in xmas.iter().enumerate() {
            if index < lower_bound {
                continue;
            }

            current_sum += number;
            contiguous_size += 1;

            if smallest_number > *number {
                smallest_number = *number;
            }

            if largest_number < *number {
                largest_number = *number;
            }

            if current_sum == sum_to_found && contiguous_size >= 2 {
                return smallest_number + largest_number;
            }

            if current_sum > sum_to_found {
                lower_bound += 1;
                continue 'try_again;
            }

            if lower_bound == xmas.len() - 1 {
                break 'try_again;
            }
        }
    }

    0
}

pub fn get_input(filename: &Path) -> Xmas {
    let result_numbers = common::read_input_as_integer(filename);
    let mut numbers: Vec<usize> = Vec::new();

    for number in result_numbers {
        numbers.push(number);
    }

    numbers
}
