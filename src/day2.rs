use crate::Day;

pub struct Day2;

impl Day for Day2 {
    fn part1(&self, input: String) -> i64 {
        input
            .lines()
            .map(|round| round.as_bytes())
            .map(|round| (((round[2] - 19 - round[0]) % 3) * 3 + round[2] - 'W' as u8) as i64)
            .sum()
    }

    fn part2(&self, input: String) -> i64 {
        input
            .lines()
            .map(|round| round.as_bytes())
            .map(|round| ((round[0] - 'A' as u8 + round[2] - 'W' as u8 + 1) % 3 + 1 + (round[2] - 'X' as u8) * 3) as i64)
            .sum()
    }
}