use crate::Day;

use std::collections::HashSet;

pub struct Day6;

impl Day for Day6 {
    fn part1(&self, input: String) -> i64 {
        find_marker(input, 4)
    }

    fn part2(&self, input: String) -> i64 {
        find_marker(input, 14)
    }
}

fn find_marker(input: String, size: usize) -> i64 {
    (size..input.len())
        .skip_while(|i| input[*i - size..*i]
            .chars()
            .collect::<HashSet<char>>()
            .len() < size
        )
        .next()
        .unwrap() as i64
}