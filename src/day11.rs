use crate::Day;

use itertools::Itertools;
use std::collections::VecDeque;
use std::str::FromStr;

pub struct Day11;

impl Day for Day11 {
    fn part1(&self, input: String) -> i64 {
        monkey_business(input, 20, 3)
    }

    fn part2(&self, input: String) -> i64 {
        monkey_business(input, 10000, 1)
    }
}

fn monkey_business(input: String, rounds: i64, relief: i64) -> i64 {
    let mut monkeys = parse_input(input);
    let mut inspected: Vec<i64> = vec![0; monkeys.len()];
    let divisor: i64 = monkeys
        .iter()
        .map(|monkey| monkey.test)
        .product();
    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            inspected[m] += monkeys[m].items.len() as i64;
            for _ in 0..monkeys[m].items.len() {
                let mut item = monkeys[m].items.pop_front().unwrap();
                item = (monkeys[m].operation)(item) / relief;
                item %= divisor;
                let new_monkey = if item % monkeys[m].test == 0 { monkeys[m].monkey1 } else { monkeys[m].monkey2 };
                monkeys[new_monkey].items.push_back(item);
            }
        }
    }
    inspected
        .into_iter()
        .sorted()
        .rev()
        .take(2)
        .product()
}

fn parse_input(input: String) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|monkey| Monkey::from_str(monkey).unwrap())
        .collect()
}

struct Monkey {
    pub items: VecDeque<i64>,
    pub operation: Box<dyn Fn(i64) -> i64>,
    pub test: i64,
    pub monkey1: usize,
    pub monkey2: usize,
}

impl FromStr for Monkey {
    type Err = <i64 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().skip(1);
        let items = lines.next().unwrap().split_once(": ").unwrap().1;
        let items: VecDeque<i64> = items.split(", ").map(|item| item.parse::<i64>().unwrap()).collect();
        let operation: Vec<&str> = lines.next().unwrap().split_whitespace().collect();
        let operation: Box<dyn Fn(i64) -> i64> = if operation[5] == "old" {
            Box::new(|old| old * old)
        } else {
            let value = operation[5].parse::<i64>().unwrap();
            match operation[4] {
                "+" => Box::new(move |old| old + value),
                "*" => Box::new(move |old| old * value),
                _ => panic!(),
            }
        };
        let divisible = lines
            .map(|line| line
                .split(" ")
                .last()
                .unwrap()
                .parse::<i64>()
                .unwrap()
            )
            .collect::<Vec<_>>();
        Ok(Monkey { items, operation, test: divisible[0], monkey1: divisible[1] as usize, monkey2: divisible[2] as usize })
    }
}