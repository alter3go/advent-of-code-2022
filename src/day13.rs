use rstest::rstest;
use serde_json::{json, Value};
use std::cmp::Ordering;

use crate::fs;

#[derive(Clone, Eq)]
struct Packet {
    value: Value,
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        compare_packets(&self.value, &other.value) == Ordering::Equal
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(compare_packets(&self.value, &other.value))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        compare_packets(&self.value, &other.value)
    }
}

fn compare_packets(p1: &Value, p2: &Value) -> Ordering {
    if p1.is_i64() && p2.is_i64() {
        let p1i = p1.as_i64().unwrap();
        let p2i = p2.as_i64().unwrap();
        p1i.cmp(&p2i)
    } else if p1.is_array() && p2.is_array() {
        let p1a = p1.as_array().unwrap();
        let p2a = p2.as_array().unwrap();
        for (i, value) in p1a.into_iter().enumerate() {
            if p2a.len() <= i {
                return Ordering::Greater;
            }
            let cmp = compare_packets(value, &p2a[i]);
            if cmp != Ordering::Equal {
                return cmp;
            }
        }
        p1a.len().cmp(&p2a.len())
    } else if p1.is_array() && p2.is_i64() {
        compare_packets(p1, &json!([p2]))
    } else if p2.is_array() && p1.is_i64() {
        compare_packets(&json!([p1]), p2)
    } else {
        panic!("Expected each value to be array or integer");
    }
}

#[rstest]
#[case(json!([1,1,3,1,1]), json!([1,1,5,1,1]), Ordering::Less)]
#[case(json!([[1],[2,3,4]]), json!([[1],4]), Ordering::Less)]
#[case(json!([9]), json!([[8,7,6]]), Ordering::Greater)]
#[case(json!([[4,4],4,4]), json!([[4,4],4,4,4]), Ordering::Less)]
#[case(json!([7,7,7,7]), json!([7,7,7]), Ordering::Greater)]
#[case(json!([]), json!([3]), Ordering::Less)]
#[case(json!([[[]]]), json!([[]]), Ordering::Greater)]
#[case(json!([1,[2,[3,[4,[5,6,7]]]],8,9]), json!([1,[2,[3,[4,[5,6,0]]]],8,9]), Ordering::Greater)]
fn test_compare_packets(#[case] p1: Value, #[case] p2: Value, #[case] result: Ordering) {
    assert_eq!(compare_packets(&p1, &p2), result);
}

pub fn part_1(filename: &str) -> usize {
    let mut result = 0;
    let mut i = 1;
    let mut lines = fs::read_lines(filename).unwrap().filter_map(|r| r.ok());
    loop {
        let pair: Vec<Value> = lines
            .by_ref()
            .take(3)
            .filter(|s| s.len() > 0)
            .map(|s| serde_json::from_str::<Value>(&s).unwrap())
            .inspect(|j| assert!(j.is_array()))
            .collect();
        if pair.len() < 2 {
            break;
        }
        match compare_packets(&pair[0], &pair[1]) {
            Ordering::Greater => {}
            _ => {
                result += i;
            }
        }
        i += 1;
    }
    result
}

#[rstest]
#[case::test("./test13.txt", 13)]
#[case::input("./input13.txt", 5185)]
fn test_part_1(#[case] filename: &str, #[case] result: usize) {
    assert_eq!(part_1(filename), result);
}

pub fn part_2(filename: &str) -> usize {
    let divider_packets = [
        Packet {
            value: json!([[2]]),
        },
        Packet {
            value: json!([[6]]),
        },
    ];
    let mut packets: Vec<Packet> = fs::read_lines(filename)
        .unwrap()
        .filter_map(|r| r.ok())
        .filter(|s| s.len() > 0)
        .map(|s| Packet {
            value: serde_json::from_str::<Value>(&s).unwrap(),
        })
        .chain(divider_packets.iter().cloned())
        .collect();

    packets.sort();
    packets
        .into_iter()
        .enumerate()
        .filter_map(|(i, p)| {
            if p == divider_packets[0] || p == divider_packets[1] {
                Some(i)
            } else {
                None
            }
        })
        .map(|i| i + 1)
        .product()
}

#[rstest]
#[case::test("./test13.txt", 140)]
#[case::input("./input13.txt", 23751)]
fn test_part_2(#[case] filename: &str, #[case] result: usize) {
    assert_eq!(part_2(filename), result);
}
