use std::{
    io::{self, BufRead},
    ops::RangeInclusive,
};

use rstest::rstest;

use crate::fs;

struct AssignmentPairsInput<B> {
    lines: io::Lines<B>,
}

type AssignmentPair = (RangeInclusive<u32>, RangeInclusive<u32>);

impl<B> Iterator for AssignmentPairsInput<B>
where
    B: BufRead,
{
    type Item = AssignmentPair;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.next()?.unwrap();
        let (left, right) = line.split_once(',').unwrap();
        let (lss, les) = left.split_once('-').unwrap();
        let (rss, res) = right.split_once('-').unwrap();
        Some((
            lss.parse().unwrap()..=les.parse().unwrap(),
            rss.parse().unwrap()..=res.parse().unwrap(),
        ))
    }
}

#[rstest]
#[case("1-2,5-100\n3-4,100-1000", vec![(1..=2, 5..=100), (3..=4, 100..=1000)])]
#[case("7-11,13-14", vec![(7..=11, 13..=14)])]
#[case("", vec![])]
fn test_assignment_pairs_input(#[case] lines_str: String, #[case] expected: Vec<AssignmentPair>) {
    let lines = io::Cursor::new(lines_str).lines();
    let assignment_pairs_input = AssignmentPairsInput { lines };
    assert_eq!(
        assignment_pairs_input.collect::<Vec<AssignmentPair>>(),
        expected
    );
}

fn one_contains_other(pair: AssignmentPair) -> bool {
    (pair.0.contains(&pair.1.start()) && pair.0.contains(&pair.1.end()))
        || (pair.1.contains(&pair.0.start()) && pair.1.contains(&pair.0.end()))
}

#[rstest]
#[case((1..=4, 2..=3), true)]
#[case((2..=3, 1..=4), true)]
#[case((1..=4, 2..=5), false)]
#[case((2..=5, 1..=4), false)]
#[case((1..=1, 1..=2), true)]
fn test_one_contains_other(#[case] pair: AssignmentPair, #[case] result: bool) {
    assert_eq!(one_contains_other(pair), result);
}

pub fn part_1(filename: &str) -> u32 {
    let mut result = 0;
    for pair in (AssignmentPairsInput {
        lines: fs::read_lines(filename).unwrap(),
    }) {
        if one_contains_other(pair) {
            result += 1;
        }
    }
    result
}

#[test]
fn test_part_1() {
    assert_eq!(part_1("./test04.txt"), 2);
}

fn overlaps(pair: AssignmentPair) -> bool {
    pair.0.contains(pair.1.start())
        || pair.0.contains(pair.1.end())
        || pair.1.contains(pair.0.start())
        || pair.1.contains(pair.0.end())
}

#[rstest]
#[case((1..=4, 2..=3), true)]
#[case((2..=3, 1..=4), true)]
#[case((1..=4, 2..=5), true)]
#[case((2..=5, 1..=4), true)]
#[case((1..=2, 2..=3), true)]
#[case((1..=2, 3..=4), false)]
fn test_overlaps(#[case] pair: AssignmentPair, #[case] result: bool) {
    assert_eq!(overlaps(pair), result)
}

pub fn part_2(filename: &str) -> u32 {
    let mut result = 0;
    for pair in (AssignmentPairsInput {
        lines: fs::read_lines(filename).unwrap(),
    }) {
        if overlaps(pair) {
            result += 1;
        }
    }
    result
}

#[test]
fn test_part_2() {
    assert_eq!(part_2("./test04.txt"), 4);
}
