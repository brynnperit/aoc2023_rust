#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum RockTile {
    RoundRock,
    CubeRock,
    Empty,
}

impl RockTile {
    pub fn from_char(tile_char: char) -> Option<Self> {
        match tile_char {
            'O' => Some(Self::RoundRock),
            '#' => Some(Self::CubeRock),
            '.' => Some(Self::Empty),
            _ => None,
        }
    }
}
