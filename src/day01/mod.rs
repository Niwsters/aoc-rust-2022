mod input;

fn elves(input: &str) -> Vec<u32> {
    let lines: Vec<&str> = input.split("\n").collect();

    let mut elves = vec![];
    let mut elf = 0;
    for line in lines {
        if line == "" {
            elves.push(elf);
            elf = 0;
            continue;
        }
        elf += line.parse::<u32>().unwrap();
    }

    elves
}

fn part1(input: &str) -> u32 {
    *elves(input).iter().max().unwrap()
}

fn part2(input: &str) -> u32 {
    let mut elves = elves(input);
    elves.sort();
    elves.iter().rev().take(3).sum()
}

pub fn test() {
    assert_eq!(part1(input::TEST), 24000);
    assert_eq!(part2(input::TEST), 41000);
    assert_eq!(part1(input::REAL), 66616);
    assert_eq!(part2(input::REAL), 199172);
}
