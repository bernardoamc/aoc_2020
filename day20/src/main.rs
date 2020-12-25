use std::collections::HashMap;
use std::io::{self, Read};

const MONSTER_1: usize = 18;
const MONSTER_2: [usize; 8] = [0, 5, 6, 11, 12, 17, 18, 19];
const MONSTER_3: [usize; 6] = [1, 4, 7, 10, 13, 16];

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Tile {
    points: Vec<Vec<char>>,
    width: usize,
    height: usize,
    transformations: Option<Vec<Tile>>,
}

impl Tile {
    pub fn new(lines: std::str::Lines) -> Self {
        let points: Vec<Vec<_>> = lines.map(|l| l.chars().collect()).collect();
        let height = points.len();
        let width = points[0].len();
        let transformations: Vec<Tile> =
            TransformationIterator::new(points.clone(), width, height).collect();

        Self {
            points,
            width,
            height,
            transformations: Some(transformations),
        }
    }

    pub fn get_row_without_borders(&self, row: usize) -> Vec<char> {
        self.points[row][1..self.width - 1].to_vec()
    }

    pub fn get(&self, row: usize, column: usize) -> char {
        self.points[row][column]
    }

    pub fn top_border(&self) -> Vec<char> {
        (0..self.height).map(|c| self.get(0, c)).collect()
    }

    pub fn bottom_border(&self) -> Vec<char> {
        (0..self.height)
            .map(|c| self.get(self.height - 1, c))
            .collect()
    }

    pub fn left_border(&self) -> Vec<char> {
        (0..self.width).map(|r| self.get(r, 0)).collect()
    }

    pub fn right_border(&self) -> Vec<char> {
        (0..self.width)
            .map(|r| self.get(r, self.width - 1))
            .collect()
    }
}

pub struct TransformationIterator {
    points: Vec<Vec<char>>,
    width: usize,
    height: usize,
    rotations: usize,
    flipped_rotations: usize,
}

impl TransformationIterator {
    pub fn rotate_left(&mut self) {
        let mut rotation = self.points.clone();
        let max_idx = self.width - 1;

        for row in 0..self.width {
            for column in 0..self.height {
                let x = self.points[row][column];
                rotation[max_idx - column][row] = x;
            }
        }

        self.points = rotation;
    }

    fn flip_horizontal(&mut self) {
        self.points = self
            .points
            .iter()
            .map(|r| {
                let mut row = r.clone();
                row.reverse();
                row
            })
            .collect();
    }

    pub fn new(points: Vec<Vec<char>>, width: usize, height: usize) -> Self {
        Self {
            points,
            width,
            height,
            rotations: 4,
            flipped_rotations: 4,
        }
    }
}

impl<'a> Iterator for TransformationIterator {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rotations == 4 {
            self.rotations -= 1;
            return Some(Tile {
                points: self.points.clone(),
                width: self.width,
                height: self.height,
                transformations: None,
            });
        }

        if self.rotations > 0 {
            self.rotate_left();
            self.rotations -= 1;
            return Some(Tile {
                points: self.points.clone(),
                width: self.width,
                height: self.height,
                transformations: None,
            });
        }

        if self.flipped_rotations == 4 {
            self.flip_horizontal();
            self.flipped_rotations -= 1;
            return Some(Tile {
                points: self.points.clone(),
                width: self.width,
                height: self.height,
                transformations: None,
            });
        }

        if self.flipped_rotations > 0 {
            self.rotate_left();
            self.flipped_rotations -= 1;
            return Some(Tile {
                points: self.points.clone(),
                width: self.width,
                height: self.height,
                transformations: None,
            });
        }

        None
    }
}

fn parse(input: &str) -> HashMap<usize, Tile> {
    let tiles: Vec<&str> = input.split("\n\n").collect();
    let mut tilemap = HashMap::new();

    for tile in tiles {
        let mut lines = tile.lines();

        let tile_id = lines
            .next()
            .unwrap()
            .strip_prefix("Tile ")
            .and_then(|s| s.strip_suffix(":"))
            .unwrap()
            .parse::<usize>()
            .unwrap();

        tilemap.insert(tile_id.clone(), Tile::new(lines));
    }

    tilemap
}

fn find_top_left(tilemap: &HashMap<usize, Tile>) -> (usize, Tile) {
    for (id, tile) in tilemap {
        for transformation in tile.transformations.clone().unwrap().iter() {
            let mut right_matches_amount = 0;
            let mut bottom_matches_amount = 0;

            let left_border = transformation.left_border();
            let top_border = transformation.top_border();

            for (other_id, other_tile) in tilemap {
                if id == other_id {
                    continue;
                }

                other_tile
                    .transformations
                    .clone()
                    .unwrap()
                    .iter()
                    .for_each(|t| {
                        if t.right_border() == left_border {
                            right_matches_amount += 1;
                        }

                        if t.bottom_border() == top_border {
                            bottom_matches_amount += 1;
                        }
                    })
            }

            if right_matches_amount == 0 && bottom_matches_amount == 0 {
                return (*id, transformation.clone());
            }
        }
    }

    panic!("Top left not found!");
}

