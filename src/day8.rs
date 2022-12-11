use crate::Day;
use crate::Grid;

pub struct Day8;

impl Day for Day8 {
    fn part1(&self, input: String) -> i64 {
        let forest = Grid::from(input);
        let mut visible = Grid::new(forest.rows(), forest.cols(), 0);
        for i in 0..forest.rows() {
            check_visible(forest.get_row(i), visible.get_row_mut(i));
            check_visible(forest.get_col(i), visible.get_col_mut(i));
            check_visible(
                forest.get_row(i).into_iter().rev().collect(),
                visible.get_row_mut(i).into_iter().rev().collect()
            );
            check_visible(
                forest.get_col(i).into_iter().rev().collect(),
                visible.get_col_mut(i).into_iter().rev().collect()
            );
        }
        visible.sum()
    }

    fn part2(&self, input: String) -> i64 {
        let forest = Grid::from(input);
        (1..forest.rows() - 1)
            .map(|r| (1..forest.cols() - 1)
                .map(|c| get_scenic_score(&forest, r, c))
                .max()
                .unwrap()
            )
            .max()
            .unwrap()
    }
}

fn check_visible(trees: Vec<i64>, mut visible: Vec<&mut i64>) {
    trees
        .into_iter()
        .enumerate()
        .fold(-1, |tallest, (i, tree)| {
            if tree > tallest {
                *visible[i] = 1;
                return tree
            }
            tallest
        });
}

fn get_scenic_score(forest: &Grid<i64>, r: usize, c: usize) -> i64 {
    let mut left = forest.get_row(r);
    let mut up = forest.get_col(c);
    let right = left.split_off(c).into_iter().skip(1).rev().collect();
    let down = up.split_off(r).into_iter().skip(1).rev().collect();
    [left, right, up, down]
        .into_iter()
        .map(|line| line
            .iter()
            .rev()
            .enumerate()
            .skip_while(|(_, tree)| **tree < forest.get(r, c))
            .next()
            .unwrap_or((line.len() - 1, &0))
            .0 as i64 + 1
        )
        .product::<i64>()
}