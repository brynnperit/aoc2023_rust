use super::coord_2_d::Coord2D;

#[derive(Hash)]
pub struct GridMap<T> {
    tiles: Vec<Vec<T>>,
}

impl<T> GridMap<T> {
    pub fn new(tiles: Vec<Vec<T>>) -> Self {
        GridMap { tiles }
    }

    pub fn get_ref(&self, coord: &Coord2D<usize>) -> Option<&T> {
        self.tiles.get(coord.get_row())?.get(coord.get_col())
    }

    pub fn set(&mut self, coord: &Coord2D<usize>, tile: T) {
        self.tiles[coord.get_row()][coord.get_col()] = tile
    }

    pub fn rows_iter(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        self.tiles.iter().map(|row| row.iter())
    }

    pub fn cols_iter(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        ColumnsIterator::new(self)
    }

    pub fn row_count(&self) -> usize {
        self.tiles.len()
    }

    pub fn col_count(&self) -> usize {
        if let Some(first_row) = self.tiles.first() {
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

struct ColumnsIterator<'a, T> {
    col: usize,
    map: &'a GridMap<T>,
}

impl<'a, T> ColumnsIterator<'a, T> {
    fn new(map: &'a GridMap<T>) -> Self {
        ColumnsIterator { col: 0, map }
    }
}

impl<'a, T> Iterator for ColumnsIterator<'a, T> {
    type Item = ColumnIterator<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.col += 1;
        if self.col <= self.map.col_count() {
            return Some(ColumnIterator::new(self.col, self.map));
        }
        None
    }
}

struct ColumnIterator<'a, T> {
    col: usize,
    row: usize,
    map: &'a GridMap<T>,
}

impl<'a, T> ColumnIterator<'a, T> {
    fn new(col: usize, map: &'a GridMap<T>) -> Self {
        ColumnIterator { col, row: 0, map }
    }
}

impl<'a, T> Iterator for ColumnIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.row += 1;
        if self.row <= self.map.row_count() {
            return Some(&self.map.tiles[self.row - 1][self.col - 1]);
        }
        None
    }
}
