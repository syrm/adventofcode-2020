use std::io::Error;
use std::path::Path;
use crate::lib::common;

#[inline]
pub fn solve_part1(boarding_passes: &Vec<String>) -> u16 {
    let mut highest_seat_id = 0;

    for boarding_pass in boarding_passes {
        let seat_id = get_seat_id(
            get_position(&boarding_pass[..7], 127),
            get_position(&boarding_pass[7..], 7)
        );

        if seat_id > highest_seat_id {
            highest_seat_id = seat_id;
        }
    }

    highest_seat_id
}

#[inline]
pub fn solve_part2(boarding_passes: &Vec<String>) -> u16 {
    let mut my_seat_id: u16 = 0;
    let mut seats: Vec<u16> = vec![0u16; 8*128];

    for boarding_pass in boarding_passes {
        let seat_id = get_seat_id(
            get_position(&boarding_pass[..7], 127),
            get_position(&boarding_pass[7..], 7)
        );
        seats[seat_id as usize] = 1;
    }

    for (index, occupied) in seats.iter().enumerate() {
        if occupied == &(0 as u16) && (index > 0 && seats[index-1] == 1) && (index+1 < seats.len() && seats[index+1] == 1) {
            my_seat_id = index as u16;
            break;
        }
    }

    my_seat_id
}

fn get_seat_id(row: u16, column: u16) -> u16 {
    row * 8 + column
}

fn get_position(boarding_pass: &str, max_position: u16) -> u16 {
    let mut upper_bound: u16 = max_position;
    let mut lower_bound: u16 = 0;

    for row_letter in boarding_pass.chars() {
        let offset = lower_bound + (upper_bound - lower_bound) / 2;
        match row_letter {
            'F' | 'L' => upper_bound = offset,
            'B' | 'R' => lower_bound = offset + 1,
             _  => panic!("Wrong boarding pass: {}", boarding_pass),
        };

        if upper_bound == lower_bound {
            break;
        }
    }

    upper_bound
}

pub fn get_input(filename: &Path) -> Result<Vec<String>, Error> {
    common::read_input(filename)
}
