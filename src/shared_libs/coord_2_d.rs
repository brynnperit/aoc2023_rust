use num::{CheckedSub, Integer, One};

use super::direction::Direction;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Coord2D<T: Integer + Copy> {
    x: T,
    y: T,
}

impl<T: Integer + Copy + CheckedSub + One> Coord2D<T> {
    pub fn new_row_column(row: T, col: T) -> Self {
        Coord2D { x: col, y: row }
    }

    pub fn in_direction(&self, direction: &Direction) -> Option<Self> {
        match direction {
            Direction::North => Some(Self {
                x: self.x,
                y: self.y.checked_sub(&num::one())?,
            }),
            Direction::East => Some(Self {
                x: self.x.add(num::one()),
                y: self.y,
            }),
            Direction::South => Some(Self {
                x: self.x,
                y: self.y.add(num::one()),
            }),
            Direction::West => Some(Self {
                x: self.x.checked_sub(&num::one())?,
                y: self.y,
            }),
        }
    }

    pub fn get_row(&self) -> T {
        self.y
    }

    pub fn get_col(&self) -> T {
        self.x
    }
}
