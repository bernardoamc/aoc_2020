use std::io::{self, Read};

fn get_encryption_key(card_loop_size: u64, door_public_key: u64) -> u64 {
    (0..card_loop_size).fold(1, |mut encryption_key, _| {
        encryption_key *= door_public_key;
        encryption_key %= 20201227;
        encryption_key
    })
}

fn crack_loop_size(public_key: u64) -> u64 {
    let mut loop_size = 1;
    let mut value: u64 = 1;

    loop {
        value *= 7;
        value %= 20201227;
        if value == public_key {
            return loop_size;
        }
        loop_size += 1;
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let card_public_key: u64 = lines.next().unwrap().parse().unwrap();
    let door_public_key: u64 = lines.next().unwrap().parse().unwrap();
    let card_loop_size = crack_loop_size(card_public_key);

    println!("{:?}", get_encryption_key(card_loop_size, door_public_key));
}
