use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

use rock_tile::RockTile;

use crate::shared_libs::{coord_2_d::Coord2D, direction::Direction, grid_map::GridMap};

mod rock_tile;

pub struct ReflectorRocks {
    map: GridMap<RockTile>,
}

impl ReflectorRocks {
    pub fn from_file(path: std::ffi::OsString) -> Self {
        let input = clio::Input::new(&path).unwrap();
        let mut rows = Vec::new();
        for line in std::io::BufRead::lines(std::io::BufReader::new(input)).map_while(Result::ok) {
            rows.push(line.chars().filter_map(RockTile::from_char).collect());
        }
        ReflectorRocks {
            map: GridMap::new(rows),
        }
    }

    pub fn roll(&mut self, direction: &Direction) {
        let starting_position = match direction {
            Direction::North => Coord2D::new_row_column(0, 0),
            Direction::East => Coord2D::new_row_column(0, self.map.col_count() - 1),
            Direction::South => Coord2D::new_row_column(self.map.row_count() - 1, 0),
            Direction::West => Coord2D::new_row_column(0, 0),
        };
        let inner_direction = direction.reverse();
        let outer_direction = match direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        let mut outer_position = starting_position;
        while self.map.get_ref(&outer_position).is_some() {
            let mut inner_position = outer_position;
            while let Some(rock) = self.map.get_ref(&inner_position) {
                if *rock == RockTile::RoundRock {
                    self.roll_rock(&inner_position, direction);
                }
                if inner_position.in_direction(&inner_direction).is_none() {
                    break;
                }
            }
            if outer_position.in_direction(&outer_direction).is_none() {
                break;
            }
        }
    }

    fn roll_rock(
        &mut self,
        rock_coord: &Coord2D<usize>,
        direction: &Direction,
    ) -> Option<Coord2D<usize>> {
        let mut check_coord = rock_coord.new_in_direction(direction)?;
        if *self.map.get_ref(&check_coord)? == RockTile::Empty {
            while check_coord.in_direction(direction).is_some() {
                if let Some(rock) = self.map.get_ref(&check_coord) {
                    if *rock != RockTile::Empty {
                        check_coord.in_direction(&direction.reverse());
                        break;
                    }
                } else {
                    check_coord.in_direction(&direction.reverse());
                    break;
                }
            }
            self.map.set(&check_coord, RockTile::RoundRock);
            self.map.set(rock_coord, RockTile::Empty);
            return Some(check_coord);
        }
        None
    }
    pub fn get_loads(&self, direction: &Direction) -> Vec<u64> {
        match direction {
            Direction::North => {
                let mut loads = Vec::new();
                let rows = self.map.row_count();
                for col in self.map.cols_iter() {
                    let column_load: usize = col
                        .enumerate()
                        .filter(|(_, rock)| **rock == RockTile::RoundRock)
                        .map(|(row_index, _)| rows - row_index)
                        .sum();
                    loads.push(column_load.try_into().unwrap());
                }
                loads
            }
            Direction::East => todo!(),
            Direction::South => todo!(),
            Direction::West => todo!(),
        }
    }
    pub fn spin_cycle(&mut self, directions: Vec<Direction>, cycles: u64) {
        let mut cycle_hashes = HashMap::new();
        let mut current_cycle = 0;
        let mut skipped_to_end = false;
        let mut cycle_limit = cycles;
        while current_cycle <= cycle_limit {
            let mut hasher = DefaultHasher::new();
            self.map.hash(&mut hasher);
            let current_hash = hasher.finish();
            let possible_hash = cycle_hashes.get(&current_hash);
            if !skipped_to_end && possible_hash.is_some() {
                let repeat_cycle = possible_hash.unwrap();
                let repetition_count = current_cycle - repeat_cycle;
                let remaining_cycles = (cycle_limit - current_cycle) % repetition_count;
                cycle_limit = current_cycle + remaining_cycles;
                skipped_to_end = true;
            } else {
                cycle_hashes.insert(current_hash, current_cycle);
                for direction in directions.iter() {
                    self.roll(direction);
                }
            }
            current_cycle += 1;
        }
    }
}
