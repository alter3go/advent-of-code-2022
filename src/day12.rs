use petgraph::algo::dijkstra;
use petgraph::prelude::GraphMap;
use rstest::rstest;

use crate::vec2d;
use crate::vec2d::Vec2d;

#[derive(Debug, PartialEq)]
struct Heightmap {
    elevations: Vec2d<u8>,
    start: (usize, usize),
    end: (usize, usize),
}

fn map_from_file(filename: &str) -> Vec2d<u8> {
    let map_vec: Vec<u8>;
    let input = vec2d::input_from_file(filename);
    let (width, height) = (input.get(0).unwrap().len(), input.len());
    map_vec = input.into_iter().flatten().collect();
    Vec2d::new(map_vec, height, width)
}

fn heightmap_from_file(filename: &str) -> Heightmap {
    let mut elevations: Vec2d<u8> = map_from_file(filename);
    let mut start = (0, 0);
    let mut end = (0, 0);
    for i in 0..elevations.row_count {
        for j in 0..elevations.col_count {
            let spot = elevations.index_mut(i, j);
            if *spot == 'S' as u8 {
                start = (i, j);
                *spot = 'a' as u8;
            } else if *spot == 'E' as u8 {
                end = (i, j);
                *spot = 'z' as u8;
            }
        }
    }
    Heightmap {
        elevations,
        start,
        end,
    }
}

#[test]
fn test_heightmap_from_file() {
    let result = Heightmap {
        elevations: Vec2d::new(
            Vec::from_iter("aabqponmabcryxxlaccszzxkacctuvwjabdefghi".bytes()),
            5,
            8,
        ),
        start: (0, 0),
        end: (2, 5),
    };
    assert_eq!(heightmap_from_file("./test12.txt"), result);
}

fn paths_from_heightmap(
    heightmap: &Heightmap,
    reversed_edges: bool,
) -> GraphMap<(usize, usize), i32, petgraph::Directed> {
    let mut paths = GraphMap::new();
    for i in 0..heightmap.elevations.row_count {
        for j in 0..heightmap.elevations.col_count {
            paths.add_node((i, j));
        }
    }
    for i in 0..heightmap.elevations.row_count {
        for j in 0..heightmap.elevations.col_count {
            let me = heightmap.elevations.index(i, j);
            let me_coords = (i, j);
            if i > 0 {
                // Add a directed edge, if appropriate, to the square above this one
                let them = heightmap.elevations.index(i - 1, j);
                let them_coords = (i - 1, j);
                if *them <= me + 1 {
                    if reversed_edges {
                        paths.add_edge(them_coords, me_coords, 1);
                    } else {
                        paths.add_edge(me_coords, them_coords, 1);
                    }
                }
            }
            if i < heightmap.elevations.row_count - 1 {
                // The square below
                let them = heightmap.elevations.index(i + 1, j);
                let them_coords = (i + 1, j);
                if *them <= me + 1 {
                    if reversed_edges {
                        paths.add_edge(them_coords, me_coords, 1);
                    } else {
                        paths.add_edge(me_coords, them_coords, 1);
                    }
                }
            }
            if j > 0 {
                // The square to the left
                let them = heightmap.elevations.index(i, j - 1);
                let them_coords = (i, j - 1);
                if *them <= me + 1 {
                    if reversed_edges {
                        paths.add_edge(them_coords, me_coords, 1);
                    } else {
                        paths.add_edge(me_coords, them_coords, 1);
                    }
                }
            }
            if j < heightmap.elevations.col_count - 1 {
                // The square to the right
                let them = heightmap.elevations.index(i, j + 1);
                let them_coords = (i, j + 1);
                if *them <= me + 1 {
                    if reversed_edges {
                        paths.add_edge(them_coords, me_coords, 1);
                    } else {
                        paths.add_edge(me_coords, them_coords, 1);
                    }
                }
            }
        }
    }
    paths
}

pub fn part_1(filename: &str) -> i32 {
    let map = heightmap_from_file(filename);
    let paths = paths_from_heightmap(&map, false);
    *dijkstra(&paths, map.start, Some(map.end), |_| 1)
        .get(&map.end)
        .unwrap()
}

#[rstest]
#[case::test("./test12.txt", 31)]
#[case::input("./input12.txt", 497)]
fn test_part_1(#[case] filename: &str, #[case] result: i32) {
    assert_eq!(part_1(filename), result);
}

pub fn part_2(filename: &str) -> i32 {
    let map = heightmap_from_file(filename);
    let paths = paths_from_heightmap(&map, true);
    let shortest_paths = dijkstra(&paths, map.end, None, |_| 1);
    shortest_paths
        .into_iter()
        .filter_map(|(k, v)| {
            if *map.elevations.index(k.0, k.1) == 'a' as u8 {
                Some(v)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

#[rstest]
#[case::test("./test12.txt", 29)]
#[case::input("./input12.txt", 492)]
fn test_part_2(#[case] filename: &str, #[case] result: i32) {
    assert_eq!(part_2(filename), result);
}
