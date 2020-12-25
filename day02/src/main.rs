use std::collections::HashMap;
use std::io::{self, Read};

struct Policy<'s> {
    c1: u32,
    c2: u32,
    character: char,
    input: &'s str,
}

impl<'s> Policy<'s> {
    pub fn parse(input: &'s str) -> Self {
        let parts: Vec<&str> = input.split_whitespace().collect();
        let constraints: Vec<u32> = parts[0]
            .split("-")
            .map(|n| n.parse::<u32>().unwrap())
            .collect();
        let character = parts[1].strip_suffix(":").unwrap().chars().next().unwrap();
        let input = parts[2];

        Self {
            c1: constraints[0],
            c2: constraints[1],
            character,
            input,
        }
    }

    pub fn old_policy_comply(&self) -> bool {
        let primitives = self.input.chars().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

        primitives.contains_key(&self.character)
            && primitives[&self.character] >= self.c1
            && primitives[&self.character] <= self.c2
    }

    pub fn new_policy_comply(&self) -> bool {
        let opt1 = self.input.chars().nth((self.c1 - 1) as usize);
        let opt2 = self.input.chars().nth((self.c2 - 1) as usize);

        (opt1 == Some(self.character) && opt2 != Some(self.character))
            || (opt1 != Some(self.character) && opt2 == Some(self.character))
    }
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| Policy::parse(line).old_policy_comply())
        .filter(|p| *p)
        .count()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| Policy::parse(line).new_policy_comply())
        .filter(|p| *p)
        .count()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{:?}", part1(&input));
    println!("{:?}", part2(&input));
}
