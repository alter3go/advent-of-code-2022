use std::{collections::HashSet, fs::File, io};

use rstest::rstest;

use crate::fs;

#[derive(Default)]
struct Rucksack {
    first_compartment: HashSet<char>,
    second_compartment: HashSet<char>,
}

struct RucksacksInput {
    lines: io::Lines<io::BufReader<File>>,
}

impl Iterator for RucksacksInput {
    type Item = Rucksack;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lines.next() {
            None => None,
            Some(line_result) => {
                let line = line_result.unwrap();
                let item_count = line.len();
                let boundary = item_count / 2;
                let mut rucksack = Rucksack {
                    ..Default::default()
                };
                for (i, item) in line.chars().enumerate() {
                    if i < boundary {
                        rucksack.first_compartment.insert(item);
                    } else {
                        rucksack.second_compartment.insert(item);
                    }
                }
                Some(rucksack)
            }
        }
    }
}

fn get_priority(item: char) -> Result<u32, &'static str> {
    match item {
        'a'..='z' => Ok(item as u32 - 96),
        'A'..='Z' => Ok(item as u32 - 38),
        _ => Err("Invalid rucksack item"),
    }
}

#[rstest]
#[case('a', 1)]
#[case('z', 26)]
#[case('A', 27)]
#[case('Z', 52)]
#[should_panic]
#[case('@', 0)]
fn test_get_priority(#[case] item: char, #[case] priority: u32) {
    assert_eq!(get_priority(item).unwrap(), priority);
}

pub fn part_1(filename: &str) -> u32 {
    let lines = fs::read_lines(filename).unwrap();
    let rucksacks_input = RucksacksInput { lines };
    let mut total_priority = 0;
    for rucksack in rucksacks_input {
        for item in rucksack.first_compartment {
            if rucksack.second_compartment.contains(&item) {
                let priority = get_priority(item).unwrap();
                total_priority += priority;
                break;
            }
        }
    }
    total_priority
}

#[test]
fn test_part_1() {
    assert_eq!(part_1("./test03.txt"), 157);
}

pub fn part_2(filename: &str) -> u32 {
    let lines = fs::read_lines(filename).unwrap();
    let rucksacks_input = RucksacksInput { lines };
    let mut total_priority = 0;
    let mut common_items = HashSet::new();
    for (i, rucksack) in rucksacks_input.enumerate() {
        if i % 3 == 0 {
            common_items.clear();
            for item in ('a'..='z').chain('A'..='Z') {
                common_items.insert(item);
            }
        }

        let rucksack_items: HashSet<_> = rucksack
            .first_compartment
            .union(&rucksack.second_compartment)
            .copied()
            .collect();
        common_items = common_items
            .intersection(&rucksack_items)
            .copied()
            .collect();

        if i % 3 == 2 {
            assert_eq!(common_items.len(), 1);
            let badge = common_items.drain().next().unwrap();
            total_priority += get_priority(badge).unwrap();
        }
    }
    total_priority
}

#[test]
fn test_part_2() {
    assert_eq!(part_2("./test03.txt"), 70);
}
