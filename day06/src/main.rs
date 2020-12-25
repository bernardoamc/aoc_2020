use std::collections::HashSet;
use std::hash::Hash;
use std::io::{self, Read};
use std::iter::FromIterator;

fn part1(input: &str) -> usize {
    let mut answers: HashSet<char> = HashSet::new();
    let mut count: usize = 0;

    for line in input.lines() {
        if line.is_empty() {
            count += answers.len();
            answers = HashSet::new();

            continue;
        }

        for c in line.chars().into_iter() {
            answers.insert(c);
        }
    }

    count
}

fn intersection<T: Eq + Hash>(a: HashSet<T>, b: &HashSet<T>) -> HashSet<T> {
    a.into_iter().filter(|e| b.contains(e)).collect()
}

fn part2(input: &str) -> usize {
    let mut answers: HashSet<char> = HashSet::from_iter(('a'..='z').into_iter());
    let mut count: usize = 0;

    for line in input.lines() {
        if line.is_empty() {
            count += answers.len();
            answers = HashSet::from_iter(('a'..='z').into_iter());

            continue;
        }

        let individual: HashSet<char> = HashSet::from_iter(line.chars().into_iter());
        answers = intersection(answers, &individual);
    }

    count
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    input.push('\n');
    println!("{:?}", part1(&input));
    println!("{:?}", part2(&input));
}
