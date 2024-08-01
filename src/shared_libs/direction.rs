#[derive(Eq, PartialEq, Hash,Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub const CARDINAL_DIRECTIONS: [Direction; 4] =
        [Self::North, Self::East, Self::South, Self::West];

    pub const fn reverse(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}
