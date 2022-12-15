use rstest::*;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
#[cfg(test)]
use std::io::Cursor;
use std::io::Read;
use std::ops::{Add, Mul};
use std::rc::Rc;
use std::str::{self, FromStr};
use std::{
    fs::File,
    io::{self, BufRead},
    ops::RangeInclusive,
    path::Path,
};

mod vec2d;

use crate::vec2d::Vec2d;

fn main() {
    println!("{}", day_01_1("./input01.txt"));
    println!("{}", day_01_2("./input01.txt"));
    println!("{}", day_02_1("./input02.txt"));
    println!("{}", day_02_2("./input02.txt"));
    println!("{}", day_03_1("./input03.txt"));
    println!("{}", day_03_2("./input03.txt"));
    println!("{}", day_04_1("./input04.txt"));
    println!("{}", day_04_2("./input04.txt"));
    println!("{}", day_05_1("./input05.txt"));
    println!("{}", day_05_2("./input05.txt"));
    println!("{}", day_06_1("./input06.txt"));
    println!("{}", day_06_2("./input06.txt"));
    println!("{}", day_07_1("./input07.txt"));
    println!("{}", day_07_2("./input07.txt"));
    println!("{}", day_08_1("./input08.txt"));
    println!("{}", day_08_2("./input08.txt"));
    println!("{}", day_09_1("./input09.txt"));
    println!("{}", day_09_2("./input09.txt"));
    println!("{}", day_10_1("./input10.txt"));
    println!("{}", day_10_2("./input10.txt"));
    println!("{}", day_11_1("./input11.txt"));
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

fn day_05_1(filename: &str) -> String {
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    let mut lines = read_lines(filename).unwrap().filter_map(|s| s.ok());
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

fn day_05_2(filename: &str) -> String {
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    let mut lines = read_lines(filename).unwrap().filter_map(|s| s.ok());
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
fn test_day_05_1() {
    assert_eq!(day_05_1("./test05.txt"), "CMZ");
}

#[test]
fn test_day_05_2() {
    assert_eq!(day_05_2("./test05.txt"), "MCD");
}

fn find_marker(distinct_count: usize, datastream: &[u8]) -> Option<usize> {
    let mut iter = datastream.into_iter();
    let mut last_n: VecDeque<&u8> = VecDeque::from_iter(iter.by_ref().take(distinct_count - 1));
    let mut unique: HashSet<&u8> = HashSet::new();
    for (byte_number, byte) in iter.enumerate().map(|(i, b)| (i + distinct_count, b)) {
        if last_n.contains(&byte) {
            last_n.push_back(byte);
            last_n.pop_front();
        } else {
            unique.extend(last_n.clone());
            if unique.len() == distinct_count - 1 {
                return Some(byte_number);
            } else {
                unique.clear();
                last_n.push_back(byte);
                last_n.pop_front();
            }
        }
    }
    None
}

#[rstest]
#[case(4, "bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes(), Some(5))]
#[case(4, "nppdvjthqldpwncqszvftbrmjlhg".as_bytes(), Some(6))]
#[case(4, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes(), Some(10))]
#[case(4, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes(), Some(11))]
#[case(4, "abcabcabcabcabcabcabcabc".as_bytes(), None)]
#[case(14, "mjqjpqmgbljsphdztnvjfqwrcgsmlb".as_bytes(), Some(19))]
#[case(14, "bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes(), Some(23))]
#[case(14, "nppdvjthqldpwncqszvftbrmjlhg".as_bytes(), Some(23))]
#[case(14, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes(), Some(29))]
#[case(14, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes(), Some(26))]
fn test_find_marker(
    #[case] distinct_count: usize,
    #[case] bytes: &[u8],
    #[case] result: Option<usize>,
) {
    assert_eq!(find_marker(distinct_count, bytes), result);
}

fn first_marker_in_file(distinct_count: usize, filename: &str) -> usize {
    let file = File::open(filename).unwrap();
    find_marker(
        distinct_count,
        &(file.bytes().filter_map(|b| b.ok()).collect::<Vec<u8>>()),
    )
    .unwrap()
}

fn day_06_1(filename: &str) -> usize {
    first_marker_in_file(4, filename)
}

fn day_06_2(filename: &str) -> usize {
    first_marker_in_file(14, filename)
}

#[derive(Default)]
struct DirectoryListing {
    files: HashMap<String, u64>,
    subdirectories: HashMap<String, Rc<RefCell<DirectoryListing>>>,
    parent: Option<Rc<RefCell<DirectoryListing>>>,
}

impl DirectoryListing {
    fn new(parent: Option<Rc<RefCell<DirectoryListing>>>) -> DirectoryListing {
        DirectoryListing {
            files: HashMap::new(),
            subdirectories: HashMap::new(),
            parent,
        }
    }

    fn size(&self) -> u64 {
        self.files.values().sum::<u64>()
            + self
                .subdirectories
                .values()
                .map(|rc| rc.borrow().size())
                .sum::<u64>()
    }
}

fn inspect_filesystem(filename: &str) -> DirectoryListing {
    let root = Rc::new(RefCell::new(DirectoryListing::new(None)));
    {
        let mut current_directory = Rc::clone(&root);
        for line in read_lines(filename).unwrap().filter_map(|s| s.ok()) {
            if line.starts_with("$ ") {
                let command: Vec<&str> = (&line[2..]).split(' ').collect();
                if command[0] == "cd" {
                    let dirname = command[1];
                    if dirname == "/" {
                        if current_directory.borrow().parent.is_some() {
                            current_directory = Rc::clone(&root);
                        }
                    } else {
                        let current_clone = Rc::clone(&current_directory);
                        if dirname == ".." {
                            current_directory =
                                Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
                        } else {
                            current_directory = Rc::clone(
                                current_clone.borrow().subdirectories.get(dirname).unwrap(),
                            );
                        }
                    }
                }
            } else if line.starts_with("dir") {
                let dirname = &line[4..];
                if !current_directory
                    .borrow()
                    .subdirectories
                    .contains_key(dirname)
                {
                    let new_directory = DirectoryListing::new(Some(Rc::clone(&current_directory)));
                    current_directory
                        .borrow_mut()
                        .subdirectories
                        .insert(dirname.to_string(), Rc::new(RefCell::new(new_directory)));
                }
            } else {
                let (size, filename) = line.split_once(' ').unwrap();
                current_directory
                    .borrow_mut()
                    .files
                    .insert(filename.to_string(), size.parse().unwrap());
            }
        }
    }
    root.take()
}

fn all_sizes(dir: &DirectoryListing) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();
    for subdir in dir.subdirectories.values() {
        result.extend(all_sizes(&subdir.borrow()));
    }
    result.push(dir.size());
    result
}

fn day_07_1(filename: &str) -> u64 {
    let root = inspect_filesystem(filename);
    all_sizes(&root)
        .into_iter()
        .filter(|size| *size <= 100_000)
        .sum()
}

#[test]
fn test_day_07_1() {
    assert_eq!(day_07_1("./test07.txt"), 95437);
}

fn day_07_2(filename: &str) -> u64 {
    let root = inspect_filesystem(filename);
    let unused_space = 70_000_000 - root.size();
    let needed_space: u64 = 30_000_000;
    let min_space_to_free = needed_space - unused_space;
    all_sizes(&root)
        .into_iter()
        .filter(|size| *size >= min_space_to_free)
        .min()
        .unwrap()
}

#[test]
fn test_day_07_2() {
    assert_eq!(day_07_2("./test07.txt"), 24933642);
}

fn forest_from_file<T>(filename: &str) -> Vec2d<T>
where
    T: Copy + FromStr,
    T::Err: Debug,
{
    let forest_vec: Vec<T>;
    let mut forest_input: Vec<Vec<u8>> = Vec::new();
    let mut input = io::BufReader::new(File::open(filename).unwrap());
    loop {
        let mut line = Vec::new();
        match input.read_until('\n' as u8, &mut line) {
            Err(_) => break,
            _ => {
                if line.len() < 1 {
                    break;
                } else if line.last() == Some(&('\n' as u8)) {
                    line.pop();
                }
            }
        }
        forest_input.push(line);
    }
    let (width, height) = (forest_input.get(0).unwrap().len(), forest_input.len());
    forest_vec = forest_input
        .into_iter()
        .flatten()
        .map(|u| str::from_utf8(&[u]).unwrap().parse::<T>().unwrap())
        .collect();
    Vec2d::new(forest_vec, width, height)
}

fn day_08_1(filename: &str) -> u32 {
    let forest = forest_from_file(filename);
    let from_left = Vec2d::new(
        (0..forest.row_count)
            .map(|i| find_min_heights(&Vec::from(forest.row(i))))
            .flatten()
            .collect(),
        forest.row_count,
        forest.col_count,
    );
    let from_right = Vec2d::new(
        (0..forest.row_count)
            .map(|i| {
                let reversed_row = forest.row(i).iter().rev().copied().collect();
                let mut min_heights = find_min_heights(&reversed_row);
                min_heights.reverse();
                min_heights
            })
            .flatten()
            .collect(),
        forest.row_count,
        forest.col_count,
    );
    let mut from_top = forest.clone();
    for j in 0..forest.col_count {
        let col_min_heights = find_min_heights(&forest.col(j));
        for i in 0..forest.row_count {
            *from_top.index_mut(i, j) = col_min_heights[i];
        }
    }
    let mut from_bottom = forest.clone();
    for j in 0..forest.col_count {
        let col_min_heights: Vec<i8> = find_min_heights(&forest.col(j).into_iter().rev().collect())
            .into_iter()
            .rev()
            .collect();
        for i in 0..forest.row_count {
            *from_bottom.index_mut(i, j) = col_min_heights[i];
        }
    }
    let mut visible_count = 0;
    for i in 0..forest.row_count {
        for j in 0..forest.col_count {
            let height = forest.index(i, j);
            if height > from_left.index(i, j)
                || height > from_right.index(i, j)
                || height > from_top.index(i, j)
                || height > from_bottom.index(i, j)
            {
                visible_count += 1;
            }
        }
    }
    visible_count
}

#[test]
fn test_day_08_1() {
    assert_eq!(day_08_1("./test08.txt"), 21);
}

#[rstest]
#[case(vec![2,5,5,1,2], vec![-1,2,5,5,5])]
fn test_find_min_heights(#[case] row: Vec<i8>, #[case] result: Vec<i8>) {
    assert_eq!(find_min_heights(&row), result);
}

fn find_min_heights(row: &Vec<i8>) -> Vec<i8> {
    let mut max = -1;
    row.iter()
        .map(|s| {
            let cur = max;
            max = max.max(*s);
            cur
        })
        .collect()
}

fn find_directional_scores(row: &Vec<u32>) -> Vec<u32> {
    let mut scores = row.clone();
    scores[0] = 0;
    for i in 1..row.len() {
        scores[i] = 1;
        let mut j = i - 1;
        while j > 0 && row[j] < row[i] {
            scores[i] += scores[j];
            j -= scores[j] as usize;
        }
    }
    scores
}

#[rstest]
#[case(vec![3, 5, 3, 9, 0], vec![0, 1, 1, 3, 1])]
fn test_find_directional_scores(#[case] row: Vec<u32>, #[case] result: Vec<u32>) {
    assert_eq!(find_directional_scores(&row), result);
}

fn day_08_2(filename: &str) -> u32 {
    let forest: Vec2d<u32> = forest_from_file(filename);
    let from_left = Vec2d::new(
        (0..forest.row_count)
            .map(|i| find_directional_scores(&Vec::from(forest.row(i))))
            .flatten()
            .collect(),
        forest.row_count,
        forest.col_count,
    );
    let from_right = Vec2d::new(
        (0..forest.row_count)
            .map(|i| {
                let reversed_row = forest.row(i).iter().rev().copied().collect();
                let mut scores = find_directional_scores(&reversed_row);
                scores.reverse();
                scores
            })
            .flatten()
            .collect(),
        forest.row_count,
        forest.col_count,
    );
    let mut from_top = forest.clone();
    for j in 0..forest.col_count {
        let col_scores = find_directional_scores(&forest.col(j));
        for i in 0..forest.row_count {
            *from_top.index_mut(i, j) = col_scores[i];
        }
    }
    let mut from_bottom = forest.clone();
    for j in 0..forest.col_count {
        let col_scores: Vec<_> =
            find_directional_scores(&forest.col(j).into_iter().rev().collect())
                .into_iter()
                .rev()
                .collect();
        for i in 0..forest.row_count {
            *from_bottom.index_mut(i, j) = col_scores[i];
        }
    }
    let mut best_score = 0;
    for i in 0..forest.row_count {
        for j in 0..forest.col_count {
            let scenic_score = from_left.index(i, j)
                * from_right.index(i, j)
                * from_top.index(i, j)
                * from_bottom.index(i, j);
            if scenic_score > best_score {
                best_score = scenic_score;
            }
        }
    }
    best_score
}

#[test]
fn test_day_08_2() {
    assert_eq!(day_08_2("./test08.txt"), 8);
}

type Coord = (i32, i32);

fn move_rope<F>(rope: &mut Vec<Coord>, head_move: Coord, mut after_step: F)
where
    F: FnMut(&Vec<Coord>) -> (),
{
    assert!(head_move.0 == 0 || head_move.1 == 0);
    let head_steps = (head_move.0 + head_move.1).abs();
    for _ in 0..head_steps {
        rope[0] = (
            rope[0].0 + head_move.0 / head_steps,
            rope[0].1 + head_move.1 / head_steps,
        );
        for i in 1..rope.len() {
            let knot = rope[i - 1];
            let following = rope[i];
            let distance = ((knot.0 - following.0).pow(2) as f32
                + (knot.1 - following.1).pow(2) as f32)
                .sqrt();
            if distance < 1.5 {
                continue;
            } else {
                let x_incr = match following.0.cmp(&knot.0) {
                    Ordering::Less => 1,
                    Ordering::Equal => 0,
                    Ordering::Greater => -1,
                };
                let y_incr = match following.1.cmp(&knot.1) {
                    Ordering::Less => 1,
                    Ordering::Equal => 0,
                    Ordering::Greater => -1,
                };
                rope[i] = (following.0 + x_incr, following.1 + y_incr);
            }
        }
        after_step(rope);
    }
}

#[rstest]
#[case::on_top_2(vec![(4, 5), (5, 5)], (1, 0), vec![(5, 5), (5, 5)])]
#[case::one_left_2(vec![(5, 5), (5, 5)], (-1, 0), vec![(4, 5), (5, 5)])]
#[case::one_right_2(vec![(5, 5), (5, 5)], (1, 0), vec![(6, 5), (5, 5)])]
#[case::one_up_2(vec![(5, 5), (5, 5)], (0, 1), vec![(5, 6), (5, 5)])]
#[case::one_down_2(vec![(5, 5), (5, 5)], (0, -1), vec![(5, 4), (5, 5)])]
#[case::ne_2(vec![(5, 6), (5, 5)], (1, 0), vec![(6, 6), (5, 5)])]
#[case::se_2(vec![(4, 4), (5, 5)], (2, 0), vec![(6, 4), (5, 5)])]
#[case::sw_2(vec![(5, 4), (5, 5)], (-1, 0), vec![(4, 4), (5, 5)])]
#[case::nw_2(vec![(5, 6), (5, 5)], (-1, 0), vec![(4, 6), (5, 5)])]
#[case::nne_2(vec![(6, 6), (5, 5)], (0, 1), vec![(6, 7), (6, 6)])]
#[case::sse_2(vec![(6, 6), (5, 5)], (0, -3), vec![(6, 3), (6, 4)])]
#[case::ssw_2(vec![(4, 5), (5, 5)], (0, -2), vec![(4, 3), (4, 4)])]
#[case::nnw_2(vec![(4, 4), (5, 5)], (0, 3), vec![(4, 7), (4, 6)])]
#[case::ene_2(vec![(5, 6), (5, 5)], (2, 0), vec![(7, 6), (6, 6)])]
#[case::ese_2(vec![(5, 4), (5, 5)], (2, 0), vec![(7, 4), (6, 4)])]
#[case::wsw_2(vec![(3, 3), (4, 4)], (0, 1), vec![(3, 4), (4, 4)])]
#[case::wnw_2(vec![(5, 6), (5, 5)], (-2, 0), vec![(3, 6), (4, 6)])]
#[case::line_3(vec![(-1, 0), (-1, 0), (-1, 0)], (6, 0), vec![(5, 0), (4, 0), (3, 0)])]
fn test_move_rope(#[case] mut rope: Vec<Coord>, #[case] m: Coord, #[case] result: Vec<Coord>) {
    move_rope(&mut rope, m, |_| {});
    assert_eq!(rope, result);
}

#[test]
fn test_move_rope_callback() {
    let mut rope = vec![(0, 0), (0, 0)];
    let mut call_count = 0;
    move_rope(&mut rope, (5, 0), |_| {
        call_count += 1;
    });
    assert_eq!(call_count, 5);
}

fn moves_from_file(filename: &str) -> Vec<Coord> {
    read_lines(filename)
        .unwrap()
        .into_iter()
        .filter_map(|r| r.ok())
        .map(|line| {
            let (dir, step) = line.split_once(' ').unwrap();
            match dir {
                "R" => (step.parse::<i32>().unwrap(), 0),
                "L" => (-step.parse::<i32>().unwrap(), 0),
                "U" => (0, step.parse::<i32>().unwrap()),
                "D" => (0, -step.parse::<i32>().unwrap()),
                _ => panic!("invalid move direction"),
            }
        })
        .collect()
}

#[test]
fn test_moves_from_file() {
    assert_eq!(
        moves_from_file("./test09.txt"),
        [
            (4, 0),
            (0, 4),
            (-3, 0),
            (0, -1),
            (4, 0),
            (0, -1),
            (-5, 0),
            (2, 0),
        ]
    );
}

fn day_09(filename: &str, mut rope: Vec<Coord>) -> usize {
    let mut tail_visits: HashSet<Coord> = HashSet::new();
    let last_knot_idx = rope.len() - 1;
    for m in moves_from_file(filename) {
        move_rope(&mut rope, m, |rope| {
            tail_visits.insert(rope[last_knot_idx]);
        })
    }
    tail_visits.len()
}

fn day_09_1(filename: &str) -> usize {
    day_09(filename, vec![(0, 0), (0, 0)])
}

fn day_09_2(filename: &str) -> usize {
    day_09(filename, vec![(0, 0); 10])
}

#[test]
fn test_day_09_1() {
    assert_eq!(day_09_1("./test09.txt"), 13);
}

#[rstest]
#[case("./test09.txt", 1)]
#[case("./test09-2.txt", 36)]
fn test_day_09_2(#[case] filename: &str, #[case] result: usize) {
    assert_eq!(day_09_2(filename), result);
}

type Instruction = Option<i32>;

fn run_program(program: Vec<Instruction>) -> Vec<i32> {
    let mut x = 1;
    [1].into_iter()
        .chain(
            program
                .into_iter()
                .map(|instruction| match instruction {
                    Some(a) => {
                        vec![0, a]
                    }
                    None => vec![0],
                })
                .flatten()
                .map(|a| {
                    x += a;
                    x
                }),
        )
        .collect()
}

#[rstest]
#[case(vec![None, Some(3), Some(-5)], vec![1, 1, 1, 4, 4, -1])]
fn test_run_program(#[case] program: Vec<Instruction>, #[case] cycle_x_values: Vec<i32>) {
    assert_eq!(run_program(program), cycle_x_values);
}

fn program_from_file(filename: &str) -> Vec<Instruction> {
    read_lines(filename)
        .unwrap()
        .into_iter()
        .filter_map(|r| r.ok())
        .map(|line| {
            if line == "noop" {
                None
            } else {
                if !line.starts_with("addx ") {
                    panic!("Unknown instruction {}", line);
                }
                Some(line[5..].parse::<i32>().unwrap())
            }
        })
        .collect()
}

#[test]
fn test_program_from_file() {
    assert_eq!(
        program_from_file("./test10.txt"),
        [None, Some(3), Some(-5),]
    );
}

fn day_10_1(filename: &str) -> i32 {
    let x_values = run_program(program_from_file(filename));
    [20, 60, 100, 140, 180, 220]
        .into_iter()
        .fold(0, |acc, cycle| acc + cycle * x_values[cycle as usize - 1])
}

#[test]
fn test_day_10_1() {
    assert_eq!(day_10_1("./test10-2.txt"), 13140);
}

fn day_10_2(filename: &str) -> String {
    let sprite_positions = run_program(program_from_file(filename));
    let mut output = String::new();
    for row in 0..6 {
        for col in 0..40 {
            let pixel = 40 * row + col;
            let sprite_pos = sprite_positions[pixel as usize];
            if sprite_pos - 1 <= col && col <= sprite_pos + 1 {
                output += "#";
            } else {
                output += ".";
            }
        }
        output += "\n";
    }
    output
}

#[test]
fn test_day_10_2() {
    assert_eq!(
        day_10_2("./test10-2.txt"),
        "##..##..##..##..##..##..##..##..##..##..\n\
         ###...###...###...###...###...###...###.\n\
         ####....####....####....####....####....\n\
         #####.....#####.....#####.....#####.....\n\
         ######......######......######......####\n\
         #######.......#######.......#######.....\n"
    );
}

#[derive(Debug, PartialEq)]
struct Monkey {
    items: VecDeque<u32>,
    operator: fn(u32, u32) -> u32,
    operand: Option<u32>,
    test: u32,
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
                "*" => u32::mul,
                "+" => u32::add,
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
                operator: u32::mul,
                operand: Some(19),
                test: 23,
                if_true: 2,
                if_false: 3
            },
            Monkey {
                items: VecDeque::from([54, 65, 75, 74]),
                operator: u32::add,
                operand: Some(6),
                test: 19,
                if_true: 2,
                if_false: 0
            },
            Monkey {
                items: VecDeque::from([79, 60, 97]),
                operator: u32::mul,
                operand: None,
                test: 13,
                if_true: 1,
                if_false: 3
            },
            Monkey {
                items: VecDeque::from([74]),
                operator: u32::add,
                operand: Some(3),
                test: 17,
                if_true: 0,
                if_false: 1
            },
        ])
    );
}

fn do_monkey_business(mut monkeys: VecDeque<Monkey>, rounds: usize, relief: u32) -> u32 {
    let num_monkeys = monkeys.len();
    let mut inspections = vec![0; num_monkeys];

    for _ in 0..rounds {
        for i in 0..num_monkeys {
            let mut monkey = monkeys.pop_front().unwrap();
            for _ in 0..monkey.items.len() {
                let mut item = monkey.items.pop_front().unwrap();
                inspections[i] += 1;
                item = (monkey.operator)(
                    item,
                    match monkey.operand {
                        Some(n) => n,
                        None => item,
                    },
                ) / relief;
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
    inspections.into_iter().rev().take(2).fold(1, u32::mul)
}

fn day_11_1(filename: &str) -> u32 {
    do_monkey_business(monkeys_from_file(filename), 20, 3)
}

#[test]
fn test_day_11_1() {
    assert_eq!(day_11_1("./test11.txt"), 10605);
}

fn day_11_2(filename: &str) -> u32 {
    do_monkey_business(monkeys_from_file(filename), 10_000, 1)
}

#[test]
fn test_day_11_2() {
    assert_eq!(day_11_2("./test11.txt"), 2713310158);
}
