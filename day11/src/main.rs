use std::io::{self, Read};

use grid::Grid2D;
mod grid;

static DIRECTIONS: &'static [(isize, isize)] = &[
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

fn iteration_part1(grid: &Grid2D) -> (Grid2D, isize) {
    let mut occupied = 0;
    let mut new_grid = grid.clone();

    for (x, y) in grid.points_iter() {
        let seat = grid.get(x, y).unwrap();
        let occupied_neighbours = grid.neighbors(x, y).iter().filter(|&&s| s == '#').count();

        match (occupied_neighbours, seat) {
            (0, 'L') => {
                new_grid.set(x, y, '#');
                occupied += 1;
            }
            (taken, '#') if taken >= 4 => {
                new_grid.set(x, y, 'L');
            }
            (_, '#') => {
                occupied += 1;
            }
            _ => (),
        }
    }

    (new_grid, occupied)
}

fn iteration_part2(grid: &Grid2D) -> (Grid2D, isize) {
    let mut occupied = 0;
    let mut new_grid = grid.clone();

    for (x, y) in grid.points_iter() {
        let seat = grid.get(x, y).unwrap();
        let in_sight = DIRECTIONS
            .iter()
            .map(|(row_step, column_step)| {
                grid.in_sight(x + row_step, y + column_step, *row_step, *column_step)
                    .find(|&v| v == 'L' || v == '#')
            })
            .filter_map(|v| v)
            .filter(|&t| t == '#')
            .count();

        match (in_sight, seat) {
            (0, 'L') => {
                new_grid.set(x, y, '#');
                occupied += 1;
            }
            (taken, '#') if taken >= 5 => {
                new_grid.set(x, y, 'L');
            }
            (_, '#') => {
                occupied += 1;
            }
            _ => (),
        }
    }

    (new_grid, occupied)
}

fn part1(input: &str) -> isize {
    let mut grid: Grid2D = Grid2D::new(&input);

    loop {
        let (new_grid, occupied) = iteration_part1(&grid);

        if grid == new_grid {
            return occupied;
        }

        grid = new_grid;
    }
}

fn part2(input: &str) -> isize {
    let mut grid: Grid2D = Grid2D::new(&input);

    loop {
        let (new_grid, occupied) = iteration_part2(&grid);

        if grid == new_grid {
            return occupied;
        }

        grid = new_grid;
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{:?}", part1(&input));
    println!("{:?}", part2(&input));
}
