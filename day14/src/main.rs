use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Read};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref MASK_RE: Regex = Regex::new(r"^mask = (\w+)$").unwrap();
    static ref MEM_RE: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)$").unwrap();
}

pub fn part1<'a, I>(lines: I) -> u64
where
    I: IntoIterator<Item = &'a str>,
{
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut or_active_mask: u64 = 0;
    let mut and_active_mask: u64 = 0;

    for line in lines {
        if let Some(mask) = MASK_RE
            .captures(&line)
            .and_then(|captures| Some(captures.get(1)?.as_str()))
        {
            or_active_mask = u64::from_str_radix(&mask.clone().replace("X", "0"), 2).unwrap();
            and_active_mask = u64::from_str_radix(&mask.clone().replace("X", "1"), 2).unwrap();
        }

        if let Some((location, value)) = MEM_RE
            .captures(&line)
            .and_then(|captures| Some((captures.get(1)?.as_str(), captures.get(2)?.as_str())))
        {
            let location = location.parse::<u64>().unwrap();
            let value = value.parse::<u64>().unwrap();

            memory.insert(location, (value | or_active_mask) & and_active_mask);
        }
    }

    memory.values().sum()
}

fn floating_combinations(mask: u64, location: u64, value: u64, memory: &mut HashMap<u64, u64>) {
    if mask == 0 {
        return;
    }

    let x = mask & (!mask + 1); // get right-most ‘1’ of mask
    let mask = mask & !x; // clear right-most ‘1’ of mask

    memory.insert(location & !x, value);
    memory.insert(location | x, value);
    floating_combinations(mask, location & !x, value, memory);
    floating_combinations(mask, location | x, value, memory);
}

pub fn part2<'a, I>(lines: I) -> u64
where
    I: IntoIterator<Item = &'a str>,
{
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut floating_active_mask: u64 = 0;
    let mut or_active_mask: u64 = 0;

    for line in lines {
        if let Some(mask) = MASK_RE
            .captures(&line)
            .and_then(|captures| Some(captures.get(1)?.as_str()))
        {
            or_active_mask = u64::from_str_radix(&mask.clone().replace("X", "0"), 2).unwrap();

            let floating_mask = &mask.clone().replace("1", "0");
            floating_active_mask =
                u64::from_str_radix(&floating_mask.replace("X", "1"), 2).unwrap();
        }

        if let Some((location, value)) = MEM_RE
            .captures(&line)
            .and_then(|captures| Some((captures.get(1)?.as_str(), captures.get(2)?.as_str())))
        {
            let location = location.parse::<u64>().unwrap();
            let value = value.parse::<u64>().unwrap();

            memory.insert(location | or_active_mask, value);

            floating_combinations(
                floating_active_mask,
                location | or_active_mask,
                value,
                &mut memory,
            );
        }
    }

    memory.values().sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{:?}", part1(input.lines()));
    println!("{:?}", part2(input.lines()));
}
