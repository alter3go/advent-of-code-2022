use rstest::*;
use std::collections::HashSet;
use std::{
    fs::File,
    io::{self, BufRead, Cursor},
    ops::RangeInclusive,
    path::Path,
};

fn main() {
    println!("{}", day_01_1("./input01.txt"));
    println!("{}", day_01_2("./input01.txt"));
    println!("{}", day_02_1("./input02.txt"));
    println!("{}", day_02_2("./input02.txt"));
    println!("{}", day_03_1("./input03.txt"));
    println!("{}", day_03_2("./input03.txt"));
    println!("{}", day_04_1("./input04.txt"));
    println!("{}", day_04_2("./input04.txt"));
}

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
    let lines = read_lines(filename)?;
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

fn day_01_1(filename: &str) -> u32 {
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
fn test_day01_1() {
    assert_eq!(day_01_1("./test01.txt"), 24000);
}

fn day_01_2(filename: &str) -> u32 {
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
fn test_day01_2() {
    assert_eq!(day_01_2("./test01.txt"), 45000);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

fn decrypt_their_move(a: char) -> Result<Move, &'static str> {
    match a {
        'A' => Ok(Move::Rock),
        'B' => Ok(Move::Paper),
        'C' => Ok(Move::Scissors),
        _ => Err("Invalid opponent move"),
    }
}

fn decrypt_my_move(a: char, _: &Move) -> Result<Move, &'static str> {
    match a {
        'X' => Ok(Move::Rock),
        'Y' => Ok(Move::Paper),
        'Z' => Ok(Move::Scissors),
        _ => Err("Invalid player move"),
    }
}

fn decrypt_my_move_differently(a: char, them: &Move) -> Result<Move, &'static str> {
    match a {
        'X' => match them {
            Move::Rock => Ok(Move::Scissors),
            Move::Paper => Ok(Move::Rock),
            Move::Scissors => Ok(Move::Paper),
        },
        'Y' => Ok(*them),
        'Z' => match them {
            Move::Rock => Ok(Move::Paper),
            Move::Paper => Ok(Move::Scissors),
            Move::Scissors => Ok(Move::Rock),
        },
        _ => Err("Invalid player move"),
    }
}

struct Round {
    me: Move,
    them: Move,
}

struct TournamentInput {
    lines: io::Lines<io::BufReader<File>>,
    decryptor: MyMoveDecryptor,
}

type MyMoveDecryptor = fn(char, &Move) -> Result<Move, &'static str>;

impl Iterator for TournamentInput {
    type Item = Round;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lines.next()? {
            Ok(round_string) => {
                if round_string.len() != 3 {
                    return None;
                }
                let mut chars = round_string.chars();
                let their_encrypted = chars.next().unwrap();
                let them = decrypt_their_move(their_encrypted).unwrap();
                let space = chars.next().unwrap();
                assert_eq!(space, ' ');
                let my_encrypted = chars.next().unwrap();
                let me = (self.decryptor)(my_encrypted, &them).unwrap();
                Some(Round { me, them })
            }
            Err(e) => panic!("{}", e),
        }
    }
}

fn score_round(round: Round) -> u32 {
    let move_points = match round.me {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    };
    let win_points = match (round.me, round.them) {
        (Move::Rock, Move::Paper) => 0,
        (Move::Paper, Move::Scissors) => 0,
        (Move::Scissors, Move::Rock) => 0,

        (Move::Rock, Move::Rock) => 3,
        (Move::Paper, Move::Paper) => 3,
        (Move::Scissors, Move::Scissors) => 3,

        (Move::Rock, Move::Scissors) => 6,
        (Move::Paper, Move::Rock) => 6,
        (Move::Scissors, Move::Paper) => 6,
    };
    move_points + win_points
}

#[rstest]
#[case::rock_loss(Round{me: Move::Rock, them: Move::Paper}, 1)]
#[case::paper_tie(Round{me: Move::Paper, them: Move::Paper}, 5)]
#[case::scissors_win(Round{me: Move::Scissors, them: Move::Paper}, 9)]
fn test_score_round(#[case] round: Round, #[case] score: u32) {
    assert_eq!(score_round(round), score);
}

fn process_tournament(filename: &str, decryptor: MyMoveDecryptor) -> u32 {
    let lines = read_lines(filename).unwrap();
    let tournament = TournamentInput { lines, decryptor };
    let mut total_score = 0;
    for round in tournament {
        total_score += score_round(round);
    }
    total_score
}

fn day_02_1(filename: &str) -> u32 {
    process_tournament(filename, decrypt_my_move)
}

#[test]
fn test_day_02_1() {
    assert_eq!(day_02_1("./test02.txt"), 15);
}

fn day_02_2(filename: &str) -> u32 {
    process_tournament(filename, decrypt_my_move_differently)
}

#[test]
fn test_day_02_2() {
    assert_eq!(day_02_2("./test02.txt"), 12);
}

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

fn day_03_1(filename: &str) -> u32 {
    let lines = read_lines(filename).unwrap();
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
fn test_day_03_1() {
    assert_eq!(day_03_1("./test03.txt"), 157);
}

fn day_03_2(filename: &str) -> u32 {
    let lines = read_lines(filename).unwrap();
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
fn test_day_03_2() {
    assert_eq!(day_03_2("./test03.txt"), 70);
}

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
    let lines = Cursor::new(lines_str).lines();
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

fn day_04_1(filename: &str) -> u32 {
    let mut result = 0;
    for pair in (AssignmentPairsInput {
        lines: read_lines(filename).unwrap(),
    }) {
        if one_contains_other(pair) {
            result += 1;
        }
    }
    result
}

#[test]
fn test_day_04_1() {
    assert_eq!(day_04_1("./test04.txt"), 2);
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

fn day_04_2(filename: &str) -> u32 {
    let mut result = 0;
    for pair in (AssignmentPairsInput {
        lines: read_lines(filename).unwrap(),
    }) {
        if overlaps(pair) {
            result += 1;
        }
    }
    result
}

#[test]
fn test_day_04_2() {
    assert_eq!(day_04_2("./test04.txt"), 4);
}
