use grid_tile::GridTile;

use crate::shared_libs::grid_map::GridMap;

use super::{ReflectionResult, ReflectionType};

mod grid_tile;

pub struct ReflectionGrid {
    grid: GridMap<GridTile>,
}

impl ReflectionGrid {
    pub fn from_iter(iter: &mut impl Iterator<Item = String>) -> Option<Self> {
        let mut grid_tiles = Vec::new();
        for line in iter {
            match line.as_str() {
                "" => break,
                _ => grid_tiles.push(line.chars().filter_map(GridTile::from_char).collect()),
            }
        }
        if !grid_tiles.is_empty() {
            return Some(ReflectionGrid {
                grid: GridMap::new(grid_tiles),
            });
        }
        None
    }

    pub fn get_reflection_result(&self) -> Option<ReflectionResult> {
        let row_result = check_lines(
            self.grid.rows_iter().map(Self::get_line_hash).collect(),
            ReflectionType::HorizontalLine,
        );
        if row_result.is_some() {
            return row_result;
        }
        let col_result = check_lines(
            self.grid.cols_iter().map(Self::get_line_hash).collect(),
            ReflectionType::VerticalLine,
        );
        if col_result.is_some() {
            return col_result;
        }
        None
    }

    fn get_line_hash<'a>(line: impl Iterator<Item = &'a GridTile>) -> u64 {
        let mut hash_value = 0;
        for tile in line {
            if *tile == GridTile::Ash {
                hash_value += 1;
            }
            hash_value *= 2;
        }
        hash_value
    }

    pub fn get_alternate_reflection_result(&self) -> Option<ReflectionResult> {
        let row_result =
            check_lines_alternate(self.grid.rows_iter(), ReflectionType::HorizontalLine);
        if row_result.is_some() {
            return row_result;
        }
        let col_result = check_lines_alternate(self.grid.cols_iter(), ReflectionType::VerticalLine);
        if col_result.is_some() {
            return col_result;
        }
        None
    }
}

fn check_lines_alternate<'a>(
    lines: impl Iterator<Item = impl Iterator<Item = &'a GridTile>>,
    reflection: ReflectionType,
) -> Option<ReflectionResult> {
    let mut all_lines: Vec<Vec<&GridTile>> = Vec::new();
    for line in lines {
        all_lines.push(line.collect());
    }
    for test_index in 1..all_lines.len() {
        let mut cumulative_differences = 0;
        for (low_index, high_index) in (0..=test_index.saturating_sub(1))
            .rev()
            .zip(test_index..all_lines.len())
        {
            cumulative_differences +=
                get_count_of_differences(&all_lines[low_index], &all_lines[high_index]);
            if cumulative_differences > 1 {
                break;
            }
        }
        if cumulative_differences == 1 {
            return Some(ReflectionResult {
                reflection_type: reflection,
                lines_to_left_or_above_reflection: test_index.try_into().unwrap(),
            });
        }
    }
    None
}

fn get_count_of_differences(first_line: &[&GridTile], second_line: &[&GridTile]) -> u64 {
    first_line
        .iter()
        .zip(second_line.iter())
        .filter(|(first, second)| first != second)
        .count()
        .try_into()
        .unwrap()
}

fn check_lines(line_hashes: Vec<u64>, reflection: ReflectionType) -> Option<ReflectionResult> {
    let mut matching_indices = Vec::new();
    for line_index in 1..line_hashes.len() {
        if line_hashes[line_index] == line_hashes[line_index - 1] {
            matching_indices.push(line_index);
        }
    }
    for matching_index in matching_indices {
        let mut all_indices_match = true;
        for (low_index, high_index) in (0..=matching_index.saturating_sub(1))
            .rev()
            .zip(matching_index..line_hashes.len())
        {
            if line_hashes[low_index] != line_hashes[high_index] {
                all_indices_match = false;
                break;
            }
        }
        if all_indices_match {
            return Some(ReflectionResult {
                reflection_type: reflection,
                lines_to_left_or_above_reflection: matching_index.try_into().unwrap(),
            });
        }
    }
    None
}
