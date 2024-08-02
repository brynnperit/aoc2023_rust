use std::{collections::BTreeMap, io::BufRead};

use light_tile::LightTile;

use crate::shared_libs::{coord_2_d::Coord2D, direction::Direction, grid_map::GridMap};

mod light_tile;

#[derive(Clone)]
pub struct LightGrid {
    map: GridMap<LightTile>,
}

impl LightGrid {
    pub fn from_file(path: std::ffi::OsString) -> Self {
        let input = std::io::BufReader::new(clio::Input::new(&path).unwrap());
        let map = input
            .lines()
            .map_while(Result::ok)
            .map(|line| line.chars().filter_map(LightTile::from_char).collect())
            .collect();
        let map = GridMap::new(map);
        LightGrid { map }
    }

    pub fn energized_tiles_from_default_entry(&self) -> u64 {
        let start_coord = Coord2D::new_row_column(0, 0);
        let start_direction = Direction::East;

        self.evaluate_beam(start_coord, start_direction)
    }

    fn evaluate_beam(&self, start_coord: Coord2D<usize>, start_direction: Direction) -> u64 {
        let mut visited_map =
            GridMap::new(vec![
                vec![TraverseTile::NotVisited; self.map.col_count()];
                self.map.row_count()
            ]);

        let mut existing_beams = BTreeMap::new();
        existing_beams.insert(start_coord, vec![start_direction.clone()]);

        let mut beams = Vec::new();
        beams.push((start_coord, start_direction));

        let mut beams_to_evaluate: Vec<(Coord2D<usize>, Direction)> = Vec::new();

        while let Some((mut current_coord, mut current_direction)) = beams.pop() {
            if let Some(light_tile) = self.map.get_ref(&current_coord) {
                visited_map.set(&current_coord, TraverseTile::Visited);
                match light_tile {
                    LightTile::Empty => (),
                    LightTile::Mirror(Direction::East, Direction::North) => {
                        current_direction = match current_direction {
                            Direction::North => Direction::East,
                            Direction::East => Direction::North,
                            Direction::South => Direction::West,
                            Direction::West => Direction::South,
                        }
                    }
                    LightTile::Mirror(Direction::East, Direction::South) => {
                        current_direction = match current_direction {
                            Direction::North => Direction::West,
                            Direction::East => Direction::South,
                            Direction::South => Direction::East,
                            Direction::West => Direction::North,
                        }
                    }
                    LightTile::Splitter(Direction::North) => match current_direction {
                        Direction::East | Direction::West => {
                            current_direction = Direction::North;
                            if let Some(new_coord) =
                                current_coord.new_in_direction(&Direction::South)
                            {
                                beams_to_evaluate.push((new_coord, Direction::South));
                            }
                        }
                        _ => (),
                    },
                    LightTile::Splitter(Direction::East) => match current_direction {
                        Direction::North | Direction::South => {
                            current_direction = Direction::East;
                            if let Some(new_coord) =
                                current_coord.new_in_direction(&Direction::West)
                            {
                                beams_to_evaluate.push((new_coord, Direction::West));
                            }
                        }
                        _ => (),
                    },
                    _ => panic!("Unexpected light tile encountered"),
                }
                if current_coord.in_direction(&current_direction).is_some() {
                    match light_tile {
                        LightTile::Empty => {
                            beams.push((current_coord, current_direction));
                        }
                        _ => {
                            beams_to_evaluate.push((current_coord, current_direction));
                        }
                    }
                }
                for (coord, direction) in beams_to_evaluate.drain(..) {
                    let new_beam_contains = existing_beams.entry(coord).or_insert(Vec::new());
                    if !new_beam_contains.contains(&direction) {
                        new_beam_contains.push(direction.clone());
                        beams.push((coord, direction));
                    }
                }
            };
        }
        self.count_visited_tile_union(visited_map)
    }

    fn count_visited_tile_union(&self, visited_map: GridMap<TraverseTile>) -> u64 {
        visited_map
            .rows_iter()
            .map(|row| {
                row.filter(|tile| **tile != TraverseTile::NotVisited)
                    .count()
            })
            .sum::<usize>()
            .try_into()
            .unwrap()
    }

    pub fn max_energized_tiles(&self) -> u64 {
        let row_count = self.map.row_count();
        let col_count = self.map.col_count();
        let mut start_coords_directions = Vec::new();
        for col_index in 0..col_count {
            start_coords_directions.push((Coord2D::new_row_column(0, col_index), Direction::South));
            start_coords_directions.push((
                Coord2D::new_row_column(row_count - 1, col_index),
                Direction::North,
            ));
        }

        for row_index in 0..row_count {
            start_coords_directions.push((Coord2D::new_row_column(row_index, 0), Direction::East));
            start_coords_directions.push((
                Coord2D::new_row_column(row_index, col_count - 1),
                Direction::West,
            ));
        }

        let threads = std::thread::available_parallelism().unwrap().get();
        let (tx, rx) = std::sync::mpsc::channel();

        for chunk in start_coords_directions.chunks((start_coords_directions.len() / threads) + 1) {
            let tx = tx.clone();
            let self_clone = self.clone();
            let chunk_clone: Vec<(Coord2D<usize>, Direction)> = chunk
                .iter()
                .map(|(coord, direction)| (*coord, direction.clone()))
                .collect();
            std::thread::spawn(move || {
                for (coord, direction) in chunk_clone {
                    tx.send(self_clone.evaluate_beam(coord, direction)).unwrap();
                }
            });
        }
        drop(tx);
        rx.into_iter().max().unwrap_or_default()
    }
}

#[derive(PartialEq, Clone)]
enum TraverseTile {
    Visited,
    NotVisited,
}
