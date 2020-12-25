use std::io::{self, Read};

pub fn solve(input: &str, step: usize, down: usize) -> u64 {
    let mut index = step;
    let mut trees = 0;

    for line in input.lines().skip(down).step_by(down) {
        let len = line.len();
        index = index % len;
        let mut line = line.chars();

        if line.nth(index).unwrap() == '#' {
            trees += 1;
        }

        index += step;
    }

    trees
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{:?}", solve(&input, 3, 1));

    let total_trees: u64 = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .fold(1, |acc, (step, down)| {
            acc * solve(&input, *step as usize, *down as usize)
        });
    println!("{:?}", total_trees);
}
