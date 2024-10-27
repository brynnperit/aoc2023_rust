use num::{CheckedSub, Integer, One};

use super::direction::Direction;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub struct Coord2D<T: Integer + Copy> {
    x: T,
    y: T,
}

impl<T: Integer + Copy + CheckedSub + One> Coord2D<T> {
    pub const fn new(x: T, y: T) -> Self {
        Coord2D { x, y }
    }

    pub const fn new_row_column(row: T, col: T) -> Self {
        Coord2D { x: col, y: row }
    }

    pub fn in_direction(&mut self, direction: &Direction) -> Option<Self> {
        match direction {
            Direction::North => self.y = self.y.checked_sub(&num::one())?,
            Direction::East => self.x = self.x.add(num::one()),
            Direction::South => self.y = self.y.add(num::one()),
            Direction::West => self.x = self.x.checked_sub(&num::one())?,
        }
        Some(*self)
    }

    pub fn new_in_direction(&self, direction: &Direction) -> Option<Self> {
        self.new_in_direction_at_distance(direction, num::one())
    }

    pub fn new_in_direction_at_distance(&self, direction: &Direction, distance: T) -> Option<Self> {
        match direction {
            Direction::North => Some(Self {
                x: self.x,
                y: self.y.checked_sub(&distance)?,
            }),
            Direction::East => Some(Self {
                x: self.x.add(distance),
                y: self.y,
            }),
            Direction::South => Some(Self {
                x: self.x,
                y: self.y.add(distance),
            }),
            Direction::West => Some(Self {
                x: self.x.checked_sub(&distance)?,
                y: self.y,
            }),
        }
    }

    pub const fn get_row(&self) -> T {
        self.y
    }

    pub const fn get_col(&self) -> T {
        self.x
    }

    pub const fn get_x(&self) -> T {
        self.x
    }

    pub const fn get_y(&self) -> T {
        self.y
    }
}
