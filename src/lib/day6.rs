use std::io::Error;
use std::path::Path;
use std::collections::{HashSet, HashMap};
use super::common;

#[inline]
pub fn solve_part1(groups: &Vec<Vec<String>>) -> u16 {
    let mut answered_yes: u16 = 0;
    let mut group_answer = HashSet::new();

    for group in groups {
        for passenger in group {
            for answer in passenger.chars() {
                group_answer.insert(answer);
            }
        }

        answered_yes += group_answer.len() as u16;
        group_answer.clear();
    }

    answered_yes
}

#[inline]
pub fn solve_part2(groups: &Vec<Vec<String>>) -> u16 {
    let mut answered_yes: u16 = 0;
    let mut group_answer = HashMap::new();

    for group in groups {
        for passenger in group {
            for answer in passenger.chars() {
                group_answer.insert(answer, group_answer.get(&answer).unwrap_or(&0) + 1);
            }
        }

        for (_, answer) in group_answer {
            if answer == group.len() {
                answered_yes += 1;
            }
        }

        group_answer = HashMap::new();
    }

    answered_yes
}

pub fn get_input(filename: &Path) -> Result<Vec<Vec<String>>, Error> {
    let result_lines = common::read_input(filename);
    let mut groups: Vec<Vec<String>> = Vec::new();
    let mut passengers: Vec<String> = Vec::new();

    for line in result_lines? {
        if line == "" {
            groups.push(passengers);
            passengers = Vec::new();
            continue;
        }

        passengers.push(line);
    }

    groups.push(passengers);

    Ok(groups)
}
