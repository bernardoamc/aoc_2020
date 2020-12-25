use std::collections::HashMap;
use std::io::{self, Read};

const DIRECTIONS: [&str; 6] = ["ne", "nw", "se", "sw", "e", "w"];
const BASE_COORDINATES: [(i16, i16); 6] = [(0, 1), (-1, 1), (1, -1), (0, -1), (1, 0), (-1, 0)];

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub enum State {
    White,
    Black,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
struct Tile {
    x: i16,
    y: i16,
}

impl Tile {
    fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    fn step(&mut self, direction: &str) {
        let direction = match direction {
            "se" => (1, -1),
            "sw" => (0, -1),
            "ne" => (0, 1),
            "nw" => (-1, 1),
            "e" => (1, 0),
            "w" => (-1, 0),
            _ => panic!("Invalid direction!"),
        };

        self.x += direction.0;
        self.y += direction.1;
    }

    fn neighbours(&self) -> Vec<Tile> {
        BASE_COORDINATES
            .iter()
            .map(|(x, y)| Tile::new(self.x + x, self.y + y))
            .collect()
    }
}

fn part1(input: &str) -> HashMap<Tile, State> {
    let mut tiles: HashMap<Tile, State> = HashMap::new();

    for mut line in input.lines() {
        let mut tile = Tile::new(0, 0);

        while !line.is_empty() {
            for direction in DIRECTIONS.iter() {
                if line.starts_with(direction) {
                    line = &line[direction.len()..];
                    tile.step(direction);
                    break;
                }
            }
        }

        let state = match tiles.get(&tile) {
            None => State::Black,
            Some(state) => match state {
                State::Black => State::White,
                State::White => State::Black,
            },
        };

        tiles.insert(tile, state);
    }

    tiles
}

fn part2(tiles: HashMap<Tile, State>) -> HashMap<Tile, State> {
    let mut state = tiles.clone();

    for _ in 0..100 {
        let current_state = state.clone();

        for (tile, tile_state) in current_state.iter() {
            let neighbours = tile.neighbours();

            for neighbour in &neighbours {
                if !state.contains_key(&neighbour) {
                    let black_neighbours = neighbour
                        .neighbours()
                        .iter()
                        .filter(|&n| *current_state.get(n).unwrap_or(&State::White) == State::Black)
                        .count();

                    if black_neighbours == 2 {
                        state.insert(neighbour.clone(), State::Black);
                    } else {
                        state.insert(neighbour.clone(), State::White);
                    }
                }
            }

            let black_neighbours = neighbours
                .iter()
                .filter(|&n| *current_state.get(n).unwrap_or(&State::White) == State::Black)
                .count();

            match &tile_state {
                State::White => {
                    if black_neighbours == 2 {
                        state.insert(tile.clone(), State::Black);
                    }
                }
                State::Black => {
                    if black_neighbours == 0 || black_neighbours > 2 {
                        state.insert(tile.clone(), State::White);
                    }
                }
            };
        }
    }

    state
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let tiles = part1(&input);
    let black_tiles = tiles
        .iter()
        .filter(|(_, state)| **state == State::Black)
        .count();
    println!("{:?}", black_tiles);

    let tiles = part2(tiles);
    let black_tiles = tiles
        .iter()
        .filter(|(_, state)| **state == State::Black)
        .count();
    println!("{:?}", black_tiles);
}
