use std::{fs::File, io};

use crate::fs;

struct CaloriesInput {
    lines: io::Lines<io::BufReader<File>>,
    processing: bool,
    current: u32,
}

impl Iterator for CaloriesInput {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        for line in &mut self.lines {
            match line {
                Ok(line) => match &line[..] {
                    "" => break,
                    _ => {
                        let calories = line.parse::<u32>().expect("calories not parseable as u32");
                        self.current += calories;
                        self.processing = true;
                    }
                },
                Err(_) => {}
            }
        }
        if self.processing {
            let current = self.current;
            self.current = 0;
            self.processing = false;
            Some(current)
        } else {
            None
        }
    }
}

fn file_calories(filename: &str) -> io::Result<CaloriesInput> {
    let lines = fs::read_lines(filename)?;
    Ok(CaloriesInput {
        lines,
        processing: false,
        current: 0,
    })
}

#[test]
fn test_file_calories() {
    let elves = file_calories("./test01.txt").unwrap();
    assert_eq!(
        elves.into_iter().collect::<Vec<u32>>(),
        vec![6_000, 4_000, 11_000, 24_000, 10_000]
    );
}

pub fn part_1(filename: &str) -> u32 {
    let mut most_calories = 0_u32;
    if let Ok(elves) = file_calories(filename) {
        for calories in elves {
            if calories > most_calories {
                most_calories = calories;
            }
        }
    }
    most_calories
}

#[test]
fn test_part_1() {
    assert_eq!(part_1("./test01.txt"), 24000);
}

pub fn part_2(filename: &str) -> u32 {
    let mut top_three = [0_u32; 3];
    if let Ok(elves) = file_calories(filename) {
        for calories in elves {
            if calories >= top_three[0] {
                top_three[2] = top_three[1];
                top_three[1] = top_three[0];
                top_three[0] = calories;
            } else if calories >= top_three[1] {
                top_three[2] = top_three[1];
                top_three[1] = calories;
            } else if calories > top_three[2] {
                top_three[2] = calories;
            }
        }
    }
    top_three.iter().sum()
}

#[test]
fn test_part_2() {
    assert_eq!(part_2("./test01.txt"), 45000);
}
