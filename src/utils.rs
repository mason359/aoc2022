use std::env;
use std::fs;
use reqwest::blocking::Client;

use crate::Day;
use crate::day1;

pub fn parse_args(mut args: env::Args) -> (u32, u32, bool) {
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
    let use_test_input = Some("-t".to_string()) == args.next();
    (day.unwrap(), part.unwrap(), use_test_input)
}

pub fn get_day(day: u32) -> Box<dyn Day> {
    match day {
        1 => Box::new(day1::Day1),
        1..=25 => unimplemented!(),
        _ => panic!("Invalid day"),
    }
}

pub fn get_input(day: u32, use_test_input: bool) -> String {
    let input: String;
    if use_test_input {
        input = fs::read_to_string("./input/input0.txt").unwrap();
    } else {
        input = match fs::read_to_string(format!("./input/input{}.txt", day)) {
            Ok(text) => text,
            Err(_) => download_input(day),
        }
    }
    input
}

pub fn download_input(day: u32) -> String {
    println!("Downloading input for day {}...", day);

    let uri = format!("https://adventofcode.com/2022/day/{}/input", day);
    let session = fs::read_to_string("./session").unwrap();
    let client = Client::new();
    let input = client.get(uri).header("Cookie", format!("session={}", session)).send().unwrap().text().unwrap();
    fs::write(format!("./input/input{}.txt", day), input.as_str()).unwrap();
    input
}