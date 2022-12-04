use crate::Day;

pub struct Day4;

impl Day for Day4 {
    fn part1(&self, input: String) -> i64 {
        count_overlapping(input, contains)
    }

    fn part2(&self, input: String) -> i64 {
        count_overlapping(input, overlaps)
    }
}

fn count_overlapping(input: String, check_overlap: fn((i64, i64), (i64, i64)) -> bool) -> i64 {
    input
        .lines()
        .map(|pair| pair
            .split(",")
            .map(|elf| elf
                .split_once("-")
                .unwrap()
            )
            .map(|(lower, upper)| (lower.parse::<i64>().unwrap(), upper.parse::<i64>().unwrap()))
            .collect::<Vec<_>>()
        )
        .filter(|pair| check_overlap(pair[0], pair[1]) || check_overlap(pair[1], pair[0]))
        .count() as i64
}

fn contains(elf1: (i64, i64), elf2: (i64, i64)) -> bool {
    elf1.0 <= elf2.0 && elf1.1 >= elf2.1
}

fn overlaps(elf1: (i64, i64), elf2: (i64, i64)) -> bool {
    elf1.0 <= elf2.0 && elf1.1 >= elf2.0
}