use std::collections::HashSet;
use std::io::{self, Read};
use std::iter::FromIterator;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut ids = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        let row = u32::from_str_radix(&line[..7].replace("F", "0").replace("B", "1"), 2).unwrap();
        let column =
            u32::from_str_radix(&line[7..].replace("L", "0").replace("R", "1"), 2).unwrap();

        ids.push((row * 8) + column);
    }

    ids.sort();
    println!("{:?}", ids.last());

    let set: HashSet<&u32> = HashSet::from_iter(ids.iter());

    for id in ids[0]..ids.last().unwrap().to_owned() {
        if !set.contains(&(id + 1)) && set.contains(&(id + 2)) {
            println!("{:?}", id + 1);
        }
    }
}
