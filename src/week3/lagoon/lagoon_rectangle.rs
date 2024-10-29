use crate::shared_libs::coord_2_d::Coord2D;

use super::lagoon_side::LagoonSide;

pub struct LagoonRectangle {
    first_corner: Coord2D<i64>,
    second_corner: Coord2D<i64>,
}

impl LagoonRectangle {
    pub fn from_horizontal_sides(first_side: &LagoonSide, second_side: &LagoonSide) -> Self {
        LagoonRectangle {
            first_corner: first_side.lower_point,
            second_corner: Coord2D::new(
                first_side.higher_point.get_x(),
                second_side.higher_point.get_y(),
            ),
        }
    }

    pub fn calculate_area(&self) -> u64 {
        (self
            .first_corner
            .get_x()
            .abs_diff(self.second_corner.get_x())
            + 1)
            * (self
                .first_corner
                .get_y()
                .abs_diff(self.second_corner.get_y())
                + 1)
    }
}
