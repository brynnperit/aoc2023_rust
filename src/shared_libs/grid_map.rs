use super::coord_2_d::Coord2D;

pub struct GridMap<T> {
    tiles: Vec<Vec<T>>,
}

impl<T> GridMap<T> {
    pub fn new(tiles: Vec<Vec<T>>) -> Self {
        GridMap { tiles }
    }

    pub fn get_ref(&self, coord: Coord2D<usize>) -> Option<&T> {
        Some(self.tiles.get(coord.get_row())?.get(coord.get_col())?)
    }

    pub fn set(&mut self, coord: &Coord2D<usize>, tile: T) {
        self.tiles[coord.get_row()][coord.get_col()] = tile
    }

    pub fn rows_iter(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        self.tiles.iter().map(|row| row.iter())
    }

    pub fn row_count(&self) -> usize {
        self.tiles.len()
    }

    pub fn col_count(&self) -> usize {
        if let Some(first_row) = self.tiles.get(0) {
            return first_row.len();
        }
        0
    }
}

impl<T: Eq> GridMap<T> {
    pub fn find_coords(&self, to_find: T) -> Option<Coord2D<usize>> {
        for row in 0..self.tiles.len() {
            if let Some(col) = self.tiles[row].iter().position(|item| *item == to_find) {
                return Some(Coord2D::new_row_column(row, col));
            }
        }
        None
    }
}
