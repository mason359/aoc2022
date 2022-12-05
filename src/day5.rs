use crate::Day;

pub struct Day5;

impl Day for Day5 {
    fn part1(&self, input: String) -> i64 {
        move_crates(input, crate_mover_9000);
        0
    }

    fn part2(&self, input: String) -> i64 {
        move_crates(input, crate_mover_9001);
        0
    }
}

fn move_crates(input: String, crate_mover: fn(&mut Vec<Vec<char>>, Vec<Vec<usize>>)) {
    let (stacks, steps) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(stacks);
    let steps = parse_steps(steps);
    crate_mover(&mut stacks, steps);
    let top_crates: String = stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect();
    println!("{top_crates}");
}

fn crate_mover_9000(stacks: &mut Vec<Vec<char>>, steps: Vec<Vec<usize>>) {
    for step in steps {
        for _ in 0..step[0] {
            let krate = stacks[step[1] - 1].pop().unwrap();
            stacks[step[2] - 1].push(krate);
        }
    }
}

fn crate_mover_9001(stacks: &mut Vec<Vec<char>>, steps: Vec<Vec<usize>>) {
    for step in steps {
        let from = &mut stacks[step[1] - 1];
        let mut moved = from.split_off(from.len() - step[0]);
        stacks[step[2] - 1].append(&mut moved);
    }
}

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    let mut stacks = Vec::new();
    for line in input.lines().rev().skip(1) {
        for (stack_num, krate) in line.chars().skip(1).step_by(4).enumerate() {
            if stacks.len() <= stack_num {
                stacks.push(Vec::new());
            }
            if krate != ' ' {
                stacks[stack_num].push(krate);
            }
        }
    }
    stacks
}

fn parse_steps(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| line
            .split(" ")
            .skip(1)
            .step_by(2)
            .map(|num| num.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>()
}