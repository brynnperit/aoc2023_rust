use std::io::BufRead;

use super::source_target_map::{self, SourceTargetMap};

pub fn get_location_numbers_from_seed_input_as_ranges(input: clio::Input) -> Vec<usize> {
    let mut location_numbers = Vec::new();
    let mut map_sets = Vec::new();

    parse_input(input, &mut location_numbers, &mut map_sets);

    let seed_ranges = get_seed_ranges(location_numbers, map_sets);
    seed_ranges
        .iter()
        .map(|seed_range| seed_range.range_start)
        .collect::<Vec<_>>()
}

fn get_seed_ranges(
    location_numbers: Vec<usize>,
    map_sets: Vec<Vec<SourceTargetMap>>,
) -> Vec<SeedRange> {
    let mut first_ranges = transform_locations_to_ranges(location_numbers);

    let mut second_ranges = Vec::with_capacity(first_ranges.len());
    let mut third_ranges = Vec::with_capacity(first_ranges.len());

    let mut current_ranges_ref = &mut first_ranges;
    let mut skipped_ranges_ref = &mut second_ranges;
    let mut next_ranges_ref = &mut third_ranges;

    let mut applicable_seed_ranges = Vec::new();
    for map_set in map_sets {
        for map in map_set {
            for seed_range in current_ranges_ref.drain(..) {
                match seed_range.split_seed_range(&map) {
                    Some(mut vector) => applicable_seed_ranges.append(&mut vector),
                    None => skipped_ranges_ref.push(seed_range),
                }
            }
            for mut seed_range in applicable_seed_ranges.drain(..) {
                match map.map_number(seed_range.range_start) {
                    Some(mapped_number) => {
                        seed_range.range_start = mapped_number;
                        next_ranges_ref.push(seed_range);
                    }
                    _ => skipped_ranges_ref.push(seed_range),
                }
            }
            (current_ranges_ref, skipped_ranges_ref) = (skipped_ranges_ref, current_ranges_ref);
        }
        next_ranges_ref.append(current_ranges_ref);
        (next_ranges_ref, current_ranges_ref) = (current_ranges_ref, next_ranges_ref);
    }
    if !first_ranges.is_empty() {
        first_ranges
    } else if !second_ranges.is_empty() {
        second_ranges
    } else {
        third_ranges
    }
}

fn transform_locations_to_ranges(location_numbers: Vec<usize>) -> Vec<SeedRange> {
    let mut seed_ranges = Vec::new();
    let mut location_iter = location_numbers.into_iter();
    while let Some(seed_range_start) = location_iter.next() {
        if let Some(seed_range_length) = location_iter.next() {
            seed_ranges.push(SeedRange::new(seed_range_start, seed_range_length))
        }
    }
    seed_ranges
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

struct SeedRange {
    range_start: usize,
    range_length: usize,
}

impl SeedRange {
    fn new(range_start: usize, range_length: usize) -> Self {
        Self {
            range_start,
            range_length,
        }
    }

    pub fn split_seed_range(&self, map: &SourceTargetMap) -> Option<Vec<SeedRange>> {
        self.split_seed_range_by_numbers(map.get_source_range_start(), map.get_range_length())
    }

    fn split_seed_range_by_numbers(
        &self,
        map_range_start: usize,
        map_range_length: usize,
    ) -> Option<Vec<SeedRange>> {
        if map_range_start < self.range_start
            && map_range_start + map_range_length >= self.range_start
        {
            return self.split_seed_range_by_numbers(
                self.range_start,
                map_range_length - (self.range_start - map_range_start),
            );
        }
        if map_range_start > self.range_start
            && map_range_start < self.range_start + self.range_length
        {
            let first_seed_range_length = map_range_start - self.range_start;
            let first_seed_range = SeedRange::new(self.range_start, first_seed_range_length);
            let mut seed_range_vector = vec![first_seed_range];

            let leftover_seed_range_length = self.range_length - first_seed_range_length;
            let second_seed_range = SeedRange::new(map_range_start, leftover_seed_range_length);
            let split_result =
                second_seed_range.split_seed_range_by_numbers(map_range_start, map_range_length);
            match split_result {
                Some(mut range_vector) => seed_range_vector.append(&mut range_vector),
                None => seed_range_vector.push(second_seed_range),
            }
            return Some(seed_range_vector);
        }
        if map_range_start == self.range_start {
            if map_range_length < self.range_length {
                let leftover_length = self.range_length - map_range_length;
                let first_range = SeedRange::new(map_range_start, map_range_length);
                let second_range =
                    SeedRange::new(map_range_start + map_range_length, leftover_length);
                return Some(vec![first_range, second_range]);
            } else {
                return Some(vec![SeedRange::new(self.range_start, self.range_length)]);
            }
        }
        None
    }
}
