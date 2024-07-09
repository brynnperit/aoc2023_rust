use crate::shared_libs::direction::Direction;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PipeTile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    StartPosition,
}

impl PipeTile {
    pub const NAVIGABLE_PIPES: [PipeTile; 6] = [
        Self::Vertical,
        Self::Horizontal,
        Self::NorthEast,
        Self::NorthWest,
        Self::SouthEast,
        Self::SouthWest,
    ];
    pub const VERTICAL_CONNECTING: [Direction; 2] = [Direction::North, Direction::South];
    pub const HORIZONTAL_CONNECTING: [Direction; 2] = [Direction::West, Direction::East];
    pub const NORTH_EAST_CONNECTING: [Direction; 2] = [Direction::North, Direction::East];
    pub const NORTH_WEST_CONNECTING: [Direction; 2] = [Direction::North, Direction::West];
    pub const SOUTH_WEST_CONNECTING: [Direction; 2] = [Direction::South, Direction::West];
    pub const SOUTH_EAST_CONNECTING: [Direction; 2] = [Direction::South, Direction::East];

    pub fn from_char(tile_char: char) -> Self {
        match tile_char {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            '.' => Self::Ground,
            'S' => Self::StartPosition,
            other => panic!("Found invalid tile character:{}", other),
        }
    }

    pub const fn get_2_connecting_directions(&self) -> &[Direction; 2] {
        match self {
            PipeTile::Vertical => &Self::VERTICAL_CONNECTING,
            PipeTile::Horizontal => &Self::HORIZONTAL_CONNECTING,
            PipeTile::NorthEast => &Self::NORTH_EAST_CONNECTING,
            PipeTile::NorthWest => &Self::NORTH_WEST_CONNECTING,
            PipeTile::SouthWest => &Self::SOUTH_WEST_CONNECTING,
            PipeTile::SouthEast => &Self::SOUTH_EAST_CONNECTING,
            _ => {
                panic!("Cannot get only 2 navigable connecting directions for this direction")
            }
        }
    }

    pub fn connects(&self, other_tile: &PipeTile, other_direction: &Direction) -> bool {
        if *self == Self::StartPosition || *other_tile == Self::StartPosition {
            return false;
        }
        match self {
            PipeTile::Ground => (),
            _ => {
                if !self.get_2_connecting_directions().contains(other_direction) {
                    return false;
                }
            }
        }
        match self {
            PipeTile::Ground => match other_tile {
                PipeTile::Ground => true,
                _ => false,
            },
            _ => match other_tile {
                PipeTile::Ground => false,
                _ => other_tile
                    .get_2_connecting_directions()
                    .contains(&other_direction.reverse()),
            },
        }
    }

    pub fn append_enlarged_tile(&self, target: &mut Vec<Vec<PipeTile>>) {
        target[0].push(Self::Ground);
        match self {
            PipeTile::Horizontal | PipeTile::NorthWest | PipeTile::SouthWest => {
                target[1].push(Self::Horizontal)
            }
            _ => target[1].push(Self::Ground),
        }
        target[2].push(Self::Ground);

        match self {
            PipeTile::Vertical | PipeTile::NorthEast | PipeTile::NorthWest => {
                target[0].push(Self::Vertical)
            }
            _ => target[0].push(Self::Ground),
        }
        target[1].push(*self);
        match self{
            PipeTile::Vertical|
            PipeTile::SouthWest|
            PipeTile::SouthEast => target[2].push(Self::Vertical),
            _=>target[2].push(Self::Ground),
        }
        
        target[0].push(Self::Ground);
        match self {
            PipeTile::Horizontal | PipeTile::NorthEast | PipeTile::SouthEast => {
                target[1].push(Self::Horizontal)
            }
            _ => target[1].push(Self::Ground),
        }
        target[2].push(Self::Ground);
    }
}
