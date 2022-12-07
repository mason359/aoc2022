use crate::Day;

use std::str::Lines;

pub struct Day7;

impl Day for Day7 {
    fn part1(&self, input: String) -> i64 {
        let mut dirs: Vec<i64> = Vec::new();
        let mut lines = input.lines();
        lines.next();
        get_directories(lines.by_ref(), &mut dirs);
        dirs
            .into_iter()
            .filter(|size| *size <= 100000)
            .sum::<i64>()
    }

    fn part2(&self, input: String) -> i64 {
        let mut dirs: Vec<i64> = Vec::new();
        let mut lines = input.lines();
        lines.next();
        let total_size = get_directories(lines.by_ref(), &mut dirs);
        dirs.sort();
        dirs
            .into_iter()
            .skip_while(|size| *size < total_size - 40000000)
            .next()
            .unwrap()
    }
}

fn get_directories(lines: &mut Lines, dirs: &mut Vec<i64>) -> i64 {
    lines.next();
    let contents: Vec<&str> = lines
        .take_while(|line| !line.starts_with('$'))
        .collect();
    let size = contents
        .iter()
        .map(|item| match item.split_once(' ').unwrap() {
            ("dir", _) => {
                let size = get_directories(lines, dirs);
                dirs.push(size);
                size
            }
            (size, _) => size.parse::<i64>().unwrap()
        })
        .sum::<i64>();
    lines.next();
    size
}