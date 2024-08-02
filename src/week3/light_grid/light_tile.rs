use crate::shared_libs::direction::Direction;

#[derive(Clone)]
pub enum LightTile {
    Empty,
    Mirror(Direction, Direction),
    Splitter(Direction),
}

impl LightTile {
    pub fn from_char(light_char: char) -> Option<Self> {
        Some(match light_char {
            '.' => LightTile::Empty,
            '|' => LightTile::Splitter(Direction::North),
            '-' => LightTile::Splitter(Direction::East),
            '/' => LightTile::Mirror(Direction::East, Direction::North),
            '\\' => LightTile::Mirror(Direction::East, Direction::South),
            _ => return None,
        })
    }
}
