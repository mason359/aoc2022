use std::time::Instant;

pub trait Day {
    fn part1(&self) -> i64;

    fn part2(&self) -> i64;

    fn run_solution(&self, day_num: u32, part_num: u32) {
        let runner = match part_num {
            1 => Box::new(|| -> i64 { self.part1() }) as Box<dyn Fn() -> i64>,
            2 => Box::new(|| -> i64 { self.part2() }) as Box<dyn Fn() -> i64>,
            _ => panic!("Invalid part"),
        };

        println!("Day {}, Part {}:", day_num, part_num);

        let start = Instant::now();
        let result = runner();
        let time = start.elapsed().as_secs_f64();

        println!("Result: {}", result);
        println!("Finished in {}s", time);
    }
}