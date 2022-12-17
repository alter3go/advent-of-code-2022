use std::collections::VecDeque;

use crate::fs;

pub fn part_1(filename: &str) -> String {
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    let mut lines = fs::read_lines(filename).unwrap().filter_map(|s| s.ok());
    let crate_section = lines.by_ref().take_while(|s| s != "");
    for layer in crate_section {
        if layer == String::from("") {
            break;
        }
        for (i, doof) in layer.as_bytes().chunks(4).enumerate() {
            if stacks.len() <= i {
                stacks.insert(i, VecDeque::new());
            }
            if doof[0] == '[' as u8 {
                stacks[i].push_back(doof[1] as char);
            }
        }
    }
    for crate_move in lines {
        let op = &crate_move[5..];
        let (count_str, rest) = op.split_once(" from ").unwrap();
        let (from_str, to_str) = rest.split_once(" to ").unwrap();
        let count: usize;
        let from: usize;
        let to: usize;
        (count, from, to) = (
            count_str.parse().unwrap(),
            from_str.parse().unwrap(),
            to_str.parse().unwrap(),
        );
        for _ in 0..count {
            let deal = stacks[from - 1].pop_front().unwrap();
            stacks[to - 1].push_front(deal);
        }
    }
    stacks.into_iter().map(|s| s[0]).collect()
}

#[test]
fn test_part_1() {
    assert_eq!(part_1("./test05.txt"), "CMZ");
}

pub fn part_2(filename: &str) -> String {
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    let mut lines = fs::read_lines(filename).unwrap().filter_map(|s| s.ok());
    let crate_section = lines.by_ref().take_while(|s| s != "");
    for layer in crate_section {
        if layer == String::from("") {
            break;
        }
        for (i, doof) in layer.as_bytes().chunks(4).enumerate() {
            if stacks.len() <= i {
                stacks.insert(i, VecDeque::new());
            }
            if doof[0] == '[' as u8 {
                stacks[i].push_back(doof[1] as char);
            }
        }
    }
    for crate_move in lines {
        let op = &crate_move[5..];
        let (count_str, rest) = op.split_once(" from ").unwrap();
        let (from_str, to_str) = rest.split_once(" to ").unwrap();
        let count: usize;
        let from: usize;
        let to: usize;
        (count, from, to) = (
            count_str.parse().unwrap(),
            from_str.parse().unwrap(),
            to_str.parse().unwrap(),
        );
        let mut deal: VecDeque<_> = stacks[from - 1].drain(..count).collect();
        deal.append(&mut stacks[to - 1]);
        stacks[to - 1] = deal;
    }
    stacks.into_iter().map(|s| s[0]).collect()
}

#[test]
fn test_part_2() {
    assert_eq!(part_2("./test05.txt"), "MCD");
}
