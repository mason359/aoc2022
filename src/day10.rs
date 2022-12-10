use crate::Day;

pub struct Day10;

impl Day for Day10 {
    fn part1(&self, input: String) -> i64 {
        let cycles: Vec<i64> = (0..21)
            .map(|_| 0)
            .chain(parse_input(input).into_iter())
            .collect();
        let mut x = 1;
        let mut cycle = -20;
        cycles
            .into_iter()
            .array_chunks()
            .map(|batch: [i64; 40]| {
                let signal = batch
                    .into_iter()
                    .sum::<i64>();
                cycle += 40;
                x += signal;
                x * cycle
            })
            .sum::<i64>()
    }

    fn part2(&self, input: String) -> i64 {
        let cycles = parse_input(input);
        let mut screen: Vec<Vec<char>> = (0..6)
            .map(|_| (0..40)
                .map(|_| ' ')
                .collect()
            )
            .collect();
        let mut x = 1;
        for (i, cycle) in cycles.into_iter().enumerate() {
            let row = i / 40;
            let col = i % 40;
            if x - 1 <= col as i64 && x + 1 >= col as i64 {
                screen[row][col] = '█';
            }
            x += cycle;
        }
        let out: String = screen
            .into_iter()
            .map(|row| row.into_iter().collect())
            .collect::<Vec<String>>()
            .join("\n");
        println!("{out}");
        0
    }
}

fn parse_input(input: String) -> Vec<i64> {
    input
        .lines()
        .map(|line| {
            if line == "noop" {
                vec![0]
            } else {
                vec![0, line.split_once(' ').unwrap().1.parse::<i64>().unwrap()]
            }
        })
        .flatten()
        .collect()
}