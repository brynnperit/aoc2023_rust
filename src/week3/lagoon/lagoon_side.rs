use std::cmp::Ordering;

use crate::shared_libs::{coord_2_d::Coord2D, direction::Direction};

#[derive(PartialEq, Eq, Clone)]
pub struct LagoonSide {
    pub lower_point: Coord2D<i64>,
    pub higher_point: Coord2D<i64>,
    pub direction: Direction,
}

impl LagoonSide {
    pub fn new_plain(side_string: &str, previous_side: Option<&LagoonSide>) -> Option<Self> {
        let mut string_iterator = side_string.split_ascii_whitespace();
        let side_direction = match string_iterator.next()?.chars().next()? {
            'D' => Direction::South,
            'L' => Direction::West,
            'U' => Direction::North,
            'R' => Direction::East,
            _ => return None,
        };
        let side_length = string_iterator.next()?.parse::<i64>().unwrap();
        Self::new(previous_side, side_direction, side_length)
    }

    pub fn new_hex(side_string: &str, previous_side: Option<&LagoonSide>) -> Option<Self> {
        let hex_code_str = side_string.split_ascii_whitespace().nth(2)?;
        let hex_length_str = &hex_code_str[2..7];
        let hex_direction_str = &hex_code_str[7..8];
        let side_direction = match hex_direction_str {
            "1" => Direction::South,
            "2" => Direction::West,
            "3" => Direction::North,
            "0" => Direction::East,
            _ => return None,
        };
        let side_length = i64::from_str_radix(hex_length_str, 16).ok()?;
        Self::new(previous_side, side_direction, side_length)
    }

    fn new(
        previous_side: Option<&LagoonSide>,
        side_direction: Direction,
        side_length: i64,
    ) -> Option<LagoonSide> {
        let last_coord_of_previous = match previous_side {
            Some(previous_side) => previous_side.get_end_point(),
            None => Coord2D::new(0, 0),
        };
        let first_point = last_coord_of_previous;
        let second_point =
            first_point.new_in_direction_at_distance(&side_direction, side_length)?;
        match side_direction {
            Direction::North => Some(LagoonSide {
                lower_point: second_point,
                higher_point: first_point,
                direction: side_direction,
            }),
            Direction::East => Some(LagoonSide {
                lower_point: first_point,
                higher_point: second_point,
                direction: side_direction,
            }),
            Direction::South => Some(LagoonSide {
                lower_point: first_point,
                higher_point: second_point,
                direction: side_direction,
            }),
            Direction::West => Some(LagoonSide {
                lower_point: second_point,
                higher_point: first_point,
                direction: side_direction,
            }),
        }
    }

    pub fn get_end_point(&self) -> Coord2D<i64> {
        match self.direction {
            Direction::North => self.lower_point,
            Direction::East => self.higher_point,
            Direction::South => self.higher_point,
            Direction::West => self.lower_point,
        }
    }

    pub fn get_overlap(&self, other: &Self) -> Option<Self> {
        let orientation = &self.direction;
        let other_orientation = &other.direction;
        match orientation {
            Direction::North | Direction::South => match other_orientation {
                Direction::North | Direction::South => self.get_vertical_overlap(other),
                Direction::East | Direction::West => None,
            },
            Direction::East | Direction::West => match other_orientation {
                Direction::North | Direction::South => None,
                Direction::East | Direction::West => self.get_horizontal_overlap(other),
            },
        }
    }

    fn get_horizontal_overlap(&self, other: &Self) -> Option<Self> {
        let lower_first = self.lower_point;
        let higher_first = self.higher_point;
        let lower_second = other.lower_point;
        let higher_second = other.higher_point;
        if higher_first.get_x() < lower_second.get_x()
            || higher_second.get_x() < lower_first.get_x()
        {
            None
        } else {
            let overlap_lower_x = lower_first.get_x().max(lower_second.get_x());
            let overlap_higher_x = higher_first.get_x().min(higher_second.get_x());
            Some(LagoonSide {
                lower_point: Coord2D::new(overlap_lower_x, lower_first.get_y()),
                higher_point: Coord2D::new(overlap_higher_x, lower_first.get_y()),
                direction: self.direction.clone(),
            })
        }
    }

    fn get_vertical_overlap(&self, other: &Self) -> Option<Self> {
        let lower_first = self.lower_point;
        let higher_first = self.higher_point;
        let lower_second = other.lower_point;
        let higher_second = other.higher_point;
        if higher_first.get_y() < lower_second.get_y()
            || higher_second.get_y() < lower_first.get_y()
        {
            None
        } else {
            let overlap_lower_y = lower_first.get_y().max(lower_second.get_y());
            let overlap_higher_y = higher_first.get_y().min(higher_second.get_y());
            Some(LagoonSide {
                lower_point: Coord2D::new(lower_first.get_x(), overlap_lower_y),
                higher_point: Coord2D::new(lower_first.get_x(), overlap_higher_y),
                direction: self.direction.clone(),
            })
        }
    }

