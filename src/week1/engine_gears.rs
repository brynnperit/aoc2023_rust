use std::{
    io::BufRead,
    ops::{Add, Sub},
};

use crate::shared_libs::coord_2_d::Coord2D;

pub fn get_all_part_numbers_from_path(path: std::ffi::OsString) -> Vec<i32> {
    get_all_part_numbers_from_input(clio::Input::new(&path).unwrap())
}

pub fn get_all_part_numbers_from_input(input: clio::Input) -> Vec<i32> {
    let mut part_numbers = Vec::new();
    let schematic = get_schematic_from_input(input);
    for part_row in &schematic.parts {
        for part in part_row {
            if schematic.part_has_any_adjacent_symbol(part) {
                part_numbers.push(part.value.parse::<i32>().unwrap())
            }
        }
    }
    part_numbers
}

fn get_schematic_from_input(input: clio::Input) -> EngineSchematic {
    let mut schematic = EngineSchematic::new();
    let input = std::io::BufReader::new(input);
    for line in input.lines().map(|line| line.unwrap()) {
        schematic.add_from_line(&line);
    }
    schematic
}

pub fn get_all_gear_ratios_from_path(path: std::ffi::OsString) -> Vec<i32> {
    get_all_gear_ratios_from_input(clio::Input::new(&path).unwrap())
}

pub fn get_all_gear_ratios_from_input(input: clio::Input) -> Vec<i32> {
    let mut gear_ratios = Vec::new();
    let schematic = get_schematic_from_input(input);
    for symbol_row in &schematic.symbols {
        for symbol in symbol_row {
            if symbol.value == '*'{
                if let Some(part_tuple) = schematic.get_two_adjacent_numbers_for_symbol(symbol) {
                    gear_ratios.push(part_tuple.0 * part_tuple.1);
                }
            }
        }
    }
    gear_ratios
}

pub struct EngineSchematic {
    parts: Vec<Vec<ValueAtCoord2D<String>>>,
    symbols: Vec<Vec<ValueAtCoord2D<char>>>,
}

impl EngineSchematic {
    const fn new() -> EngineSchematic {
        EngineSchematic {
            parts: Vec::new(),
            symbols: Vec::new(),
        }
    }

    fn add_from_line(&mut self, line: &str) {
        let coord_y = self.parts.len();
        let mut part_row = Vec::new();
        let mut symbol_row = Vec::new();
        let mut part_string = String::with_capacity(3);
        let mut coord_x = 0;
        for character in line.chars() {
            match character {
                '0'..='9' => part_string.push(character),
                _ => {
                    part_string =
                        move_part_to_row_if_non_empty(&mut part_row, part_string, coord_x, coord_y);
                    match character {
                        '.' => (),
                        _ => symbol_row.push(ValueAtCoord2D::new(character, coord_x, coord_y)),
                    }
                }
            }
            coord_x += 1;
        }
        move_part_to_row_if_non_empty(&mut part_row, part_string, coord_x, coord_y);
        self.parts.push(part_row);
        self.symbols.push(symbol_row);
    }

    fn part_has_any_adjacent_symbol(&self, part: &ValueAtCoord2D<String>) -> bool {
        for row_index in part.coord.get_y().saturating_sub(1)..=part.coord.get_y() + 1 {
            if let Some(row) = self.symbols.get(row_index) {
                for symbol in row {
                    if is_symbol_adjacent_to_part(part, symbol) {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn get_two_adjacent_numbers_for_symbol(
        &self,
        part: &ValueAtCoord2D<char>,
    ) -> Option<(i32, i32)> {
        let adjacent_numbers = self.get_adjacent_numbers_for_symbol(part, 2);
        if adjacent_numbers.len() == 2 {
            return Some((
                *adjacent_numbers.first().unwrap(),
                *adjacent_numbers.get(1).unwrap(),
            ));
        }
        None
    }

    fn get_adjacent_numbers_for_symbol(
        &self,
        symbol: &ValueAtCoord2D<char>,
        number_limit: usize,
    ) -> Vec<i32> {
        let mut adjacent_numbers = Vec::new();
        for part_row in &self.parts {
            for check_part in part_row {
                if check_part.coord.get_y().abs_diff(symbol.coord.get_y()) <= 1
                    && is_symbol_adjacent_to_part(check_part, symbol)
                {
                    adjacent_numbers.push(check_part.value.parse::<i32>().unwrap());
                    if adjacent_numbers.len() >= number_limit {
                        return adjacent_numbers;
                    }
                }
            }
        }
        adjacent_numbers
    }
}

fn move_part_to_row_if_non_empty(
    part_row: &mut Vec<ValueAtCoord2D<String>>,
    part_string: String,
    coord_x: usize,
    coord_y: usize,
) -> String {
    if !part_string.is_empty() {
        let string_start_x = coord_x - part_string.len();
        part_row.push(ValueAtCoord2D::new(part_string, string_start_x, coord_y));
        String::with_capacity(3)
    } else {
        part_string
    }
}

fn is_symbol_adjacent_to_part(
    part: &ValueAtCoord2D<String>,
    symbol: &ValueAtCoord2D<char>,
) -> bool {
    let start_is_adjacent = part.coord.get_x().abs_diff(symbol.coord.get_x()) <= 1;
    let starts_before_and_ends_adjacent_to_or_after = part.coord.get_x() < symbol.coord.get_x()
        && part.coord.get_x().add(part.value.len() - 1) >= symbol.coord.get_x().sub(1);
    start_is_adjacent || starts_before_and_ends_adjacent_to_or_after
}

pub struct ValueAtCoord2D<T> {
    value: T,
    coord: Coord2D<usize>,
}

impl<T> ValueAtCoord2D<T> {
    const fn new(value: T, coord_x: usize, coord_y: usize) -> ValueAtCoord2D<T> {
        ValueAtCoord2D {
            value,
            coord: Coord2D::new(coord_x, coord_y),
        }
    }
}
