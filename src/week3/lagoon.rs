use std::{collections::BTreeSet, io::BufRead};

use crate::shared_libs::direction::Direction;
use lagoon_rectangle::LagoonRectangle;
use lagoon_side::LagoonSide;

mod lagoon_rectangle;
mod lagoon_side;

pub struct Lagoon {
    internal_areas: Vec<LagoonRectangle>,
}

impl Lagoon {
    pub fn from_plain_file(path: std::ffi::OsString) -> Self {
        Self::from_file(path, &LagoonSide::new_plain)
    }

    pub fn from_hex_file(path: std::ffi::OsString) -> Self {
        Self::from_file(path, &LagoonSide::new_hex)
    }

    pub fn calculate_area(&self) -> u64 {
        self.internal_areas
            .iter()
            .map(LagoonRectangle::calculate_area)
            .sum()
    }

    fn from_file(
        path: std::ffi::OsString,
        lagoon_side_parser: &dyn Fn(&str, Option<&LagoonSide>) -> Option<LagoonSide>,
    ) -> Self {
        let input = std::io::BufReader::new(clio::Input::new(&path).unwrap());
        let mut sides = Vec::new();
        for side_string in input.lines().map_while(Result::ok) {
            match lagoon_side_parser(&side_string, sides.last()) {
                Some(side) => sides.push(side),
                None => panic!(
                    "This string for the lagoon side did not have the correct formatting: {}",
                    side_string
                ),
            }
        }
        Lagoon::new(sides)
    }

    fn new(mut sides: Vec<LagoonSide>) -> Self {
        Self::reduce_horizontal_side_overlap(&mut sides);

        let mut horizontal_sides = BTreeSet::new();
        for side in sides {
            match side.direction {
                Direction::East | Direction::West => {
                    horizontal_sides.insert(side);
                }
                Direction::North | Direction::South => (),
            };
        }
        let mut internal_areas = Vec::new();
        while let Some(highest_side) = horizontal_sides.pop_first() {
            let mut top_sides = vec![highest_side];
            while let Some(top_side) = top_sides.pop() {
                let mut horizontal_sides_to_remove = Vec::new();
                let mut horizontal_sides_to_add = Vec::new();
                let mut top_sides_to_add = Vec::new();
                let mut found_match = false;
                for bottom_side in horizontal_sides.iter() {
                    if let Some(top_overlap) = top_side.get_overlap(bottom_side) {
                        horizontal_sides_to_remove.push(bottom_side.clone());
                        horizontal_sides_to_add.append(&mut bottom_side.remove_overlap(&top_side));
                        top_sides_to_add.append(&mut top_side.remove_overlap(bottom_side));
                        internal_areas.push(LagoonRectangle::from_horizontal_sides(
                            &top_overlap,
                            &bottom_side.get_overlap(&top_side).unwrap(),
                        ));
                        found_match = true;
                        break;
                    }
                }
                if !found_match {
                    panic!("Failed to find a bottom-side match for a top side")
                }
                for side in horizontal_sides_to_remove {
                    horizontal_sides.remove(&side);
                }
                for side in horizontal_sides_to_add {
                    horizontal_sides.insert(side);
                }
                for side in top_sides_to_add {
                    top_sides.push(side);
                }
            }
        }

        Self { internal_areas }
    }

    fn reduce_horizontal_side_overlap(sides: &mut [LagoonSide]) {
        for side_index in 0..sides.len() {
            let previous_index = if side_index == 0 {
                sides.len() - 1
            } else {
                side_index - 1
            };
            let next_index = if side_index == sides.len() - 1 {
                0
            } else {
                side_index + 1
            };
            let previous_direction = &sides[previous_index].direction;
            let next_direction = &sides[next_index].direction;
            match sides[side_index].direction {
                Direction::North => match previous_direction {
                    Direction::East => match next_direction {
                        Direction::East => sides[previous_index].shrink_at_end(),
                        Direction::West => {
                            sides[previous_index].shrink_at_end();
                            sides[next_index].shrink_at_start();
                        }
                        _ => todo!(),
                    },
                    Direction::West => match next_direction {
                        Direction::East => (),
                        Direction::West => sides[next_index].shrink_at_start(),
                        _ => todo!(),
                    },
                    _ => todo!(),
                },
                Direction::South => match previous_direction {
                    Direction::East => match next_direction {
                        Direction::East => sides[next_index].shrink_at_start(),
                        Direction::West => (),
                        _ => todo!(),
                    },
                    Direction::West => match next_direction {
                        Direction::East => {
                            sides[previous_index].shrink_at_end();
                            sides[next_index].shrink_at_start();
                        }
                        Direction::West => sides[previous_index].shrink_at_end(),
                        _ => todo!(),
                    },
                    _ => todo!(),
                },
                Direction::East => (),
                Direction::West => (),
            };
        }
    }
}
