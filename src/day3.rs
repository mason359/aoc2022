use crate::Day;

use std::collections::HashSet;

pub struct Day3;

impl Day for Day3 {
    fn part1(&self, input: String) -> i64 {
        input
            .lines()
            .map(|sack| [&sack[..(sack.len() / 2)], &sack[(sack.len() / 2)..]]
                .iter()
                .map(|compartment| compartment
                    .bytes()
                    .collect::<HashSet<_>>()
                )
                .reduce(|items1, items2| items1
                    .intersection(&items2)
                    .copied()
                    .collect()
                )
                .unwrap()
                .iter()
                .next()
                .unwrap()
                .clone()
            )
            .map(get_priority)
            .sum::<i64>()
    }

    fn part2(&self, input: String) -> i64 {
        input
            .lines()
            .array_chunks()
            .map(|group: [&str; 3]| group
                .iter()
                .map(|elf| elf
                    .bytes()
                    .collect::<HashSet<_>>()
                )
                .reduce(|elf1, elf2| elf1
                    .intersection(&elf2)
                    .copied()
                    .collect()
                )
                .unwrap()
                .iter()
                .next()
                .unwrap()
                .clone()
            )
            .map(get_priority)
            .sum::<i64>()
    }
}

fn get_priority(item: u8) -> i64 {
    return if item >= 'a' as u8 {
        item - 'a' as u8 + 1
    } else {
        item - 'A' as u8 + 27
    } as i64
}