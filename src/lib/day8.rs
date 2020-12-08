use std::io::Error;
use std::path::Path;
use std::collections::HashMap;
use super::common;

#[derive(Debug)]
pub struct Instruction {
    operation: Operation,
    argument: isize,
}

#[derive(Debug)]
enum Operation {
    Acc,
    Jmp,
    Nop,
}

#[inline]
pub fn solve_part1(code: &Vec<Instruction>) -> isize {
    let mut accumulator: isize = 0;
    let mut index = 0;
    let mut line_executed: HashMap<usize, i8> = HashMap::new();

    loop {
        if line_executed.contains_key(&index) {
            break;
        }

        match code[index] {
            Instruction { operation: Operation::Acc, argument } => accumulator += argument,
            Instruction { operation: Operation::Jmp, argument } => index = (index as isize + argument - 1) as usize,
            Instruction { operation: Operation::Nop, argument: _ } => {},
        }

        line_executed.insert(index, 1);
        index += 1;

        if index >= code.len() {
            break;
        }
    }

    accumulator
}

#[inline]
pub fn solve_part2(code: &Vec<Instruction>) -> isize {
    let mut patchs_tried: HashMap<usize, i8> = HashMap::new();
    let mut accumulator: isize;
    let mut index: usize;
    let mut line_executed: HashMap<usize, i8>;
    let mut patched: bool;

    'try_new_patch: loop {
        accumulator = 0;
        index = 0;
        line_executed = HashMap::new();
        patched = false;

        loop {
            if patched == true && line_executed.contains_key(&index) {
                break;
            }

            line_executed.insert(index, 1);

            match code[index] {
                Instruction { operation: Operation::Acc, argument } => {
                    accumulator += argument;
                },
                Instruction { operation: Operation::Jmp, argument } => {
                    let new_index = (index as isize + argument - 1) as usize;

                    if patched || patchs_tried.contains_key(&index) {
                        index = new_index;
                    } else {
                        patched = true;
                        patchs_tried.insert(index, 1);
                    }
                },
                Instruction { operation: Operation::Nop, argument } => {
                    let new_index = (index as isize + argument - 1) as usize;

                    if patched || patchs_tried.contains_key(&index) {
                        // nop
                    } else {
                        patchs_tried.insert(index, 1);
                        index = new_index;
                        patched = true;
                    }
                },
            }

            index += 1;

            if index >= code.len() {
                break 'try_new_patch;
            }
        }
    }

    accumulator
}

pub fn get_input(filename: &Path) ->  Result<Vec<Instruction>, Error> {
    let result_lines = common::read_input(filename);
    let mut code: Vec<Instruction> = Vec::new();

    for line in result_lines? {
        let (operation, argument) = line.split_at(3);
        let operation = match operation {
            "acc" => Some(Operation::Acc),
            "jmp" => Some(Operation::Jmp),
            "nop" => Some(Operation::Nop),
            _ => None,
        };

        match operation {
            Some(operation) => code.push(Instruction { operation, argument: argument[1..].parse::<isize>().unwrap() }),
            None => {},
        }
    }

    Ok(code)
}
