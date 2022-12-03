use std::time::Instant;

pub trait Day {
    fn part1(&self, input: String) -> i64;

    fn part2(&self, input: String) -> i64;

    fn run_solution(&self, day_num: u32, part_num: u32, input: String) {
        let runner = match part_num {
            1 => Box::new(|input: String| -> i64 { self.part1(input) }) as Box<dyn Fn(_) -> i64>,
            2 => Box::new(|input: String| -> i64 { self.part2(input) }) as Box<dyn Fn(_) -> i64>,
            _ => panic!("Invalid part"),
        };

        println!("Day {}, Part {}:", day_num, part_num);

        let start = Instant::now();
        let result = runner(input);
        let mut time = start.elapsed().as_secs_f64();

        let units = ["s", "ms", "us", "ns"];
        let mut unit = "";
        for i in 0..3 {
            if time > 1.0 {
                unit = units[i];
                break;
            }
            time *= 1000.0;
        }

        println!("Result: {}", result);
        println!("Finished in {}{}", time, unit);
    }
}