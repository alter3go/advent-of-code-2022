use std::{fs::File, io};

use rstest::rstest;

use crate::fs;

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
    let lines = fs::read_lines(filename).unwrap();
    let tournament = TournamentInput { lines, decryptor };
    let mut total_score = 0;
    for round in tournament {
        total_score += score_round(round);
    }
    total_score
}

pub fn part_1(filename: &str) -> u32 {
    process_tournament(filename, decrypt_my_move)
}

#[rstest]
#[case::test("./test02.txt", 15)]
#[case::input("./input02.txt", 10624)]
fn test_part_1(#[case] filename: &str, #[case] result: u32) {
    assert_eq!(part_1(filename), result);
}

pub fn part_2(filename: &str) -> u32 {
    process_tournament(filename, decrypt_my_move_differently)
}

#[rstest]
#[case::test("./test02.txt", 12)]
#[case::input("./input02.txt", 14060)]
fn test_part_2(#[case] filename: &str, #[case] result: u32) {
    assert_eq!(part_2(filename), result);
}
