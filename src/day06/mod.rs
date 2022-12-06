mod input;

use std::collections::HashSet;

fn from(i: usize) -> usize {
    match i < 3 {
        true => 0,
        false => i - 3
    }
}

fn is_marker(s: &str) -> bool {
    if s.len() < 4 {
        return false;
    }

    let mut found: HashSet<char> = HashSet::new();

    for c in s.chars() {
        if found.contains(&c) {
            return false 
        }

        found.insert(c);
    }

    true
}

fn part1(input: &str) -> usize {
    for i in 0..input.len() {
        let from = from(i);
        let to = i+1;

        let s = &input[from..to];

        if is_marker(s) {
            return i + 1;
        }
    }

    0
}

pub fn test() {
    assert_eq!(part1(input::TEST), 7);
    assert_eq!(part1(input::REAL), 1647);
}
