#![feature(iter_array_chunks)]

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day9;
pub mod day10;

pub mod day;
pub mod utils;
pub mod aoc_utils;

use std::env;

fn main() {
    let config = utils::parse_args(env::args());
    let day_box = utils::get_day(config.day);
    let input = utils::get_input(config.day, config.use_test_input);
    day_box.run_solution(config.day, config.part, input);
}

pub use day::Day;
pub use aoc_utils::Coord;