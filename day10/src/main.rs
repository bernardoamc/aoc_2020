use std::io::{self, Read};

pub fn part1(numbers: &Vec<usize>) -> usize {
    let (mut ones, mut threes) = (0, 0);

    for window in numbers.windows(2) {
        match window[1] - window[0] {
            1 => ones += 1,
            3 => threes += 1,
            _ => (),
        }
    }

    ones * threes
}

pub fn part2(mut numbers: Vec<usize>) -> usize {
    let last_index = numbers.len() - 1;

    for i in 0..numbers.len() {
        numbers[i] = numbers[i + 1..]
            .iter()
            .take_while(|n| (**n - numbers[i]) <= 3)
            .count();
    }

    numbers[last_index] = 1;

    (0..last_index)
        .rev()
        .for_each(|i| numbers[i] = numbers[i + 1..i + 1 + numbers[i]].iter().sum());

    numbers[0]
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut numbers: Vec<usize> = input.lines().map(|x| x.parse::<usize>().unwrap()).collect();
    numbers.push(0);
    numbers.sort();
    numbers.push(numbers.last().unwrap() + 3);

    println!("{:?}", part1(&numbers));
    println!("{:?}", part2(numbers));
}
