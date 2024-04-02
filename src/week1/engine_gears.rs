use std::{
    cmp::min,
    io::BufRead,
    ops::{Add, Sub},
};

pub fn get_all_part_numbers_from_path(path: std::ffi::OsString) -> Vec<i32> {
    get_all_part_numbers_from_input(clio::Input::new(&path).unwrap())
}

pub fn get_all_part_numbers_from_input(input: clio::Input) -> Vec<i32> {
    let mut part_numbers = Vec::new();
    let schematic = get_schematic_from_input(input);
    for part in &schematic.parts {
        if schematic.part_has_any_adjacent_symbol(part) {
            part_numbers.push(part.value.parse::<i32>().unwrap())
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
    for symbol in &schematic.symbols {
        match symbol.value {
            '*' => match schematic.get_two_adjacent_numbers_for_symbol(symbol) {
                Some(part_tuple) => gear_ratios.push(part_tuple.0 * part_tuple.1),
                None => (),
            },
            _ => (),
        }
    }
    gear_ratios
}

pub struct EngineSchematic {
    parts: Vec<ValueAtCoord2D<String>>,
    symbols: Vec<ValueAtCoord2D<char>>,
    grid: EngineGrid,
}

impl EngineSchematic {
    const fn new() -> EngineSchematic {
        EngineSchematic {
            parts: Vec::new(),
            symbols: Vec::new(),
            grid: EngineGrid::new(),
        }
    }

    fn add_part(&mut self, part_number: String, coord_x: usize, coord_y: usize) {
        self.parts
            .push(ValueAtCoord2D::new(part_number, coord_x, coord_y));
    }

    fn add_symbol(&mut self, symbol: char, coord_x: usize, coord_y: usize) {
        self.symbols
            .push(ValueAtCoord2D::new(symbol, coord_x, coord_y))
    }

    fn add_from_line(&mut self, line: &str) {
        let coord_y = self.grid.grid_vec.len();
        let row = self.get_row_from_line(line);
        self.add_parts_and_symbols_from_row(&row, coord_y);
        self.grid.grid_vec.push(row);
    }

    fn get_row_from_line(&self, line: &str) -> Vec<EngineGridType> {
        let mut row = Vec::new();
        for character in line.chars() {
            match character {
                '0'..='9' => row.push(EngineGridType::Part(character)),
                '.' => row.push(EngineGridType::Empty),
                _ => row.push(EngineGridType::Symbol(character)),
            }
        }
        row
    }

    fn add_parts_and_symbols_from_row(&mut self, row: &Vec<EngineGridType>, coord_y: usize) {
        let mut number_string = String::with_capacity(3);
        let mut coord_x = 0;
        for grid_type in row {
            match grid_type {
                EngineGridType::Part(part_char) => number_string.push(*part_char),
                _ => {
                    number_string =
                        self.add_part_if_string_is_non_empty(number_string, coord_x, coord_y);
                    match grid_type {
                        EngineGridType::Symbol(symbol) => {
                            self.add_symbol(*symbol, coord_x, coord_y)
                        }
                        _ => (),
                    }
                }
            }
            coord_x += 1;
        }
        self.add_part_if_string_is_non_empty(number_string, coord_x, coord_y);
    }

    fn add_part_if_string_is_non_empty(
        &mut self,
        number_string: String,
        coord_x: usize,
        coord_y: usize,
    ) -> String {
        if !number_string.is_empty() {
            let string_start_x = coord_x - number_string.len();
            self.add_part(number_string, string_start_x, coord_y);
            String::with_capacity(3)
        } else {
            number_string
        }
    }

    fn part_has_any_adjacent_symbol(&self, part: &ValueAtCoord2D<String>) -> bool {
        let part_string_length = part.value.len();
        let check_start_x = part.coord.x.checked_sub(1).unwrap_or(0);
        let check_end_x = part.coord.x + part_string_length;
        for row_index in part.coord.y.checked_sub(1).unwrap_or(0)..=part.coord.y + 1 {
            match self.grid.grid_vec.get(row_index) {
                Some(row) => {
                    let check_end_x = min(check_end_x, row.len() - 1);
                    for check_position in &row[check_start_x..=check_end_x] {
                        match check_position {
                            EngineGridType::Symbol(_) => return true,
                            _ => (),
                        }
                    }
                }
                None => (),
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
                *adjacent_numbers.get(0).unwrap(),
                *adjacent_numbers.get(1).unwrap(),
            ));
        }
        None
    }

    fn get_adjacent_numbers_for_symbol(
        &self,
        part: &ValueAtCoord2D<char>,
        number_limit: usize,
    ) -> Vec<i32> {
        let mut adjacent_numbers = Vec::new();
        for check_part in &self.parts {
            if check_part.coord.y.abs_diff(part.coord.y) <= 1 {
                let start_is_adjacent = check_part.coord.x.abs_diff(part.coord.x) <= 1;
                let starts_before_and_ends_adjacent_to_or_after = check_part.coord.x < part.coord.x
                    && check_part.coord.x.add(check_part.value.len() - 1) >= part.coord.x.sub(1);

                if start_is_adjacent || starts_before_and_ends_adjacent_to_or_after {
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

enum EngineGridType {
    Empty,
    Part(char),
    Symbol(char),
}

pub struct EngineGrid {
    grid_vec: Vec<Vec<EngineGridType>>,
}

impl EngineGrid {
    const fn new() -> EngineGrid {
        EngineGrid {
            grid_vec: Vec::new(),
        }
    }
}

pub struct ValueAtCoord2D<T> {
    value: T,
    coord: Coord2D,
}

impl<T> ValueAtCoord2D<T> {
    const fn new(value: T, coord_x: usize, coord_y: usize) -> ValueAtCoord2D<T> {
        ValueAtCoord2D {
            value,
            coord: Coord2D::new(coord_x, coord_y),
        }
    }
}

pub struct Coord2D {
    x: usize,
    y: usize,
}

impl Coord2D {
    const fn new(x: usize, y: usize) -> Coord2D {
        Coord2D { x, y }
    }
}
