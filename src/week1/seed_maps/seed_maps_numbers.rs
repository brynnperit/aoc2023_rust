use std::io::BufRead;

use super::source_target_map::{self, SourceTargetMap};

pub fn get_location_numbers_from_seed_input(input: clio::Input) -> Vec<usize> {
    let mut location_numbers = Vec::new();
    let mut map_sets = Vec::new();

    parse_input(input, &mut location_numbers, &mut map_sets);

    for map_set in map_sets.as_slice() {
        for location_number in location_numbers.iter_mut() {
            for map in map_set {
                if let Some(mapped_number) = map.map_number(*location_number) {
                    *location_number = mapped_number;
                    break;
                }
            }
        }
    }
    location_numbers
}

fn parse_input(
    input: clio::Input,
    location_numbers: &mut Vec<usize>,
    map_sets: &mut Vec<Vec<SourceTargetMap>>,
) {
    let input = std::io::BufReader::new(input);
    let mut line_iter = input.lines();
    let seed_line = line_iter.next().unwrap().unwrap();
    let mut seed_line_iter = seed_line.split_ascii_whitespace();
    seed_line_iter.next();
    while let Ok(seed_number) = seed_line_iter.next().unwrap_or("").parse::<usize>() {
        location_numbers.push(seed_number);
    }

    while let Some(Ok(line)) = line_iter.next() {
        match line.as_str() {
            "" => (),
            _ => map_sets.push(source_target_map::get_maps(&mut line_iter)),
        }
    }
}
