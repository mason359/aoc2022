#![feature(iter_array_chunks)]

pub mod day1;
pub mod day2;
pub mod day3;

pub mod day;
pub mod utils;

use std::env;

fn main() {
    let config = utils::parse_args(env::args());
    let day_box = utils::get_day(config.day);
    let input = utils::get_input(config.day, config.use_test_input);
    day_box.run_solution(config.day, config.part, input);
}

pub use day::Day;