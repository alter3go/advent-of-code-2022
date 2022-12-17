use std::env;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod fs;
mod vec2d;

fn main() {
    let args: Vec<String> = env::args().collect();
    match (&args[1][..], &args[2][..]) {
        ("1", "1") => println!("{}", day01::part_1("./input01.txt")),
        ("1", "2") => println!("{}", day01::part_2("./input01.txt")),

        ("2", "1") => println!("{}", day02::part_1("./input02.txt")),
        ("2", "2") => println!("{}", day02::part_2("./input02.txt")),

        ("3", "1") => println!("{}", day03::part_1("./input03.txt")),
        ("3", "2") => println!("{}", day03::part_2("./input03.txt")),

        ("4", "1") => println!("{}", day04::part_1("./input04.txt")),
        ("4", "2") => println!("{}", day04::part_2("./input04.txt")),

        ("5", "1") => println!("{}", day05::part_1("./input05.txt")),
        ("5", "2") => println!("{}", day05::part_2("./input05.txt")),

        ("6", "1") => println!("{}", day06::part_1("./input06.txt")),
        ("6", "2") => println!("{}", day06::part_2("./input06.txt")),

        ("7", "1") => println!("{}", day07::part_1("./input07.txt")),
        ("7", "2") => println!("{}", day07::part_2("./input07.txt")),

        ("8", "1") => println!("{}", day08::part_1("./input08.txt")),
        ("8", "2") => println!("{}", day08::part_2("./input08.txt")),

        ("9", "1") => println!("{}", day09::part_1("./input09.txt")),
        ("9", "2") => println!("{}", day09::part_2("./input09.txt")),

        ("10", "1") => println!("{}", day10::part_1("./input10.txt")),
        ("10", "2") => println!("{}", day10::part_2("./input10.txt")),

        ("11", "1") => println!("{}", day11::part_1("./input11.txt")),
        ("11", "2") => println!("{}", day11::part_2("./input11.txt")),

        ("12", "1") => println!("{}", day12::day_12_1("./input12.txt")),
        ("12", "2") => println!("{}", day12::day_12_2("./input12.txt")),

        ("13", "1") => println!("{}", day13::day_13_1("./input13.txt")),

        _ => panic!("Unimplemented puzzle"),
    }
}
