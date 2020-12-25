use itertools::iproduct;
use std::collections::HashSet;
use std::io::{self, Read};

type Point = (i8, i8, i8, i8);
type AxisBoundary = (i8, i8);

struct Grid {
    boundaries: (AxisBoundary, AxisBoundary, AxisBoundary, AxisBoundary),
    active: HashSet<Point>,
    dimensions: usize,
}

impl Grid {
    pub fn new<'a, I>(lines: I, dimensions: usize) -> Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        let mut active: HashSet<Point> = HashSet::new();

        for (y, line) in lines.into_iter().enumerate() {
            for (x, state) in line.chars().enumerate() {
                if state == '#' {
                    active.insert((x as i8, y as i8, 0, 0));
                }
            }
        }

        let (max_x, max_y) = active
            .iter()
            .fold((0, 0), |(mx, my), &(x, y, _, _)| (mx.max(x), my.max(y)));

        Self {
            boundaries: ((0, max_x), (0, max_y), (0, 0), (0, 0)),
            active,
            dimensions,
        }
    }

    fn neighbours((x, y, z, w): Point) -> impl Iterator<Item = Point> {
        iproduct!(x - 1..=x + 1, y - 1..=y + 1, z - 1..=z + 1, w - 1..=w + 1)
            .filter(move |p| *p != (x, y, z, w))
    }

    fn active_next(&self, point: Point) -> bool {
        let active_neighbours = Self::neighbours(point)
            .filter(|p| self.active.contains(p))
            .count();
        let currently_active = self.active.contains(&point);
        active_neighbours == 3 || (currently_active && active_neighbours == 2)
    }

    fn next_boundaries(&mut self) {
        let ((min_x, max_x), (min_y, max_y), (min_z, max_z), (min_w, max_w)) = self.boundaries;
        self.boundaries = (
            (min_x - 1, max_x + 1),
            (min_y - 1, max_y + 1),
            (min_z - 1, max_z + 1),
            (min_w - 1, max_w + 1),
        );
    }

    fn next(&mut self) {
        self.next_boundaries();

        let ((min_x, max_x), (min_y, max_y), (min_z, max_z), (mut min_w, mut max_w)) =
            self.boundaries;

        if self.dimensions == 3 {
            min_w = 0;
            max_w = 0
        }

        self.active = iproduct!(min_x..=max_x, min_y..=max_y, min_z..=max_z, min_w..=max_w)
            .filter(|&pos| self.active_next(pos))
            .collect();
    }
}

fn run(input: &str, dimesions: usize) -> usize {
    let mut grid = Grid::new(input.lines(), dimesions);

    for _ in 0..6 {
        grid.next();
    }

    grid.active.len()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{:?}", run(&input, 3));
    println!("{:?}", run(&input, 4));
}
