use std::io::{self, Read};

pub fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

pub fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

pub fn chinese_remainder(buses: &Vec<(i64, i64)>) -> Option<i64> {
    let prod = buses.iter().map(|&(modulus, _)| modulus).product::<i64>();

    let mut sum = 0;

    for &(modulus, residue) in buses {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

pub fn part1(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().into_iter().collect();
    let timestamp = lines[0].parse::<u64>().unwrap();

    let mut bus_departures: Vec<(u64, u64)> = lines[1]
        .split(",")
        .filter_map(|part| part.parse::<u64>().ok())
        .filter(|&id| ((timestamp / id) * id) + id >= timestamp)
        .map(|id| (id, (timestamp / id) * id + id))
        .collect();

    bus_departures.sort_by(|&(_, dep_a), &(_, dep_b)| dep_a.cmp(&dep_b));

    (bus_departures[0].1 - timestamp) * bus_departures[0].0
}

pub fn part2(input: &str) -> i64 {
    let buses: Vec<(i64, i64)> = input
        .lines()
        .into_iter()
        .last()
        .unwrap()
        .split(",")
        .enumerate()
        .filter_map(|(offset, bus_id)| match bus_id.parse::<i64>() {
            Ok(id) => Some((id, id - offset as i64)),
            _ => None,
        })
        .collect();

    chinese_remainder(&buses).unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{:?}", part1(&input));
    println!("{:?}", part2(&input));
}
