use rstest::rstest;

use crate::fs;

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

#[test]
fn test_run_program() {
    assert_eq!(
        run_program(vec![None, Some(3), Some(-5)]),
        vec![1, 1, 1, 4, 4, -1]
    );
}

fn program_from_file(filename: &str) -> Vec<Instruction> {
    fs::read_lines(filename)
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

pub fn part_1(filename: &str) -> i32 {
    let x_values = run_program(program_from_file(filename));
    [20, 60, 100, 140, 180, 220]
        .into_iter()
        .fold(0, |acc, cycle| acc + cycle * x_values[cycle as usize - 1])
}

#[rstest]
#[case::test("./test10-2.txt", 13140)]
#[case::input("./input10.txt", 13920)]
fn test_part_1(#[case] filename: &str, #[case] result: i32) {
    assert_eq!(part_1(filename), result);
}

pub fn part_2(filename: &str) -> String {
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

#[rstest]
#[case::test(
    "./test10-2.txt",
    "##..##..##..##..##..##..##..##..##..##..\n\
     ###...###...###...###...###...###...###.\n\
     ####....####....####....####....####....\n\
     #####.....#####.....#####.....#####.....\n\
     ######......######......######......####\n\
     #######.......#######.......#######.....\n"
)]
#[case::input(
    "./input10.txt",
    "####..##..#....#..#.###..#....####...##.\n\
     #....#..#.#....#..#.#..#.#....#.......#.\n\
     ###..#....#....####.###..#....###.....#.\n\
     #....#.##.#....#..#.#..#.#....#.......#.\n\
     #....#..#.#....#..#.#..#.#....#....#..#.\n\
     ####..###.####.#..#.###..####.#.....##..\n"
)]
fn test_part_2(#[case] filename: &str, #[case] result: String) {
    assert_eq!(part_2(filename), result);
}
