use std::collections::HashMap;
use std::io::{self, Read};

pub fn parse_ints(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}

pub fn part1(numbers: &Vec<i64>) -> i64 {
    let mut start = 0;
    let mut end = numbers.len() - 1;

    loop {
        let sum = numbers[start] + numbers[end];

        if sum > 2020 {
            end -= 1;
        } else if sum < 2020 {
            start += 1;
        } else {
            break;
        }
    }

    numbers[start] * numbers[end]
}

pub fn part2(numbers: &Vec<i64>) -> i64 {
    let len = numbers.len();

    for i in 0..len - 1 {
        let mut cache: HashMap<i64, bool> = HashMap::new();

        for j in i + 1..len {
            let looking_for = 2020 - numbers[i] - numbers[j];

            if cache.contains_key(&looking_for) {
                return numbers[i] * numbers[j] * looking_for;
            }

            cache.insert(numbers[j], true);
        }
    }

    return 0;
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut values = parse_ints(&input);
    values.sort();
    println!("{:?}", part1(&values));
    println!("{:?}", part2(&values));
}
