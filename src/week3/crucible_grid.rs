use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
    io::BufRead,
};

use crucible_path::{CruciblePath, CruciblePathStep};

use crate::shared_libs::{coord_2_d::Coord2D, direction::Direction, grid_map::GridMap};

mod crucible_path;

pub struct CrucibleGrid {
    map: GridMap<u32>,
}

impl CrucibleGrid {
    pub fn from_file(path: std::ffi::OsString) -> Self {
        let input = std::io::BufReader::new(clio::Input::new(&path).unwrap());
        let map = input
            .lines()
            .map_while(Result::ok)
            .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
            .collect();
        let map = GridMap::new(map);
        CrucibleGrid { map }
    }

    pub fn find_minimum_heat_loss_route(
        &self,
        minimum_consecutive: usize,
        maximum_consecutive: usize,
    ) -> CruciblePath {
        let mut remaining_paths = BinaryHeap::new();
        let mut lowest_cost_steps = HashMap::new();
        let start_position = Coord2D::new_row_column(0, 0);
        let end_position =
            Coord2D::new_row_column(self.map.row_count() - 1, self.map.col_count() - 1);
        let first_steps = get_first_steps(start_position, end_position, minimum_consecutive);
        for step in first_steps.clone() {
            lowest_cost_steps.insert(step.0.get_previous_step().clone(), 0);
        }
        remaining_paths.extend(first_steps);
        let mut shortest_complete_path: Option<CruciblePath> = None;
        while !remaining_paths.is_empty() {
            let shortest_path = remaining_paths.pop().unwrap().0;
            if shortest_complete_path.is_none()
                || shortest_complete_path.as_ref().unwrap().get_elapsed_cost()
                    > shortest_path.estimated_minimum_cost(1)
            {
                let previous_step = &shortest_path.get_previous_step();
                if previous_step.get_position() == end_position {
                    shortest_complete_path = Some(shortest_path);
                } else {
                    let consecutive_steps = previous_step.get_previous_consecutive_steps() + 1;
                    let previous_direction = &previous_step.get_direction();
                    let next_directions = get_next_directions(
                        consecutive_steps,
                        minimum_consecutive,
                        previous_direction,
                        maximum_consecutive,
                    );
                    remaining_paths.extend(self.get_next_paths(
                        next_directions,
                        previous_step,
                        &shortest_path,
                        previous_direction,
                        consecutive_steps,
                        &mut lowest_cost_steps,
                    ));
                }
            }
        }
        shortest_complete_path.unwrap()
    }

    fn get_next_paths(
        &self,
        next_directions: Vec<Direction>,
        previous_step: &CruciblePathStep,
        shortest_path: &CruciblePath,
        previous_direction: &Direction,
        consecutive_steps: usize,
        lowest_cost_steps: &mut HashMap<CruciblePathStep, u64>,
    ) -> Vec<Reverse<CruciblePath>> {
        let mut next_paths = Vec::new();
        for direction in next_directions {
            if let Some(next_position) = previous_step.get_position().new_in_direction(&direction) {
                if let Some(next_step_cost) = self.map.get_ref(&next_position) {
                    let next_step = if direction == *previous_direction {
                        CruciblePathStep::new(next_position, direction, consecutive_steps)
                    } else {
                        CruciblePathStep::new(next_position, direction, 0)
                    };
                    let lowest_cost = lowest_cost_steps.get(&next_step).unwrap_or(&u64::MAX);
                    let new_elapsed_cost =
                        shortest_path.get_elapsed_cost() + u64::from(*next_step_cost);
                    if new_elapsed_cost < *lowest_cost {
                        lowest_cost_steps.insert(next_step.clone(), new_elapsed_cost);
                        let mut new_path = shortest_path.clone();
                        new_path.add_step(next_step, *next_step_cost);
                        next_paths.push(Reverse(new_path));
                    }
                }
            }
        }
        next_paths
    }
}

fn get_first_steps(
    start_position: Coord2D<usize>,
    end_position: Coord2D<usize>,
    minimum_consecutive: usize,
) -> Vec<Reverse<CruciblePath>> {
    let mut first_steps = Vec::new();
    let first_step_east = CruciblePathStep::new(start_position, Direction::East, 0);
    first_steps.push(Reverse(CruciblePath::new(0, first_step_east, end_position)));
    if minimum_consecutive > 1 {
        let first_step_south = CruciblePathStep::new(start_position, Direction::South, 0);
        first_steps.push(Reverse(CruciblePath::new(
            0,
            first_step_south,
            end_position,
        )));
    }
    first_steps
}

fn get_next_directions(
    consecutive_steps: usize,
    minimum_consecutive: usize,
    previous_direction: &Direction,
    maximum_consecutive: usize,
) -> Vec<Direction> {
    let next_directions = match consecutive_steps.cmp(&minimum_consecutive){
        Ordering::Less => vec![previous_direction.clone()],
        Ordering::Equal|Ordering::Greater => match consecutive_steps.cmp(&maximum_consecutive){
            Ordering::Less => Direction::CARDINAL_DIRECTIONS.iter().map(Direction::clone).filter(|d|*d!=previous_direction.reverse()).collect(),
            Ordering::Equal => Direction::CARDINAL_DIRECTIONS.iter().map(Direction::clone).filter(|d|*d!=previous_direction.reverse() && *d!=*previous_direction).collect(),
            Ordering::Greater => panic!("The path should not have taken more consecutive steps in a direction than is allowed"),
        },
    };
    next_directions
}
