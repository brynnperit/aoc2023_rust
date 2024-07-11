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
        let mut row_index = 0;
        for line in string_iter {
            if row_status.len() < row_index + 1 {
                row_status.push(SectorStatus::Empty);
            }
            let mut col_index = 0;
            for tile_char in line.chars() {
                let tile = GalaxyTile::from_char(tile_char).expect(&format!(
                    "The character {} was unexpected in this line: {}",
                    tile_char, line
                ));
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
                col_index += 1;
            }
            row_index += 1;
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
        for first_index in 0..self.galaxies.len() {
            let first_galaxy = self.galaxies[first_index];
            for second_index in first_index + 1..self.galaxies.len() {
                let second_galaxy = self.galaxies[second_index];
                all_distances.push((
                    first_galaxy,
                    second_galaxy,
                    self.get_galaxy_distance(
                        first_index + 1,
                        &first_galaxy,
                        second_index + 1,
                        &second_galaxy,
                        &factor,
                    ),
                ));
            }
        }
        // let mut first_iter = self.galaxies.iter();
        // while let Some(first_galaxy) = first_iter.next() {
        //     let mut second_iter = first_iter.clone();
        //     while let Some(second_galaxy) = second_iter.next() {
        //         all_distances.push((
        //             first_galaxy.clone(),
        //             second_galaxy.clone(),
        //             self.get_galaxy_distance(first_galaxy, second_galaxy, &factor),
        //         ));
        //     }
        // }
        all_distances
    }

    fn get_galaxy_distance(
        &self,
        first_index: usize,
        first: &Coord2D<usize>,
        second_index: usize,
        second: &Coord2D<usize>,
        factor: &ExpansionFactor,
    ) -> u64 {
        if first_index == 5 && second_index == 9 {
            print!("break");
        }
        let row_distance =
            Self::get_distance(first.get_row(), second.get_row(), &self.row_status, factor);
        let col_distance =
            Self::get_distance(first.get_col(), second.get_col(), &self.col_status, factor);
        row_distance + col_distance
    }

    fn get_distance(
        first_index: usize,
        second_index: usize,
        status_vector: &Vec<SectorStatus>,
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