fn find_first_column(
    top_tile_id: usize,
    top_tile: &Tile,
    tilemap: &mut HashMap<usize, Tile>,
    rows: usize,
) -> Vec<(usize, Tile)> {
    let current_tilemap = tilemap.clone();
    let mut candidates: Vec<(usize, Tile)> = vec![(top_tile_id, top_tile.clone())];
    let mut bottom_border: Vec<char> = top_tile.bottom_border();

    for _ in 0..rows {
        'inner: for (id, tile) in &current_tilemap {
            for transformation in tile.transformations.clone().unwrap().iter() {
                if !tilemap.contains_key(id) {
                    continue;
                }

                if transformation.top_border() == bottom_border {
                    tilemap.remove(&id);
                    bottom_border = transformation.bottom_border();
                    candidates.push((*id, transformation.clone()));
                    break 'inner;
                }
            }
        }
    }

    candidates
}

fn find_row(
    left_tile: &Tile,
    tilemap: &mut HashMap<usize, Tile>,
    columns: usize,
) -> Vec<(usize, Tile)> {
    let current_tilemap = tilemap.clone();
    let mut entries: Vec<(usize, Tile)> = Vec::new();
    let mut right_border: Vec<char> = left_tile.right_border();

    for _ in 0..columns {
        'inner: for (id, tile) in &current_tilemap {
            for transformation in tile.transformations.clone().unwrap().iter() {
                if !tilemap.contains_key(id) {
                    continue;
                }

                if transformation.left_border() == right_border {
                    tilemap.remove(&id);
                    right_border = transformation.right_border();
                    entries.push((*id, transformation.clone()));
                    break 'inner;
                }
            }
        }
    }

    entries
}

fn find_image(tilemap: &mut HashMap<usize, Tile>) -> (Vec<Vec<usize>>, Tile) {
    let tiles_amount = (tilemap.keys().len() as f64).sqrt() as usize;

    let mut image: Vec<Vec<(usize, Tile)>> = (0..tiles_amount).map(|_| Vec::new()).collect();
    let mut current_row = 0;

    let (top_left_id, top_left_tile) = find_top_left(tilemap);
    tilemap.remove(&top_left_id);

    find_first_column(top_left_id, &top_left_tile, tilemap, tiles_amount - 1)
        .iter()
        .for_each(|(id, left_tile)| {
            image[current_row].push((*id, left_tile.clone()));

            find_row(&left_tile, tilemap, tiles_amount - 1)
                .iter()
                .for_each(|(id, tile)| {
                    image[current_row].push((*id, tile.clone()));
                });

            current_row += 1;
        });

    let tile_composition = image
        .iter()
        .map(|row| row.iter().map(|r| r.0).collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>();

    let tile_rows = image[0][0].1.height - 2;
    let mut final_image: Vec<Vec<char>> =
        (0..tiles_amount * tile_rows).map(|_| Vec::new()).collect();

    for (row_idx, image_row) in image.iter().enumerate() {
        for (_, tile) in image_row {
            for tile_row in 1..=tile_rows {
                final_image[(row_idx * tile_rows) + tile_row - 1]
                    .append(&mut tile.get_row_without_borders(tile_row));
            }
        }
    }

    let final_height = final_image.len();
    let final_width = final_image[0].len();

    (
        tile_composition,
        Tile {
            points: final_image,
            height: final_height,
            width: final_width,
            transformations: None,
        },
    )
}

fn monsters_in(tile: &Tile) -> usize {
    let mut count = 0;

    for rows in tile.points.windows(3) {
        let mut rows_iter = rows.iter();
        let first_row = rows_iter.next().unwrap();
        let second_row = rows_iter.next().unwrap();
        let third_row = rows_iter.next().unwrap();

        for (idx_1, point) in first_row[MONSTER_1..].iter().enumerate() {
            if *point != '#' {
                continue;
            }

            if MONSTER_2.iter().all(|x| second_row[x + idx_1] == '#')
                && MONSTER_3.iter().all(|x| third_row[x + idx_1] == '#')
            {
                count += 1;
            }
        }
    }

    count
}

fn count_monsters(tile: &Tile) -> usize {
    for transformation in TransformationIterator::new(tile.points.clone(), tile.width, tile.height)
    {
        let monsters = monsters_in(&transformation);

        if monsters != 0 {
            return monsters;
        }
    }

    0
}

fn part1(tile_composition: &Vec<Vec<usize>>) -> usize {
    let side = tile_composition[0].len() - 1;

    tile_composition[0][0]
        * tile_composition[0][side]
        * tile_composition[side][0]
        * tile_composition[side][side]
}

fn part2(image: &Tile) -> usize {
    let monsters = count_monsters(image);
    let tags_in_monster = 15;
    let mut total_tags = 0;

    for row in 0..image.height {
        for column in 0..image.width {
            if image.get(row, column) == '#' {
                total_tags += 1;
            }
        }
    }

    total_tags - (monsters * tags_in_monster)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tilemap = parse(&input);
    let (tile_composition, image) = find_image(&mut tilemap);

    println!("{:?}", part1(&tile_composition));
    println!("{:?}", part2(&image));
}
