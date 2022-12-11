use std::fmt;

#[derive(Debug)]
pub struct Grid<T> {
    grid: Vec<Vec<T>>
}

impl<T> Grid<T>
where
    T: Clone + Copy
{
    pub fn new(rows: usize, cols: usize, fill: T) -> Self {
        let grid: Vec<Vec<T>> = (0..rows)
            .map(|_| (0..cols)
                .map(|_| fill)
                .collect()
            )
            .collect();
        Self { grid }
    }

    pub fn get(&self, row: usize, col: usize) -> T {
        self.grid[row][col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        self.grid[row][col] = value;
    }

    pub fn get_row(&self, row: usize) -> Vec<T> {
        self.grid[row].clone()
    }

    pub fn get_row_mut(&mut self, row: usize) -> Vec<&mut T> {
        self.grid[row].iter_mut().collect()
    }

    pub fn get_col_mut(&mut self, col: usize) -> Vec<&mut T> {
        self.grid.iter_mut().map(|row| &mut row[col]).collect()
    }

    pub fn get_col(&self, col: usize) -> Vec<T> {
        self.grid.iter().map(|row| row[col]).collect()
    }

    pub fn rows(&self) -> usize {
        self.grid.len()
    }

    pub fn cols(&self) -> usize {
        self.grid[0].len()
    }
}

impl Grid<i64> {
    pub fn sum(&self) -> i64 {
        self.grid
            .iter()
            .map(|row| row.iter().sum::<i64>())
            .sum::<i64>()
    }
}

impl From<String> for Grid<i64> 
{
    fn from(input: String) -> Self {
        Self {
            grid: input
                .lines()
                .map(|line| line
                    .chars()
                    .map(|digit| digit.to_digit(10).unwrap() as i64)
                    .collect()
                )
                .collect()
        }
    }
}

impl<T> fmt::Display for Grid<T>
where
    T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = self.grid
            .iter()
            .map(|row| row
                .iter()
                .map(|item| format!("{item}"))
                .collect::<String>()
            )
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{out}")
    }
}