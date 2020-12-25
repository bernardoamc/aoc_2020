use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};
use std::io::{self, Read};

fn parse_player(input: &str) -> VecDeque<u32> {
    input
        .lines()
        .skip(1)
        .map(|line| line.parse().unwrap())
        .collect()
}

fn parse(input: &str) -> (VecDeque<u32>, VecDeque<u32>) {
    let players_input: Vec<&str> = input.split("\n\n").collect();
    (
        parse_player(players_input[0]),
        parse_player(players_input[1]),
    )
}

fn part1(mut player_1: VecDeque<u32>, mut player_2: VecDeque<u32>) -> VecDeque<u32> {
    while !player_1.is_empty() && !player_2.is_empty() {
        let c1 = player_1.pop_front().unwrap();
        let c2 = player_2.pop_front().unwrap();

        if c1 > c2 {
            player_1.push_back(c1);
            player_1.push_back(c2);
        } else {
            player_2.push_back(c2);
            player_2.push_back(c1);
        }
    }

    match player_1.is_empty() {
        false => player_1,
        true => player_2,
    }
}

fn part2(player_1: &mut VecDeque<u32>, player_2: &mut VecDeque<u32>) -> bool {
    let mut seen = HashSet::new();

    while !player_1.is_empty() && !player_2.is_empty() {
        let mut hasher = DefaultHasher::new();
        player_1.hash(&mut hasher);
        player_2.hash(&mut hasher);

        if !seen.insert(hasher.finish()) {
            return true;
        }

        let c1 = player_1.pop_front().unwrap();
        let c2 = player_2.pop_front().unwrap();

        let player_1_win = if player_1.len() as u32 >= c1 && player_2.len() as u32 >= c2 {
            let mut new_one = player_1.clone();
            let mut new_two = player_2.clone();
            new_one.truncate(c1 as _);
            new_two.truncate(c2 as _);
            part2(&mut new_one, &mut new_two)
        } else {
            c1 > c2
        };

        if player_1_win {
            player_1.push_back(c1);
            player_1.push_back(c2);
        } else {
            player_2.push_back(c2);
            player_2.push_back(c1);
        }
    }

    player_2.is_empty()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (mut player_1, mut player_2) = parse(&input);

    let mut part_1_cards = part1(player_1.clone(), player_2.clone());
    let part_1_answer: u32 = (1..=part_1_cards.len() as u32)
        .rev()
        .map(|weight| weight * part_1_cards.pop_front().unwrap())
        .sum();

    println!("{:?}", part_1_answer);

    let player_1_wins = part2(&mut player_1, &mut player_2);
    let mut winner;

    if player_1_wins {
        winner = player_1;
    } else {
        winner = player_2;
    }

    let part_2_answer: u32 = (1..=winner.len() as u32)
        .rev()
        .map(|weight| weight * winner.pop_front().unwrap())
        .sum();

    println!("{:?}", part_2_answer);
}
