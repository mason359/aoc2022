use crate::Day;

use std::collections::HashSet;

pub struct Day3;

impl Day for Day3 {
    fn part1(&self, input: String) -> i64 {
        input
            .lines()
            .map(|sack| sack.split_at(sack.len() / 2))
            .map(|(sack1, sack2)| get_common(&[sack1, sack2]))
            .map(get_priority)
            .sum::<i64>()
    }

    fn part2(&self, input: String) -> i64 {
        input
            .lines()
            .array_chunks()
            .map(|group: [&str; 3]| get_common(&group))
            .map(get_priority)
            .sum::<i64>()
    }
}

fn get_common(sacks: &[&str]) -> u8 {
    sacks
        .iter()
        .map(|sack| sack
            .bytes()
            .collect::<HashSet<_>>()
        )
        .reduce(|items1, items2| &items1 & &items2)
        .unwrap()
        .into_iter()
        .next()
        .unwrap()
}

fn get_priority(item: u8) -> i64 {
    return if item >= 'a' as u8 {
        item - 'a' as u8 + 1
    } else {
        item - 'A' as u8 + 27
    } as i64
}