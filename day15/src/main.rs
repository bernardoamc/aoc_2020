use std::collections::HashMap;
use std::io::{self, Read};

pub fn pick_nth(input: &str, nth_number: u32) -> u32 {
    let mut spoken: HashMap<u32, u32> = HashMap::new();
    let mut current_turn = 0;
    let mut last_seen = 0;

    let numbers: Vec<u32> = input
        .split(",")
        .filter_map(|v| v.parse::<u32>().ok())
        .collect();

    for number in numbers {
        last_seen = number;
        current_turn += 1;
        spoken.insert(last_seen, current_turn);
    }

    while current_turn < nth_number {
        last_seen = match spoken.insert(last_seen, current_turn) {
            Some(num) => current_turn - num,
            None => 0,
        };

        current_turn += 1;
    }

    last_seen
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{:?}", pick_nth(&input, 2020));
    println!("{:?}", pick_nth(&input, 30000000));
}
