use rstest::rstest;

use crate::fs;
use crate::vec2d::Vec2d;

#[derive(PartialEq)]
struct Cave {
    map: Vec2d<bool>,
    x_min: usize,
    y_min: usize,
    x_max: usize,
    y_max: usize,
}

impl std::fmt::Debug for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::from("map: \n");
        for i in 0..self.map.row_count {
            str.push_str(
                self.map
                    .row(i)
                    .iter()
                    .map(|b| match b {
                        true => '#',
                        false => '.',
                    })
                    .collect::<String>()
                    .as_str(),
            );
            str.push_str("\n");
        }
        str.push_str(&format!(", x: {}-{}", self.x_min, self.x_max));
        str.push_str(&format!(", y: {}-{}", self.y_min, self.y_max));
        write!(f, "Cave {{ {} }}", str)
    }
}

type Coord = (usize, usize);

fn cave_from_file(filename: &str, include_floor: bool) -> Cave {
    let (mut x_range, mut y_range) = (usize::MAX..=0, usize::MAX..=0);
    let rocks: Vec<Vec<Coord>> = fs::read_lines(filename)
        .unwrap()
        .filter_map(|r| r.ok())
        .chain(["500,0".to_string()])
        .map(|l| {
            l.split(" -> ")
                .map(|s| {
                    let (x_str, y_str) = s.split_once(',').unwrap();
                    let (x, y) = (x_str.parse().unwrap(), y_str.parse().unwrap());
                    x_range = *x_range.start().min(&x)..=*x_range.end().max(&x);
                    y_range = *y_range.start().min(&y)..=*y_range.end().max(&y);
                    (x, y)
                })
                .collect()
        })
        .collect();
    if include_floor {
        y_range = *y_range.start()..=*y_range.end() + 2;
        let y_count = y_range.clone().count();
        x_range = *x_range.start().min(&(500 - y_count))..=*x_range.end().max(&(500 + y_count));
    }
    let (row_count, col_count) = (y_range.clone().count(), x_range.clone().count());
    let mut vec = vec![false; row_count * col_count];
    if include_floor {
        for i in (vec.len() - col_count)..vec.len() {
            vec[i] = true;
        }
    }
    let map = Vec2d::new(vec, row_count, col_count);
    let mut cave = Cave {
        map,
        x_min: *x_range.start(),
        y_min: *y_range.start(),
        x_max: *x_range.end(),
        y_max: *y_range.end(),
    };
    for segments in rocks {
        let mut startpoint = segments[0];
        for endpoint in segments.into_iter() {
            let x_range = if startpoint.0 <= endpoint.0 {
                startpoint.0..=endpoint.0
            } else {
                endpoint.0..=startpoint.0
            };
            let y_range = if startpoint.1 <= endpoint.1 {
                startpoint.1..=endpoint.1
            } else {
                endpoint.1..=startpoint.1
            };
            for x in x_range {
                for y in y_range.clone() {
                    *cave.map.index_mut(y - cave.y_min, x - cave.x_min) = true;
                }
            }
            startpoint = endpoint;
        }
    }
    cave
}

#[test]
fn test_cave_from_file() {
    assert_eq!(
        format!("{:?}", cave_from_file("./test14.txt", false)),
        "Cave { map: \n\
         ......#...\n\
         ..........\n\
         ..........\n\
         ..........\n\
         ....#...##\n\
         ....#...#.\n\
         ..###...#.\n\
         ........#.\n\
         ........#.\n\
         #########.\n\
         , x: 494-503, y: 0-9 }"
    );
}

#[test]
fn test_cave_from_file_with_floor() {
    assert_eq!(
        format!("{:?}", cave_from_file("./test14.txt", true)),
        "Cave { map: \n\
         ............#............\n\
         .........................\n\
         .........................\n\
         .........................\n\
         ..........#...##.........\n\
         ..........#...#..........\n\
         ........###...#..........\n\
         ..............#..........\n\
         ..............#..........\n\
         ......#########..........\n\
         .........................\n\
         #########################\n\
         , x: 488-512, y: 0-11 }"
    );
}

enum SandState {
    Freefall,
    Obstructed,
    OffEdge,
}

fn sand_state(y: usize, x: usize, cave: &Cave) -> SandState {
    if y > cave.y_max || x > cave.x_max || y < cave.y_min || x < cave.x_min {
        SandState::OffEdge
    } else if *cave.map.index(y - cave.y_min, x - cave.x_min) {
        SandState::Obstructed
    } else {
        SandState::Freefall
    }
}

fn fall_until(mut y: usize, mut x: usize, cave: &Cave) -> Option<Coord> {
    loop {
        y += 1;
        match sand_state(y, x, &cave) {
            SandState::Freefall => continue,
            SandState::OffEdge => return None,
            SandState::Obstructed => {
                x -= 1;
                match sand_state(y, x, &cave) {
                    SandState::Freefall => continue,
                    SandState::OffEdge => return None,
                    SandState::Obstructed => {
                        x += 2;
                        match sand_state(y, x, &cave) {
                            SandState::Freefall => continue,
                            SandState::OffEdge => return None,
                            SandState::Obstructed => {
                                return Some((x - 1, y - 1));
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn part_1(filename: &str) -> usize {
    let mut cave = cave_from_file(filename, false);
    for i in 0.. {
        let (x, y) = (500, 0);
        match fall_until(y, x, &cave) {
            Some((x, y)) => {
                *cave.map.index_mut(y - cave.y_min, x - cave.x_min) = true;
            }
            None => {
                return i;
            }
        }
    }
    panic!("Unexpectedly broke loop");
}

#[rstest]
#[case::test("./test14.txt", 24)]
#[case::test("./input14.txt", 610)]
fn test_part_1(#[case] filename: &str, #[case] result: usize) {
    assert_eq!(part_1(filename), result);
}

pub fn part_2(filename: &str) -> usize {
    let mut cave = cave_from_file(filename, true);
    for i in 0.. {
        let (x, y) = (500, 0);
        match fall_until(y, x, &cave) {
            Some((x, y)) => {
                if (x, y) == (500, 0) {
                    return i + 1;
                }
                *cave.map.index_mut(y - cave.y_min, x - cave.x_min) = true;
            }
            None => panic!("Whoops! Fell off the edge"),
        }
    }
    panic!("Broke loop unexpectedly");
}

#[rstest]
#[case::test("./test14.txt", 93)]
#[case::test("./input14.txt", 27194)]
fn test_part_2(#[case] filename: &str, #[case] result: usize) {
    assert_eq!(part_2(filename), result);
}
