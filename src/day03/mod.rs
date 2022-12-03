mod input;

use std::collections::HashSet;

#[derive(Debug)]
struct Rucksack {
    first: HashSet<char>,
    second: HashSet<char>
}

fn rucksack(line: &str) -> Rucksack {
    let middle = line.len() / 2;
    let (first, second) = line.split_at(middle);

    let first: HashSet<char> = first.chars().collect();
    let second: HashSet<char> = second.chars().collect();

    Rucksack { first, second }
}

fn in_common(rucksack: Rucksack) -> char {
    rucksack.first.intersection(&rucksack.second).last().unwrap().clone()
}

static PRIORITIES: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
fn priority(c: char) -> usize {
    PRIORITIES.find(c).unwrap() + 1
}

fn part1(input: &str) -> usize {
    input
        .split('\n')
        .map(rucksack)
        .map(in_common)
        .map(priority)
        .sum()
}

pub fn test() {
    assert_eq!(part1(input::TEST), 157);
    assert_eq!(part1(input::REAL), 0);
}
