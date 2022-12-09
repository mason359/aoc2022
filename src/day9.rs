use crate::{Coord, Day};

use num::signum;
use std::collections::HashSet;

pub struct Day9;

impl Day for Day9 {
    fn part1(&self, input: String) -> i64 {
        move_rope(input, 2)
    }
    
    fn part2(&self, input: String) -> i64 {
        move_rope(input, 10)
    }
}

fn move_rope(input: String, num_knots: usize) -> i64 {
    let moves = parse_input(input);
    let mut knots: Vec<Coord> = (0..num_knots)
        .map(|_| Coord { x: 0, y: 0 })
        .collect();
    let mut visited: HashSet<Coord> = HashSet::new();
    for (direction, spaces) in moves {
        for _ in 0..spaces {
            knots[0] += direction;
            let tail = knots
                .iter_mut()
                .reduce(move_knot)
                .unwrap();
            visited.insert(*tail);
        }
    }
    visited.len() as i64
}

fn move_knot<'a>(knot1: &mut Coord, knot2: &'a mut Coord) -> &'a mut Coord {
    let right = signum(knot1.x - knot2.x);
    let up = signum(knot1.y - knot2.y);
    let movement = match *knot1 - *knot2 {
        Coord { x: 2, y: _ } => Coord { x: 1, y: up },
        Coord { x: -2, y: _ } => Coord { x: -1, y: up },
        Coord { x: _, y: 2 } => Coord { x: right, y: 1 },
        Coord { x: _, y: -2 } => Coord { x: right, y: -1 },
        _ => Coord { x: 0, y: 0 },
    };
    *knot2 += movement;
    knot2
}

fn parse_input(input: String) -> Vec<(Coord, i64)> {
    input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(direction, spaces)| {
            let direction = match direction {
                "U" => Coord {x: 0, y: 1 },
                "D" => Coord {x: 0, y: -1 },
                "R" => Coord {x: 1, y: 0 },
                "L" => Coord {x: -1, y: 0 },
                _ => panic!(),
            };
            (direction, spaces.parse::<i64>().unwrap())
        })
        .collect()
}