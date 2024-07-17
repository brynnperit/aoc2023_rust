use crate::shared_libs::coord_2_d::Coord2D;

use super::ExpansionFactor;

pub struct GalaxyMap {
    galaxies: Vec<Coord2D<usize>>,
    row_status: Vec<SectorStatus>,
    col_status: Vec<SectorStatus>,
}

impl GalaxyMap {
    pub fn from_input(string_iter: impl Iterator<Item = String>) -> Self {
        let mut galaxies = Vec::new();
        let mut row_status = Vec::new();
        let mut col_status = Vec::new();
        for (row_index,line) in string_iter.enumerate() {
            row_status.push(SectorStatus::Empty);
            for (col_index,tile_char) in line.chars().enumerate() {
                let tile = GalaxyTile::from_char(tile_char).unwrap_or_else(|| panic!("The character {} was unexpected in this line: {}",
                    tile_char, line));
                if col_status.len() < col_index + 1 {
                    col_status.push(SectorStatus::Empty);
                }
                match tile {
                    GalaxyTile::Galaxy => {
                        row_status[row_index] = SectorStatus::NonEmpty;
                        col_status[col_index] = SectorStatus::NonEmpty;
                        galaxies.push(Coord2D::new_row_column(row_index, col_index));
                    }
                    GalaxyTile::Space => (),
                }
            }
        }
        GalaxyMap {
            galaxies,
            row_status,
            col_status,
        }
    }

    pub fn distances_between_all_pairs(
        &self,
        factor: ExpansionFactor,
    ) -> Vec<(Coord2D<usize>, Coord2D<usize>, u64)> {
        let mut all_distances = Vec::new();
        let mut first_iter = self.galaxies.iter();
        while let Some(first_galaxy) = first_iter.next() {
            let second_iter = first_iter.clone();
            for second_galaxy in second_iter {
                all_distances.push((
                    *first_galaxy,
                    *second_galaxy,
                    self.get_galaxy_distance(first_galaxy, second_galaxy, &factor),
                ));
            }
        }
        all_distances
    }

    fn get_galaxy_distance(
        &self,
        first: &Coord2D<usize>,
        second: &Coord2D<usize>,
        factor: &ExpansionFactor,
    ) -> u64 {
        let row_distance =
            Self::get_distance(first.get_row(), second.get_row(), &self.row_status, factor);
        let col_distance =
            Self::get_distance(first.get_col(), second.get_col(), &self.col_status, factor);
        row_distance + col_distance
    }

    fn get_distance(
        first_index: usize,
        second_index: usize,
        status_vector: &[SectorStatus],
        factor: &ExpansionFactor,
    ) -> u64 {
        let mut distance: u64 = first_index.abs_diff(second_index).try_into().unwrap();

        let empty_count: u64 = status_vector
            [first_index.min(second_index)..first_index.max(second_index)]
            .iter()
            .filter(|status| **status == SectorStatus::Empty)
            .count()
            .try_into()
            .unwrap();

        distance -= empty_count;
        match factor {
            ExpansionFactor::Multiply(multiple) => {
                distance += empty_count * multiple;
            }
        }
        distance
    }
}

#[derive(PartialEq, Eq)]
enum SectorStatus {
    Empty,
    NonEmpty,
}

enum GalaxyTile {
    Space,
    Galaxy,
}

impl GalaxyTile {
    pub const fn from_char(tile_char: char) -> Option<GalaxyTile> {
        match tile_char {
            '#' => Some(GalaxyTile::Galaxy),
            '.' => Some(GalaxyTile::Space),
            _ => None,
        }
    }
}
