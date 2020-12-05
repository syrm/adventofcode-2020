use std::io::Error;
use std::result::Result;
use regex::Regex;
use std::path::Path;

#[derive(Debug)]
pub struct PasswordPolicy {
    character: char,
    criterion1: i8,
    criterion2: i8
}

#[path = "common.rs"] mod common;

#[inline]
pub fn solve_part1(passwords: &Vec<(PasswordPolicy, String)>) -> i32 {
    let mut passwords_valid = 0;

    for (password_policy, password) in passwords {
        let character_find = password
            .chars()
            .filter(|char| char == &password_policy.character)
            .count();

        if character_find as i8 >= password_policy.criterion1 && character_find as i8 <= password_policy.criterion2 {
            passwords_valid += 1;
        }
    }

    passwords_valid
}

#[inline]
pub fn solve_part2(passwords: &Vec<(PasswordPolicy, String)>) -> i32 {
    let mut passwords_valid = 0;

    for (password_policy, password) in passwords {
        let chars: Vec<char> = password.chars().collect();

        if chars[(password_policy.criterion1-1) as usize] == password_policy.character && chars[(password_policy.criterion2-1) as usize] != password_policy.character
            || chars[(password_policy.criterion1-1) as usize] != password_policy.character && chars[(password_policy.criterion2-1) as usize] == password_policy.character
        {
            passwords_valid += 1;
        }
    }

    passwords_valid
}

pub fn get_input(filename: &Path) -> Result<Vec<(PasswordPolicy, String)>, Error> {
    let result_lines = common::read_input(filename);
    let mut passwords: Vec<(PasswordPolicy, String)> = Vec::new();

    for line in result_lines? {
        parse_line(line).and_then(|password| Some(passwords.push(password))); // @TODO Pourquoi je dois mettre Some ?
    }

    Ok(passwords)
}

fn parse_line(line: String) -> Option<(PasswordPolicy, String)> {
    lazy_static! {
        static ref POLICY_REGEX: Regex = Regex::new(r"(?P<minimum>\d+)-(?P<maximum>\d+) (?P<character>\w): (?P<password>\w+)").unwrap();
    }

    for capture in POLICY_REGEX.captures_iter(&line) {
        let password_policy = PasswordPolicy{
            character: capture.name("character")?.as_str().chars().next().expect(""),
            criterion1: capture.name("minimum")?.as_str().parse::<i8>().unwrap(),
            criterion2: capture.name("maximum")?.as_str().parse::<i8>().unwrap()
        };

        return Some((password_policy,capture.name("password")?.as_str().to_owned()))
    }

    None
}
