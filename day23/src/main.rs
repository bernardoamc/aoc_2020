use std::collections::VecDeque;
use std::io::{self, Read};

fn part1(input: &str) {
    let mut cups: VecDeque<usize> = input
        .split("")
        .filter_map(|n| n.parse::<usize>().ok())
        .collect();

    let cups_len = cups.len();

    for _ in 0..100 {
        let current = cups.pop_front().unwrap();
        let next_three = cups.drain(..3).collect::<Vec<usize>>();
        cups.push_back(current);

        let mut order: Vec<(usize, usize)> = cups
            .iter()
            .map(|&value| {
                if value > current {
                    current + cups_len - value
                } else {
                    current - value
                }
            })
            .enumerate()
            .collect();

        order.sort_by(|(_, a), (_, b)| a.cmp(b));
        let (destination, _) = order.iter().skip(1).next().unwrap();

        (0..3).for_each(|index| cups.insert(destination + index + 1, next_three[index]));
    }

    let one_position = cups.iter().position(|&value| value == 1).unwrap();
    cups.rotate_left(one_position);

    cups.iter().skip(1).for_each(|value| print!("{:?}", value));
    println!();
}

fn parse(input: &str) -> (Vec<usize>, usize) {
    let mut numbers: Vec<usize> = input
        .split("")
        .filter_map(|n| n.parse::<usize>().ok())
        .collect();

    (numbers.len() + 1..=1_000_000).for_each(|value| numbers.push(value));

    let mut link_list = vec![0; numbers.len() + 1];
    for i in 0..numbers.len() {
        link_list[numbers[i]] = numbers[(i + 1) % numbers.len()];
    }

    (link_list, numbers[0])
}

fn part2(mut link_list: Vec<usize>, start: usize) {
    let mut current_value = start;
    let max_deck = link_list.len() - 1;

    for _ in 0..10_000_000 {
        let mut next_three = vec![0; 3];

        let mut value_next = current_value;
        for i in 0..3 {
            value_next = link_list[value_next];
            next_three[i] = value_next;
        }

        link_list[current_value] = link_list[next_three[2]];
        let mut destination = current_value;

        loop {
            destination = if destination > 1 {
                destination - 1
            } else {
                max_deck
            };
            if !next_three.contains(&destination) {
                break;
            }
        }

        let old_next = link_list[destination];
        link_list[destination] = next_three[0];
        link_list[next_three[2]] = old_next;

        current_value = link_list[current_value];
    }

    let after_one = link_list[1];
    let next = link_list[after_one];
    println!("{:?}", after_one * next);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part1(&input);
    let (link_list, start) = parse(&input);
    part2(link_list, start);
}
