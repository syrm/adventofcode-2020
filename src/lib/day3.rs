use std::io::Error;
use std::path::Path;
use crate::lib::common;

#[inline]
pub fn solve_part1(area: &Vec<Vec<u8>>) -> u16 {
    let mut x = 0;
    let mut y = 0;
    let mut trees: u16 = 0;
    let move_x = 3;
    let move_y = 1;
    let area_height = area.len();
    let area_width = area[0].len();

    while y < area_height - 1 {
        x += move_x;
        y += move_y;

        trees += area[y][x % area_width] as u16;
    }

    trees
}

#[inline]
pub fn solve_part2(area: &Vec<Vec<u8>>, moves: &Vec<Vec<u8>>) -> u32 {
    let mut x: Vec<u16> = vec![0; moves.len()];
    let mut y: Vec<u16> = vec![0; moves.len()];
    let mut trees: Vec<u32> = vec![0; moves.len()];
    let area_height: u16 = area.len() as u16;
    let area_width: u16 = area[0].len() as u16;

    while y.iter().min().unwrap() < &(area_height - 1) {
        for (index, movexy) in moves.iter().enumerate() {
            x[index] += movexy[0] as u16;
            y[index] += movexy[1] as u16;

            if y[index] <= area_height - 1 {
                trees[index] += area[y[index] as usize][(x[index] % area_width) as usize] as u32;
            }
        }
    }

    trees.iter().fold(1, |answer, tree| answer * tree)
}

pub fn get_input(filename: &Path) -> Result<Vec<Vec<u8>>, Error> {
    let result_lines = common::read_input(filename);
    let mut aera: Vec<Vec<u8>> = Vec::new();

    for line in result_lines? {
        let mut aera_line: Vec<u8> = Vec::new();
        for character in line.chars() {
            if character == '#' {
                aera_line.push(1);
            } else {
                aera_line.push(0);
            }
        }
        aera.push(aera_line);
    }

    Ok(aera)
}
