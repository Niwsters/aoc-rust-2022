mod input;

use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Pair {
    first: HashSet<u32>,
    second: HashSet<u32>
}

fn assigned(spec: &str) -> HashSet<u32> {
    let (start, end) = spec.split_once('-').unwrap();

    let start = start.parse::<u32>().unwrap();
    let end = end.parse::<u32>().unwrap();
    (start..(end + 1)).collect::<HashSet<u32>>()
}

fn pair(line: &str) -> Pair {
    let (first, second) = line.split_once(',').unwrap();

    Pair {
        first: assigned(first),
        second: assigned(second)
    }
}

fn fully_overlaps(pair: &Pair) -> bool {
    pair.first.is_subset(&pair.second) ||
    pair.second.is_subset(&pair.first)
}

fn overlaps(pair: &Pair) -> bool {
    pair.first.intersection(&pair.second).count() > 0
}

fn part1(input: &str) -> usize {
    input
        .split('\n')
        .map(pair)
        .filter(fully_overlaps)
        .count()
}

fn part2(input: &str) -> usize {
    input
        .split('\n')
        .map(pair)
        .filter(overlaps)
        .count()
}

pub fn test() {
    assert_eq!(part1(input::TEST), 2);
    assert_eq!(part1(input::REAL), 494);
    assert_eq!(part2(input::TEST), 4);
    assert_eq!(part2(input::REAL), 833);
}
