use std::io::BufRead;

use super::source_target_map;

pub fn get_location_numbers_from_seed_input(input: clio::Input) -> Vec<usize> {
    let input = std::io::BufReader::new(input);
    let mut line_iter = input.lines();
    let seed_line = line_iter.next().unwrap().unwrap();
    let mut seed_line_iter = seed_line.split_ascii_whitespace();
    seed_line_iter.next();
    let mut location_numbers = Vec::new();
    while let Ok(seed_number) = seed_line_iter.next().unwrap_or("").parse::<usize>() {
        location_numbers.push(seed_number);
    }
    let mut map_sets = Vec::new();
    while let Some(Ok(line)) = line_iter.next() {
        match line.as_str() {
            "" => (),
            _ => map_sets.push(source_target_map::get_maps(&mut line_iter)),
        }
    }
    for map_set in map_sets.as_slice() {
        for seed_index in 0..location_numbers.len() {
            for map in map_set {
                match map.map_number(location_numbers[seed_index]) {
                    Some(mapped_number) => {
                        location_numbers[seed_index] = mapped_number;
                        break;
                    }
                    _ => (),
                }
            }
        }
    }
    location_numbers
}