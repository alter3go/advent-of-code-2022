use std::{cell::RefCell, collections::HashMap, rc::Rc};

use rstest::rstest;

use crate::fs;

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
        for line in fs::read_lines(filename).unwrap().filter_map(|s| s.ok()) {
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

pub fn part_1(filename: &str) -> u64 {
    let root = inspect_filesystem(filename);
    all_sizes(&root)
        .into_iter()
        .filter(|size| *size <= 100_000)
        .sum()
}

#[rstest]
#[case::test("./test07.txt", 95437)]
#[case::input("./input07.txt", 1490523)]
fn test_part_1(#[case] filename: &str, #[case] result: u64) {
    assert_eq!(part_1(filename), result);
}

pub fn part_2(filename: &str) -> u64 {
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

#[rstest]
#[case::test("./test07.txt", 24933642)]
#[case::input("./input07.txt", 12390492)]
fn test_part_2(#[case] filename: &str, #[case] result: u64) {
    assert_eq!(part_2(filename), result);
}
