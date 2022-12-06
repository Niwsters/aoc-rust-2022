mod input;

use std::collections::HashSet;

fn from(i: usize, packet_length: usize) -> usize {
    match i < packet_length - 1 {
        true => 0,
        false => i - (packet_length - 1)
    }
}

fn is_marker(s: &str, min_length: usize) -> bool {
    if s.len() < min_length {
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

fn marker_pos(input: &str, packet_length: usize) -> usize {
    for i in 0..input.len() {
        let from = from(i, packet_length);
        let to = i+1;

        let s = &input[from..to];

        if is_marker(s, packet_length) {
            return i + 1;
        }
    }

    0
}

fn part1(input: &str) -> usize {
    marker_pos(input, 4)
}

fn part2(input: &str) -> usize {
    marker_pos(input, 14)
}

pub fn test() {
    assert_eq!(part1(input::TEST), 7);
    assert_eq!(part1(input::REAL), 1647);
    assert_eq!(part2(input::TEST), 19);
    assert_eq!(part2(input::REAL), 2447);
}
