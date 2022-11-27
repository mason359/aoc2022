pub mod day1;

pub mod day;

use std::env;

fn main() {
    let (day, part) = parse_args(env::args());
    let day_box = get_day(day);
    day_box.run_solution(day, part);
}

fn get_day(day: u32) -> Box<dyn day::Day> {
    match day {
        1 => Box::new(day1::Day1),
        1..=25 => panic!("Day {} not implemented", day),
        _ => panic!("Invalid day"),
    }
}

fn parse_args(mut args: env::Args) -> (u32, u32) {
    args.next();
    let day = match args.next() {
        Some(day) => match day.parse::<u32>().unwrap() {
            day_num if (1..=25).contains(&day_num) => Ok(day_num),
            _ => Err("Invalid day"),
        },
        None => Err("No day number provided"),
    };
    let part = match args.next() {
        Some(part) => match part.parse::<u32>().unwrap() {
            part_num if (1..=2).contains(&part_num) => Ok(part_num),
            _ => Err("Invalid part"),
        }
        None => Err("No part number provided"),
    };
    (day.unwrap(), part.unwrap())
}

pub use day::Day;