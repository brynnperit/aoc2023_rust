use std::collections::{BTreeSet, HashMap, HashSet};

use super::Repetition;

pub struct SpringArrangement {
    springs: Vec<SpringSymbol>,
    damaged_section_sizes: Vec<usize>,
    possible_positions: HashMap<usize, BTreeSet<usize>>,
    damaged_positions: BTreeSet<usize>,
}

impl SpringArrangement {
    pub fn from_str(line: &str, repeats: &Repetition) -> Self {
        let mut split_line = line.split_ascii_whitespace();
        let spring_section = split_line
            .next()
            .unwrap_or_else(|| panic!("There should be a spring section in this line: {}", line));
        let mut springs = Vec::new();
        for spring_character in spring_section.chars() {
            springs.push(
                SpringSymbol::from_char(spring_character).unwrap_or_else(|| {
                    panic!(
                        "The spring symbol {} in the line {} is not a recognized spring symbol",
                        spring_character, line
                    )
                }),
            )
        }
        let record_section = split_line
            .next()
            .unwrap_or_else(|| panic!("There should be a record section in this line: {}", line));
        let mut damaged_section_sizes = Vec::new();
        for str_size in record_section.split(',') {
            damaged_section_sizes.push(str_size.parse().unwrap_or_else(|_| {
                panic!(
                    "{} should have been a parseable number in {}",
                    str_size, line
                )
            }));
        }

        match repeats {
            Repetition::LineRepetitions(repeats) => {
                let springs_original = springs.clone();
                let damaged_section_sizes_original = damaged_section_sizes.clone();
                for _ in 0..(*repeats - 1) {
                    springs.push(SpringSymbol::Unknown);
                    springs.extend(springs_original.iter());
                    damaged_section_sizes.extend(damaged_section_sizes_original.iter());
                }
            }
            Repetition::None => (),
        }

        let mut possible_positions = HashMap::new();
        for size in damaged_section_sizes
            .iter()
            .copied()
            .collect::<HashSet<usize>>()
            .into_iter()
        {
            let positions = springs
                .iter()
                .enumerate()
                .filter(|(p, _)| Self::section_fits(&springs, size, *p))
                .map(|(p, _)| p)
                .collect();
            possible_positions.insert(size, positions);
        }

        let damaged_positions = springs
            .iter()
            .enumerate()
            .filter(|(_, s)| **s == SpringSymbol::Damaged)
            .map(|(p, _)| p)
            .collect();

        Self {
            springs,
            damaged_section_sizes,
            possible_positions,
            damaged_positions,
        }
    }

    pub fn get_number_of_possible_arrangements(&self) -> u64 {
        let mut arrangement_caches: Vec<HashMap<usize, u64>> = Vec::new();
        for _ in 0..self.damaged_section_sizes.len() {
            arrangement_caches.push(HashMap::new());
        }
        self.get_possible_arrangements_for_section(0, 0, &mut arrangement_caches)
            .unwrap_or(0)
    }

    fn get_possible_arrangements_for_section(
        &self,
        section_index: usize,
        minimum_section_position: usize,
        arrangement_caches: &mut Vec<HashMap<usize, u64>>,
    ) -> Option<u64> {
        let mut possible_arrangements = 0;
        let section_in_spring_bounds = minimum_section_position < self.springs.len();
        let section_index_in_bounds = section_index < self.damaged_section_sizes.len();
        if section_in_spring_bounds && section_index_in_bounds {
            let section_size = self.damaged_section_sizes[section_index];
            let position_cache = self.possible_positions.get(&section_size).unwrap();
            let minimum_valid_section_position =
                position_cache.range(minimum_section_position..).next()?;
            let maximum_section_position = self
                .damaged_positions
                .range(minimum_section_position..)
                .next()
                .unwrap_or_else(|| position_cache.last().unwrap());
            if *minimum_valid_section_position <= *maximum_section_position {
                if let Some(cached_value) =
                    arrangement_caches[section_index].get(minimum_valid_section_position)
                {
                    return Some(*cached_value);
                }
                for spring_position in
                    position_cache.range(minimum_valid_section_position..=maximum_section_position)
                {
                    let minimum_next_spring_index = spring_position + section_size + 1;
                    let additional_arrangements = self
                        .get_possible_arrangements_for_section(
                            section_index + 1,
                            minimum_next_spring_index,
                            arrangement_caches,
                        )
                        .unwrap_or_default();
                    possible_arrangements += additional_arrangements;
                }
                arrangement_caches[section_index]
                    .insert(*minimum_valid_section_position, possible_arrangements);
                return Some(possible_arrangements);
            }
        } else if (section_in_spring_bounds
            && self
                .damaged_positions
                .range(minimum_section_position..)
                .next()
                .is_some())
            || section_index_in_bounds
        {
            return None;
        } else {
            return Some(1);
        }
        None
    }

    fn section_fits(springs: &[SpringSymbol], section_size: usize, spring_index: usize) -> bool {
        let mut matched_section_length = 0;
        for spring in &springs[spring_index..] {
            match matched_section_length.cmp(&section_size) {
                std::cmp::Ordering::Less => match spring {
                    SpringSymbol::Operational => return false,
                    SpringSymbol::Damaged | SpringSymbol::Unknown => matched_section_length += 1,
                },
                std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => match spring {
                    SpringSymbol::Operational | SpringSymbol::Unknown => return true,
                    SpringSymbol::Damaged => return false,
                },
            }
        }
        matched_section_length == section_size
    }

    #[cfg(test)]
    pub fn get_springs(&self) -> impl Iterator<Item = &SpringSymbol> {
        self.springs.iter()
    }

    #[cfg(test)]
    pub fn get_damaged_section_sizes(&self) -> impl Iterator<Item = &usize> {
        self.damaged_section_sizes.iter()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpringSymbol {
    Operational,
    Damaged,
    Unknown,
}

impl SpringSymbol {
    pub fn from_char(spring_char: char) -> Option<Self> {
        match spring_char {
            '.' => Some(SpringSymbol::Operational),
            '#' => Some(SpringSymbol::Damaged),
            '?' => Some(SpringSymbol::Unknown),
            _ => None,
        }
    }
}
