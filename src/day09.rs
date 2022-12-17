use std::{cmp::Ordering, collections::HashSet};

use rstest::rstest;

use crate::fs;

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
    fs::read_lines(filename)
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

pub fn part_1(filename: &str) -> usize {
    day_09(filename, vec![(0, 0), (0, 0)])
}

#[rstest]
#[case::test("./test09.txt", 13)]
#[case::input("./input09.txt", 5735)]
fn test_part_1(#[case] filename: &str, #[case] result: usize) {
    assert_eq!(part_1(filename), result);
}

pub fn part_2(filename: &str) -> usize {
    day_09(filename, vec![(0, 0); 10])
}

#[rstest]
#[case::test1("./test09.txt", 1)]
#[case::test2("./test09-2.txt", 36)]
#[case::input("./input09.txt", 2478)]
fn test_part_2(#[case] filename: &str, #[case] result: usize) {
    assert_eq!(part_2(filename), result);
}
