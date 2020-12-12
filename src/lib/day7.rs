use crate::lib::common;
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;

type Bags = HashMap<String, Vec<BagContained>>;
type BagsFound = HashMap<String, bool>;

#[derive(Debug)]
pub struct BagContained {
    name: String,
    quantity: u8,
}

#[inline]
pub fn solve_part1(bags: &Bags, bag_to_found: String) -> u16 {
    let mut bags_found: BagsFound = HashMap::new();

    for bag in bags.keys() {
        bags_found = analyse_bag_part1(bag.clone(), bag.clone(), bags, bags_found, &bag_to_found);
    }

    bags_found.len() as u16
}

fn analyse_bag_part1(
    bag_name: String,
    bag_to_analyse: String,
    bags: &Bags,
    mut bags_found: BagsFound,
    bag_to_found: &String,
) -> BagsFound {
    match bags.get(&bag_to_analyse) {
        Some(bags_contained) => {
            for bag_contained in bags_contained {
                if &bag_contained.name == bag_to_found {
                    bags_found.insert(bag_name.clone(), true);
                } else {
                    bags_found = analyse_bag_part1(
                        bag_name.clone(),
                        bag_contained.name.clone(),
                        bags,
                        bags_found,
                        bag_to_found,
                    );
                }
            }
        }
        None => {}
    }

    bags_found
}

#[inline]
pub fn solve_part2(bags: &Bags, bag_to_found: String) -> u16 {
    let mut bag_quantity: u16 = 0;

    for bag_contained in bags.get(&bag_to_found).unwrap() {
        bag_quantity += bag_contained.quantity as u16 * analyse_bag_part2(&bag_contained.name, bags)
    }

    bag_quantity
}

fn analyse_bag_part2(bag_name: &String, bags: &Bags) -> u16 {
    let mut bag_quantity: u16 = 1;

    match bags.get(bag_name) {
        Some(bags_contained) => {
            for bag_contained in bags_contained {
                bag_quantity +=
                    bag_contained.quantity as u16 * analyse_bag_part2(&bag_contained.name, bags)
            }
        }
        None => {}
    }

    return bag_quantity;
}

pub fn get_input(filename: &Path) -> Bags {
    let result_lines = common::read_input(filename);
    let mut bags: HashMap<String, Vec<BagContained>> = HashMap::new();

    let bag_regex: Regex = Regex::new(r"(?P<bag>[a-z ]+) bags contain").unwrap();
    let bag_contained_regex: Regex =
        Regex::new(r" (?P<quantity>\d+) (?P<bag_contained>[a-z ]+) bag").unwrap();

    for line in result_lines.unwrap() {
        let bag_capture = bag_regex.captures(&line).unwrap();
        let bag = bag_capture.name("bag").unwrap().as_str().to_string();

        for bag_contained in bag_contained_regex.captures_iter(&line) {
            let bag_name = bag_contained.name("bag_contained").unwrap().as_str();
            let bag_quantity = bag_contained
                .name("quantity")
                .unwrap()
                .as_str()
                .parse::<u8>()
                .unwrap();

            match bags.get_mut(&bag) {
                Some(bag_container) => {
                    bag_container.push(BagContained {
                        name: bag_name.to_string(),
                        quantity: bag_quantity,
                    });
                }
                None => {
                    bags.insert(
                        bag.clone(),
                        vec![BagContained {
                            name: bag_name.to_string(),
                            quantity: bag_quantity,
                        }],
                    );
                }
            }
        }
    }

    bags
}
