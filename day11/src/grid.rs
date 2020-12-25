#[derive(Debug, Clone, PartialEq)]
pub struct Grid2D {
    points: Vec<Vec<char>>,
    rows: isize,
    columns: isize,
}

impl Grid2D {
    pub fn new(input: &str) -> Grid2D {
        let points: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
        let rows = points.len() as isize;
        let columns = points[0].len() as isize;

        Grid2D {
            points,
            rows,
            columns,
        }
    }

    pub fn get(&self, row: isize, column: isize) -> Option<char> {
        if row < 0 || column < 0 || row >= self.rows || column >= self.columns {
            None
        } else {
            Some(self.points[row as usize][column as usize])
        }
    }

    pub fn set(&mut self, row: isize, column: isize, value: char) -> bool {
        if row < 0 || column < 0 || row >= self.rows || column >= self.columns {
            false
        } else {
            self.points[row as usize][column as usize] = value;
            true
        }
    }

    pub fn neighbors(&self, row: isize, column: isize) -> Vec<char> {
        vec![
            self.get(row, column - 1),
            self.get(row + 1, column - 1),
            self.get(row + 1, column),
            self.get(row + 1, column + 1),
            self.get(row, column + 1),
            self.get(row - 1, column + 1),
            self.get(row - 1, column),
            self.get(row - 1, column - 1),
        ]
        .into_iter()
        .filter_map(|e| e)
        .collect()
    }

    pub fn in_sight(
        &self,
        row: isize,
        column: isize,
        row_step: isize,
        column_step: isize,
    ) -> LineOfSightIterator {
        LineOfSightIterator::new(&self, row, column, row_step, column_step)
    }

    #[allow(dead_code)]
    pub fn iter(&self) -> GridIterator {
        GridIterator::new(&self)
    }

    #[allow(dead_code)]
    pub fn points_iter(&self) -> PointIterator {
        PointIterator::new(&self)
    }
}

pub struct GridIterator<'a> {
    grid: &'a Grid2D,
    row: isize,
    column: isize,
}

impl<'a> GridIterator<'a> {
    #[allow(dead_code)]
    pub fn new(grid: &'a Grid2D) -> Self {
        Self {
            grid,
            row: 0,
            column: 0,
        }
    }
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let point = self.grid.get(self.row, self.column);

        if self.column + 1 < self.grid.columns {
            self.column += 1;
        } else {
            self.column = 0;
            self.row += 1;
        }

        point
    }
}

pub struct PointIterator<'a> {
    grid: &'a Grid2D,
    row: isize,
    column: isize,
}

impl<'a> PointIterator<'a> {
    #[allow(dead_code)]
    pub fn new(grid: &'a Grid2D) -> Self {
        Self {
            grid,
            row: 0,
            column: 0,
        }
    }
}

impl<'a> Iterator for PointIterator<'a> {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        let coord = self.grid.get(self.row, self.column);
        let (row, column) = (self.row, self.column);

        if coord.is_none() {
            return None;
        }

        if self.column + 1 < self.grid.columns {
            self.column += 1;
        } else {
            self.column = 0;
            self.row += 1;
        }

        Some((row, column))
    }
}

pub struct LineOfSightIterator<'a> {
    grid: &'a Grid2D,
    row: isize,
    column: isize,
    row_step: isize,
    column_step: isize,
}

impl<'a> LineOfSightIterator<'a> {
    fn new(
        grid: &'a Grid2D,
        row: isize,
        column: isize,
        row_step: isize,
        column_step: isize,
    ) -> LineOfSightIterator<'a> {
        LineOfSightIterator {
            grid,
            row,
            column,
            row_step,
            column_step,
        }
    }
}

impl<'a> Iterator for LineOfSightIterator<'a> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        let point = self.grid.get(self.row, self.column);

        self.row += self.row_step;
        self.column += self.column_step;

        point
    }
}
