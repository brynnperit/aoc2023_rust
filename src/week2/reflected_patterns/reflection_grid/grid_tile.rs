#[derive(PartialEq)]
pub enum GridTile{
    Ash,
    Rock,
}

impl GridTile{
    pub const fn from_char(tile_char:char)->Option<Self>{
        match tile_char{
            '#'=>Some(GridTile::Ash),
            '.'=>Some(GridTile::Rock),
            _=>None
        }
    }
}