    pub fn remove_overlap(&self, other: &Self) -> Vec<LagoonSide> {
        let orientation = &self.direction;
        let other_orientation = &other.direction;
        match orientation {
            Direction::North | Direction::South => match other_orientation {
                Direction::North | Direction::South => self.remove_vertical_overlap(other),
                Direction::East | Direction::West => {
                    panic!("The two sides should have matching orientations in order to overlap")
                }
            },
            Direction::East | Direction::West => match other_orientation {
                Direction::North | Direction::South => {
                    panic!("The two sides should have matching orientations in order to overlap")
                }
                Direction::East | Direction::West => self.remove_horizontal_overlap(other),
            },
        }
    }

    fn remove_horizontal_overlap(&self, other: &Self) -> Vec<LagoonSide> {
        let lower_first = self.lower_point;
        let higher_first = self.higher_point;
        let lower_second = other.lower_point;
        let higher_second = other.higher_point;
        if higher_first.get_x() < lower_second.get_x()
            || higher_second.get_x() < lower_first.get_x()
        {
            panic!("The sides must actually overlap")
        } else {
            let overlap_lower_x = lower_first.get_x().max(lower_second.get_x());
            let overlap_higher_x = higher_first.get_x().min(higher_second.get_x());
            let mut overlap_vector = Vec::new();
            if overlap_lower_x > lower_first.get_x() {
                overlap_vector.push(LagoonSide {
                    lower_point: lower_first,
                    higher_point: Coord2D::new(overlap_lower_x - 1, lower_first.get_y()),
                    direction: self.direction.clone(),
                });
            }
            if overlap_higher_x < higher_first.get_x() {
                overlap_vector.push(LagoonSide {
                    lower_point: Coord2D::new(overlap_higher_x + 1, higher_first.get_y()),
                    higher_point: higher_first,
                    direction: self.direction.clone(),
                });
            }
            overlap_vector
        }
    }

    pub fn shrink_at_end(&mut self) {
        match self.direction {
            Direction::North => self.lower_point.in_direction(&Direction::South),
            Direction::East => self.higher_point.in_direction(&Direction::West),
            Direction::South => self.higher_point.in_direction(&Direction::North),
            Direction::West => self.lower_point.in_direction(&Direction::East),
        };
    }

    pub fn shrink_at_start(&mut self) {
        match self.direction {
            Direction::North => self.higher_point.in_direction(&Direction::North),
            Direction::East => self.lower_point.in_direction(&Direction::East),
            Direction::South => self.lower_point.in_direction(&Direction::South),
            Direction::West => self.higher_point.in_direction(&Direction::West),
        };
    }

    fn remove_vertical_overlap(&self, other: &Self) -> Vec<LagoonSide> {
        let lower_first = self.lower_point;
        let higher_first = self.higher_point;
        let lower_second = other.lower_point;
        let higher_second = other.higher_point;
        if higher_first.get_y() < lower_second.get_y()
            || higher_second.get_y() < lower_first.get_y()
        {
            panic!("The sides must actually overlap")
        } else {
            let overlap_lower_y = lower_first.get_y().max(lower_second.get_y());
            let overlap_higher_y = higher_first.get_y().min(higher_second.get_y());
            let mut overlap_vector = Vec::new();
            if overlap_lower_y > lower_first.get_y() {
                overlap_vector.push(LagoonSide {
                    lower_point: lower_first,
                    higher_point: Coord2D::new(lower_first.get_x(), overlap_lower_y - 1),
                    direction: self.direction.clone(),
                });
            }
            if overlap_higher_y < higher_first.get_y() {
                overlap_vector.push(LagoonSide {
                    lower_point: Coord2D::new(higher_first.get_x(), overlap_higher_y + 1),
                    higher_point: higher_first,
                    direction: self.direction.clone(),
                });
            }
            overlap_vector
        }
    }
}

impl PartialOrd for LagoonSide {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LagoonSide {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.lower_point.get_y().cmp(&other.lower_point.get_y()) {
            Ordering::Equal => match self.higher_point.get_y().cmp(&other.higher_point.get_y()) {
                Ordering::Equal => match self.lower_point.get_x().cmp(&other.lower_point.get_x()) {
                    Ordering::Equal => self.higher_point.get_x().cmp(&other.higher_point.get_x()),
                    non_equal => non_equal,
                },
                non_equal => non_equal,
            },
            non_equal => non_equal,
        }
    }
}
