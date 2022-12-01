use crate::Day;

use itertools::Itertools;

pub struct Day1;

impl Day for Day1 {
    fn part1(&self, input: String) -> i64 {
        input
            .split("\n\n")
            .map(|elf| elf
                .lines()
                .map(|food| food.parse::<i64>().unwrap())
                .sum::<i64>()
            )
            .max()
            .unwrap()
    }

    fn part2(&self, input: String) -> i64 {
        input
            .split("\n\n")
            .map(|elf| elf
                .lines()
                .map(|food| food.parse::<i64>().unwrap())
                .sum::<i64>()
            )
            .sorted()
            .rev()
            .take(3)
            .sum()
    }
}