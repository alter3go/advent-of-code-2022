use std::fmt::Debug;
use std::str::{from_utf8, FromStr};

use rstest::rstest;

use crate::vec2d::{self, Vec2d};

fn forest_from_file<T>(filename: &str) -> Vec2d<T>
where
    T: Copy + FromStr,
    T::Err: Debug,
{
    let forest_vec: Vec<T>;
    let forest_input = vec2d::input_from_file(filename);
    let (width, height) = (forest_input.get(0).unwrap().len(), forest_input.len());
    forest_vec = forest_input
        .into_iter()
        .flatten()
        .map(|u| from_utf8(&[u]).unwrap().parse::<T>().unwrap())
        .collect();
    Vec2d::new(forest_vec, height, width)
}

pub fn part_1(filename: &str) -> u32 {
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

#[rstest]
#[case::test("./test08.txt", 21)]
#[case::input("./input08.txt", 1835)]
fn test_part_1(#[case] filename: &str, #[case] result: u32) {
    assert_eq!(part_1(filename), result);
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

pub fn part_2(filename: &str) -> u32 {
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

#[rstest]
#[case::test("./test08.txt", 8)]
#[case::input("./input08.txt", 263670)]
fn test_part_2(#[case] filename: &str, #[case] result: u32) {
    assert_eq!(part_2(filename), result);
}
