use std::collections::HashMap;

use crate::shared_libs::{coord_2_d::Coord2D, direction::Direction, grid_map::GridMap};

use super::pipe_tile::PipeTile;

pub struct PipeTileMap {
    map: GridMap<PipeTile>,
    start_position: Coord2D<usize>,
}

impl PipeTileMap {
    pub fn from_iter(string_iter: impl Iterator<Item = String>) -> Option<Self> {
        let mut tiles = Vec::new();
        let mut start_found = false;
        let mut start_position = Coord2D::new_row_column(0, 0);
        for line in string_iter {
            let mut row = Vec::new();
            for tile_char in line.chars() {
                let tile = PipeTile::from_char(tile_char);
                if !start_found && tile == PipeTile::StartPosition {
                    start_found = true;
                    start_position = Coord2D::new_row_column(tiles.len(), row.len());
                }
                row.push(tile);
            }
            tiles.push(row);
        }
        if !start_found {
            return None;
        }
        let mut map = GridMap::new(tiles);
        let start_position_tile = Self::resolve_start_position(&map, &start_position)?;
        map.set(&start_position, start_position_tile);
        Some(Self {
            map,
            start_position,
        })
    }

    fn resolve_start_position(
        tiles: &GridMap<PipeTile>,
        start_position: &Coord2D<usize>,
    ) -> Option<PipeTile> {
        let mut direction_tiles = HashMap::new();
        for direction in Direction::CARDINAL_DIRECTIONS {
            if let Some(new_coord) = start_position.in_direction(&direction) {
                if let Some(tile) = tiles.get_ref(new_coord) {
                    direction_tiles.insert(direction, tile);
                }
            }
        }
        PipeTile::NAVIGABLE_PIPES.into_iter().find(|&possible_tile| Self::check_directions_for_connections(
                &direction_tiles,
                &possible_tile,
                possible_tile.get_2_connecting_directions(),
            ))
    }

    fn check_directions_for_connections(
        direction_tiles: &HashMap<Direction, &PipeTile>,
        possible_tile: &PipeTile,
        directions: &[Direction],
    ) -> bool {
        for direction in directions {
            if let Some(direction_tile) = direction_tiles.get(direction) {
                if !possible_tile.connects(direction_tile, direction) {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    pub fn get_pipe_loop_coords_from_start(&self) -> Vec<Coord2D<usize>> {
        let mut pipe_coords = Vec::new();
        let mut current_position = self.start_position;
        pipe_coords.push(current_position);
        let mut current_tile = self.map.get_ref(self.start_position).unwrap();
        let mut next_direction = &current_tile.get_2_connecting_directions()[0];
        loop {
            current_position = current_position
                .in_direction(next_direction)
                .unwrap_or_else(|| panic!("Could not navigate from {:?}", current_position));
            current_tile = self.map.get_ref(current_position).unwrap();
            if current_tile.get_2_connecting_directions()[0] == next_direction.reverse() {
                next_direction = &current_tile.get_2_connecting_directions()[1];
            } else {
                next_direction = &current_tile.get_2_connecting_directions()[0];
            }
            if current_position == self.start_position {
                break;
            } else {
                pipe_coords.push(current_position);
            }
        }
        pipe_coords
    }

    fn copy_main_loop_to_new_map(&self) -> GridMap<PipeTile> {
        let rows = self.map.row_count();
        let cols = self.map.col_count();
        let mut tiles = Vec::with_capacity(rows);
        let mut row = Vec::with_capacity(cols);
        for _ in 0..cols {
            row.push(PipeTile::Ground);
        }
        for _ in 0..rows {
            tiles.push(row.clone());
        }
        let mut new_map = GridMap::new(tiles);
        let main_loop_coords = self.get_pipe_loop_coords_from_start();
        for coord in main_loop_coords {
            new_map.set(&coord, *self.map.get_ref(coord).unwrap());
        }
        new_map
    }

    pub fn get_enclosed_tile_count(&self) -> u64 {
        let main_loop_only_tile_map = self.copy_main_loop_to_new_map();
        let mut enlarged_tile_map = Self::enlarge(&main_loop_only_tile_map);
        Self::transform_border_tiles_away_from_ground(&mut enlarged_tile_map);

        let mut enclosed_tile_count = 0;
        let rows = main_loop_only_tile_map.row_count();
        let cols = main_loop_only_tile_map.col_count();
        for row in 0..rows {
            for col in 0..cols {
                if *main_loop_only_tile_map
                    .get_ref(Coord2D::new_row_column(row, col))
                    .unwrap()
                    == PipeTile::Ground
                {
                    let translated_row = 1 + row * 3;
                    let translated_col = 1 + col * 3;
                    if *enlarged_tile_map
                        .get_ref(Coord2D::new_row_column(translated_row, translated_col))
                        .unwrap()
                        == PipeTile::Ground
                    {
                        enclosed_tile_count += 1;
                    }
                }
            }
        }

        enclosed_tile_count
    }

    fn enlarge(map: &GridMap<PipeTile>) -> GridMap<PipeTile> {
        let mut enlarged_map = Vec::new();
        let rows_iter = map.rows_iter();
        for mut col_iter in rows_iter {
            let mut current_rows = vec![Vec::new(), Vec::new(), Vec::new()];
            for tile in col_iter.by_ref() {
                tile.append_enlarged_tile(&mut current_rows);
            }
            enlarged_map.append(&mut current_rows);
        }
        GridMap::new(enlarged_map)
    }

    fn transform_border_tiles_away_from_ground(enlarged_tile_map: &mut GridMap<PipeTile>) {
        let mut tiles_to_check = Vec::new();
        let row_count = enlarged_tile_map.row_count();
        let col_count = enlarged_tile_map.col_count();
        for col in 0..col_count {
            tiles_to_check.push(Coord2D::new_row_column(0, col));
            tiles_to_check.push(Coord2D::new_row_column(row_count - 1, col));
        }
        for row in 0..row_count {
            tiles_to_check.push(Coord2D::new_row_column(row, 0));
            tiles_to_check.push(Coord2D::new_row_column(row, col_count - 1));
        }
        while let Some(coord_to_check) = tiles_to_check.pop() {
            let current_tile = enlarged_tile_map.get_ref(coord_to_check).unwrap();
            if *current_tile == PipeTile::Ground {
                enlarged_tile_map.set(&coord_to_check, PipeTile::StartPosition);
                for direction in Direction::CARDINAL_DIRECTIONS {
                    if let Some(coord_in_direction) = coord_to_check.in_direction(&direction) {
                        if let Some(tile_in_direction) =
                            enlarged_tile_map.get_ref(coord_in_direction)
                        {
                            if *tile_in_direction == PipeTile::Ground {
                                tiles_to_check.push(coord_in_direction);
                            }
                        }
                    }
                }
            }
        }
    }
}
