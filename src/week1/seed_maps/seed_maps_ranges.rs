use std::io::BufRead;

use super::source_target_map::{self, SourceTargetMap};

pub fn get_location_numbers_from_seed_input_as_ranges(input: clio::Input) -> Vec<usize> {
    let input = std::io::BufReader::new(input);
    let mut line_iter = input.lines();
    let seed_line = line_iter.next().unwrap().unwrap();
    let mut seed_line_iter = seed_line.split_ascii_whitespace();
    seed_line_iter.next();
    let mut location_numbers = Vec::new();
    while let Ok(seed_number) = seed_line_iter.next().unwrap_or("").parse::<usize>() {
        location_numbers.push(seed_number);
    }
    let mut seed_ranges = Vec::new();
    while location_numbers.len() > 1 {
        let seed_range_length = location_numbers.pop().unwrap();
        let seed_range_start = location_numbers.pop().unwrap();
        seed_ranges.push(SeedRange::new(seed_range_start, seed_range_length));
    }
    let mut map_sets = Vec::new();
    while let Some(Ok(line)) = line_iter.next() {
        match line.as_str() {
            "" => (),
            _ => map_sets.push(source_target_map::get_maps(&mut line_iter)),
        }
    }
    let mut next_ranges = Vec::with_capacity(seed_ranges.len());
    let mut skipped_ranges = Vec::with_capacity(seed_ranges.len());
    let mut applicable_seed_ranges = Vec::new();
    for map_set in map_sets.as_slice() {
        for map in map_set {
            for seed_range in seed_ranges.drain(..) {
                match seed_range.split_seed_range(map) {
                    Some(mut vector) => applicable_seed_ranges.append(&mut vector),
                    None => skipped_ranges.push(seed_range),
                }
            }
            for mut seed_range in applicable_seed_ranges.drain(..) {
                match map.map_number(seed_range.range_start) {
                    Some(mapped_number) => {
                        seed_range.range_start = mapped_number;
                        next_ranges.push(seed_range);
                    }
                    _ => skipped_ranges.push(seed_range),
                }
            }
            seed_ranges.append(&mut skipped_ranges);
        }
        next_ranges.append(&mut seed_ranges);
        seed_ranges.clear();
        seed_ranges.append(&mut next_ranges);
    }
    seed_ranges
        .iter()
        .map(|seed_range| seed_range.range_start)
        .collect::<Vec<_>>()
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
                return Some(vec![SeedRange::new(self.range_start,self.range_length)]);
            }
        }
        None
    }
}
