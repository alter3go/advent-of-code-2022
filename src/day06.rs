use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::Read,
};

use rstest::rstest;

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

pub fn part_1(filename: &str) -> usize {
    first_marker_in_file(4, filename)
}

#[test]
fn test_part_1() {
    assert_eq!(part_1("./input06.txt"), 1140);
}

pub fn part_2(filename: &str) -> usize {
    first_marker_in_file(14, filename)
}

#[test]
fn test_part_2() {
    assert_eq!(part_2("./input06.txt"), 3495);
}
