use std::collections::VecDeque;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::ops::{Add, Mul};
use std::str;

use rstest::rstest;

#[derive(Debug, PartialEq)]
struct Monkey {
    items: VecDeque<u64>,
    operator: fn(u64, u64) -> u64,
    operand: Option<u64>,
    test: u64,
    if_true: usize,
    if_false: usize,
}

fn monkeys_from_file(filename: &str) -> VecDeque<Monkey> {
    let mut buf = Vec::new();
    File::open(filename).unwrap().read_to_end(&mut buf).unwrap();
    String::from_utf8(buf)
        .unwrap()
        .split("\n\n")
        .into_iter()
        .map(|monkey_str| {
            let monkey_lines: Vec<&str> = monkey_str.split("\n").collect();
            let items = monkey_lines[1][18..]
                .split(", ")
                .map(|i| i.parse().unwrap())
                .collect();
            let (operator_str, argument_str) = monkey_lines[2][23..].split_once(' ').unwrap();
            let operator = match operator_str {
                "*" => u64::mul,
                "+" => u64::add,
                _ => panic!("Unknown operator"),
            };
            let operand = match argument_str {
                "old" => None,
                s => Some(s.parse().unwrap()),
            };
            let test = monkey_lines[3][21..].parse().unwrap();
            let if_true = monkey_lines[4][29..].parse().unwrap();
            let if_false = monkey_lines[5][30..].parse().unwrap();
            Monkey {
                items,
                operator,
                operand,
                test,
                if_true,
                if_false,
            }
        })
        .collect()
}

#[test]
fn test_monkeys_from_file() {
    assert_eq!(
        monkeys_from_file("./test11.txt"),
        VecDeque::from([
            Monkey {
                items: VecDeque::from([79, 98]),
                operator: u64::mul,
                operand: Some(19),
                test: 23,
                if_true: 2,
                if_false: 3
            },
            Monkey {
                items: VecDeque::from([54, 65, 75, 74]),
                operator: u64::add,
                operand: Some(6),
                test: 19,
                if_true: 2,
                if_false: 0
            },
            Monkey {
                items: VecDeque::from([79, 60, 97]),
                operator: u64::mul,
                operand: None,
                test: 13,
                if_true: 1,
                if_false: 3
            },
            Monkey {
                items: VecDeque::from([74]),
                operator: u64::add,
                operand: Some(3),
                test: 17,
                if_true: 0,
                if_false: 1
            },
        ])
    );
}

fn do_monkey_business(mut monkeys: VecDeque<Monkey>, rounds: usize, relief: u64) -> u64 {
    let num_monkeys = monkeys.len();
    let mut inspections = vec![0; num_monkeys];

    let max_worry = monkeys.iter().map(|m| m.test).fold(1, u64::mul);
    println!("{}", max_worry);

    for _ in 0..rounds {
        for i in 0..num_monkeys {
            let mut monkey = monkeys.pop_front().unwrap();
            for _ in 0..monkey.items.len() {
                let mut item = monkey.items.pop_front().unwrap();
                inspections[i] += 1;
                item = ((monkey.operator)(
                    item,
                    match monkey.operand {
                        Some(n) => n,
                        None => item,
                    },
                ) / relief)
                    % max_worry;
                if item % monkey.test == 0 {
                    let idx = match i < monkey.if_true {
                        true => monkey.if_true - 1 - i,
                        false => monkey.if_true + num_monkeys - 1 - i,
                    };
                    monkeys[idx].items.push_back(item);
                } else {
                    let idx = match i < monkey.if_false {
                        true => monkey.if_false - 1 - i,
                        false => monkey.if_false + num_monkeys - 1 - i,
                    };
                    monkeys[idx].items.push_back(item);
                }
            }
            monkeys.push_back(monkey);
        }
    }
    inspections.sort();
    inspections.into_iter().rev().take(2).fold(1, u64::mul)
}

pub fn part_1(filename: &str) -> u64 {
    do_monkey_business(monkeys_from_file(filename), 20, 3)
}

#[rstest]
#[case::test("./test11.txt", 10605)]
#[case::input("./input11.txt", 54752)]
fn test_part_1(#[case] filename: &str, #[case] result: u64) {
    assert_eq!(part_1(filename), result);
}

pub fn part_2(filename: &str) -> u64 {
    do_monkey_business(monkeys_from_file(filename), 10_000, 1)
}

#[rstest]
#[case::test("./test11.txt", 2713310158)]
#[case::input("./input11.txt", 13606755504)]
fn test_part_2(#[case] filename: &str, #[case] result: u64) {
    assert_eq!(part_2(filename), result);
}
