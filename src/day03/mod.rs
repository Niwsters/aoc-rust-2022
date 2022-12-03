mod input;

use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Rucksack {
    first: HashSet<char>,
    second: HashSet<char>
}

impl Rucksack {
    pub fn content(&self) -> HashSet<char> {
        self.first
            .union(&self.second)
            .map(|c| c.clone())
            .collect()
    }
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

#[derive(Debug, Clone)]
struct Group {
    first: Rucksack,
    second: Rucksack,
    third: Rucksack
}

fn groups(rucksacks: Vec<Rucksack>) -> Vec<Group> {
    let mut groups = vec![];
    let mut i = 0;
    let mut current: Vec<Rucksack> = vec![];
    for rucksack in rucksacks {
        current.push(rucksack);
        if i / 2 == 1 {
            i = 0;
            groups.push(Group {
                first: current[0].clone(),
                second: current[1].clone(),
                third: current[2].clone()
            });
            current = vec![];
            continue;
        }
        i += 1;
    }
    groups
}

fn group_in_common(group: &Group) -> char {
    let first = group.first.content();
    let second = group.second.content();
    let third = group.third.content();

    first
        .intersection(&second)
        .map(|c| c.clone())
        .collect::<HashSet<char>>()
        .intersection(&third)
        .last()
        .unwrap()
        .clone()
}

fn part2(input: &str) -> usize {
    let rucksacks = input
        .split('\n')
        .map(rucksack);

    groups(rucksacks.collect())
        .iter()
        .map(group_in_common)
        .map(priority)
        .sum()
}

pub fn test() {
    assert_eq!(part1(input::TEST), 157);
    assert_eq!(part1(input::REAL), 7746);
    assert_eq!(part2(input::TEST), 70);
    assert_eq!(part2(input::REAL), 2604);
}
