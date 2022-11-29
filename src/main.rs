pub mod day1;

pub mod day;
pub mod utils;

use std::env;

fn main() {
    let (day, part, use_test_input) = utils::parse_args(env::args());
    let day_box = utils::get_day(day);
    let input = utils::get_input(day, use_test_input);
    day_box.run_solution(day, part, input);
}

pub use day::Day;