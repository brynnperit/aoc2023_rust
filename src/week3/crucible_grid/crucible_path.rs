use std::cmp::Ordering;

use crate::shared_libs::{coord_2_d::Coord2D, direction::Direction};

#[derive(Clone)]
pub struct CruciblePath {
    elapsed_cost: u64,
    previous_step: CruciblePathStep,
    end_position: Coord2D<usize>,
}

impl PartialEq for CruciblePath {
    fn eq(&self, other: &Self) -> bool {
        self.estimated_minimum_cost(Self::DISTANCE_COST_FACTOR)
            == other.estimated_minimum_cost(Self::DISTANCE_COST_FACTOR)
    }
}

impl Eq for CruciblePath {}

impl PartialOrd for CruciblePath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CruciblePath {
    fn cmp(&self, other: &Self) -> Ordering {
        self.estimated_minimum_cost(Self::DISTANCE_COST_FACTOR)
            .cmp(&other.estimated_minimum_cost(Self::DISTANCE_COST_FACTOR))
    }
}

impl CruciblePath {
    const DISTANCE_COST_FACTOR: u64 = 1;

    pub fn new(
        start_cost: u32,
        previous_step: CruciblePathStep,
        end_position: Coord2D<usize>,
    ) -> Self {
        CruciblePath {
            elapsed_cost: u64::from(start_cost),
            previous_step,
            end_position,
        }
    }

    pub fn add_step(&mut self, step: CruciblePathStep, step_cost: u32) {
        self.previous_step = step;
        self.elapsed_cost += u64::from(step_cost);
    }

    pub fn get_elapsed_cost(&self) -> u64 {
        self.elapsed_cost
    }

    pub fn estimated_minimum_cost(&self, cost_factor: u64) -> u64 {
        let current_position = &self.previous_step.position;
        let x_diff: u64 = current_position
            .get_x()
            .abs_diff(self.end_position.get_x())
            .try_into()
            .unwrap();
        let y_diff: u64 = current_position
            .get_y()
            .abs_diff(self.end_position.get_y())
            .try_into()
            .unwrap();
        self.elapsed_cost + x_diff * cost_factor + y_diff * cost_factor
    }

    pub fn get_previous_step(&self) -> &CruciblePathStep {
        &self.previous_step
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CruciblePathStep {
    position: Coord2D<usize>,
    direction: Direction,
    previous_consecutive_steps: usize,
}

impl CruciblePathStep {
    pub fn new(
        position: Coord2D<usize>,
        direction: Direction,
        previous_consecutive_steps: usize,
    ) -> Self {
        CruciblePathStep {
            position,
            direction,
            previous_consecutive_steps,
        }
    }

    pub fn get_position(&self) -> Coord2D<usize> {
        self.position
    }

    pub fn get_previous_consecutive_steps(&self) -> usize {
        self.previous_consecutive_steps
    }

    pub fn get_direction(&self) -> Direction {
        self.direction.clone()
    }
}
