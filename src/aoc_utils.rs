use derive_more::{Add, AddAssign, Sub};

#[derive(Add, AddAssign, Clone, Copy, Eq, Hash, PartialEq, Sub)]
pub struct Coord {
    pub x: i64,
    pub y: i64,
}