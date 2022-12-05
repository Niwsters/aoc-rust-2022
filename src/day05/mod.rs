use std::collections::{ HashMap, VecDeque };

mod input;

type Stack = VecDeque<char>;

fn stacks(stacks_str: &str) -> Vec<Stack> {
    let mut lines: Vec<&str> = stacks_str.split('\n').collect();
    let last_line = lines.pop().unwrap();

    let mut dict: HashMap<usize, usize> = HashMap::new();
    for (i, c) in last_line.chars().enumerate() {
        if c != ' ' {
            dict.insert((c.to_digit(10).unwrap() - 1) as usize, i);
        }
    }

    let stack_count = dict.keys().max().unwrap() + 1;
    let mut stacks: Vec<Stack> = vec![VecDeque::new(); stack_count];
    for line in lines.iter() {
        for (stack_n, stack_i) in &dict {
            let c = line.chars().nth(*stack_i).unwrap();
            if c != ' ' {
                stacks[*stack_n].push_front(c);
            }
        }
    }

    stacks
}

#[derive(Debug)]
struct Instruction {
    amount: u32,
    from: usize,
    to: usize
}

fn instruction(instr_str: &str) -> Instruction {
    let mut split = instr_str.split(' ');
    split.next();
    let amount: u32 = split.next().unwrap().parse().unwrap();
    split.next();
    let from = split.next().unwrap().parse().unwrap();
    split.next();
    let to = split.next().unwrap().parse().unwrap();

    Instruction {
        amount,
        from,
        to
    }
}

fn instructions(procedure_str: &str) -> Vec<Instruction> {
    procedure_str
        .split('\n')
        .map(instruction)
        .collect()
}

fn apply(stacks: Vec<Stack>, instructions: Vec<Instruction>) -> Vec<Stack> {
    let mut stacks = stacks.clone();

    for instruction in instructions {
        for _ in 0..instruction.amount {
            let a_crate = stacks[instruction.from - 1].pop_back().unwrap();
            stacks[instruction.to - 1].push_back(a_crate);
        }
    }

    stacks
}

fn top_crates(stacks: Vec<Stack>) -> String {
    stacks
        .iter()
        .map(|stack| stack.iter().last().unwrap())
        .collect()
}

fn part1(input: &str) -> String {
    let (stacks_str, procedure) = input.split_once("\n\n").unwrap();

    let stacks = stacks(stacks_str);
    let instructions = instructions(procedure);
    let stacks = apply(stacks, instructions);

    top_crates(stacks)
}

pub fn test() {
    assert_eq!(part1(input::TEST), "CMZ");
    assert_eq!(part1(input::REAL), "VPCDMSLWJ");
}
