mod input;

#[derive(Clone, Debug)]
enum Choice {
    Rock,
    Paper,
    Scissors
}

fn score_choice(choice: &Choice) -> u32 {
    match choice {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3
    }
}

fn score_win(me: &Choice, them: &Choice) -> u32 {
    match me {
        Choice::Rock => match them {
            Choice::Rock => 3,
            Choice::Paper => 0,
            Choice::Scissors => 6
        },
        Choice::Paper => match them {
            Choice::Rock => 6,
            Choice::Paper => 3,
            Choice::Scissors => 0
        },
        Choice::Scissors => match them {
            Choice::Rock => 0,
            Choice::Paper => 6,
            Choice::Scissors => 3
        }
    }
}

fn choice_pt1(c: &str) -> Choice {
    match c {
        "A" => Choice::Rock,
        "B" => Choice::Paper,
        "C" => Choice::Scissors,
        "X" => Choice::Rock,
        "Y" => Choice::Paper,
        "Z" => Choice::Scissors,
        _ => panic!("Unknown choice: {}", c)
    }
}

fn choices_pt1(line: &str) -> [Choice; 2] {
    let mut split = line.split(' ');
    [choice_pt1(split.next().unwrap()), choice_pt1(split.next().unwrap())]
}

enum Goal {
    Win,
    Draw,
    Lose
}

fn goal(c: &str) -> Goal {
    match c {
        "X" => Goal::Lose,
        "Y" => Goal::Draw,
        "Z" => Goal::Win,
        _ => panic!("Unknown goal: {}", c)
    }
}

fn losing_move(them: &Choice) -> Choice {
    match them {
        Choice::Rock => Choice::Scissors,
        Choice::Paper => Choice::Rock,
        Choice::Scissors => Choice::Paper
    }
}

fn drawing_move(them: &Choice) -> Choice {
    them.clone()
}

fn winning_move(them: &Choice) -> Choice {
    match them {
        Choice::Rock => Choice::Paper,
        Choice::Paper => Choice::Scissors,
        Choice::Scissors => Choice::Rock
    }
}

fn choice_pt2(goal_str: &str, them: &Choice) -> Choice {
    match goal(goal_str) {
        Goal::Lose => losing_move(them),
        Goal::Draw => drawing_move(them),
        Goal::Win => winning_move(them)
    }
}

fn choices_pt2(line: &str) -> [Choice; 2] {
    let mut split = line.split(' ');
    let them = choice_pt1(split.next().unwrap());
    let me = choice_pt2(split.next().unwrap(), &them);
    [them, me]
}

fn score(choices: [Choice; 2]) -> u32 {
    let them = &choices[0];
    let me = &choices[1];
    score_choice(me) + score_win(me, them)
}

fn part1(input: &str) -> u32 {
    input
        .split('\n')
        .map(|line| choices_pt1(line))
        .map(|choices| score(choices))
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .split('\n')
        .map(|line| choices_pt2(line))
        .map(|choices| score(choices))
        .sum()
}

pub fn test() {
    assert_eq!(part1(input::TEST), 15);
    assert_eq!(part1(input::REAL), 13446);
    assert_eq!(part2(input::TEST), 12);
    assert_eq!(part2(input::REAL), 12);
}
