use std::collections::HashMap;
use std::io::Error;
use regex::Regex;
use std::path::Path;

#[path = "common.rs"] mod common;

type Passport = HashMap<String, String>;

#[inline]
pub fn solve_part1(passports: &Vec<Passport>) -> u16 {
    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut passports_valid = 0;

    'passport: for passport in passports {
        for required_field in &required_fields {
            if passport.get(&required_field.to_string()) == None {
                continue 'passport;
            }
        }
        passports_valid += 1;
    }

    passports_valid
}

#[inline]
pub fn solve_part2(passports: &Vec<Passport>) -> u16 {
    let mut required_fields: HashMap<&str, fn(String) -> bool> = HashMap::new();

    required_fields.insert(
      "byr",
      |data| {
          let year = data.parse::<u16>();
          match year {
              Err(_) => false,
              Ok(year) => year >= 1920 && year <= 2002,
          }
      },
    );

    required_fields.insert(
        "iyr",
        |data| {
            let year = data.parse::<u16>();
            match year {
                Err(_) => false,
                Ok(year) => year >= 2010 && year <= 2020,
            }
        },
    );

    required_fields.insert(
        "eyr",
        |data| {
            let year = data.parse::<u16>();
            match year {
                Err(_) => false,
                Ok(year) => year >= 2020 && year <= 2030,
            }
        },
    );

    required_fields.insert(
        "hgt",
        |data| {
            match data.get(data.len()-2..data.len()) {
                Some("cm") => {
                    let size = data.get(..data.len()-2);
                    match size {
                        Some(size) => match size.parse::<u8>() {
                            Ok(size) => size >= 150 && size <= 193,
                            Err(_) => false,
                        }
                        None => false,
                    }
                },
                Some("in") => {
                    let size = data.get(..data.len()-2);
                    match size {
                        Some(size) => match size.parse::<u8>() {
                            Ok(size) => size >= 59 && size <= 76,
                            Err(_) => false,
                        }
                        None => false,
                    }
                },
                _ => false,
            }
        },
    );

    required_fields.insert(
        "hcl",
        |data| {
            lazy_static! {
                static ref HAIR_REGEX: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            }

            HAIR_REGEX.is_match(&data)
        },
    );

    required_fields.insert(
        "ecl",
        |data| {
            lazy_static! {
                static ref EYE_COLOR: Vec<&'static str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
            }

            EYE_COLOR.contains(&data.as_str())
        },
    );

    required_fields.insert(
        "pid",
        |data| {
            lazy_static! {
                static ref PASSPORT_ID_REGEX: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
            }

            PASSPORT_ID_REGEX.is_match(&data)
        },
    );

    let mut passports_valid = 0;

    'passport: for passport in passports {
        for (required_field, check_field) in &required_fields {
            match passport.get(&required_field.to_string()) {
                None => continue 'passport,
                Some(data) => {
                    if  check_field(data.to_string()) == false {
                        continue 'passport;
                    }
                },
            }
        }
        passports_valid += 1;
    }

    passports_valid
}

pub fn get_input(filename: &Path) -> Result<Vec<Passport>, Error> {
    let result_lines = common::read_input(filename);
    let mut passports: Vec<Passport> = Vec::new();
    let mut passport: Passport = HashMap::new();

    for line in result_lines? {
        if line == "" {
            passports.push(passport);
            passport = HashMap::new();
            continue;
        }

        let keys_value: Vec<&str> = line.split_terminator(' ').collect();

        for key_value in keys_value {
            let key_value_splitted: Vec<&str> = key_value.splitn(2, ':').collect();

            passport.insert(key_value_splitted[0].to_string(), key_value_splitted[1].to_string());
        }
    }

    passports.push(passport);

    Ok(passports)
}
