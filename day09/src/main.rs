use std::io::{self, Read};

pub fn pair_exists(numbers: &Vec<u64>, number: u64) -> bool {
    let mut start = 0;
    let mut end = numbers.len() - 1;

    while start != end {
        let sum = numbers[start] + numbers[end];

        if sum > number {
            end -= 1;
        } else if sum < number {
            start += 1;
        } else {
            return true;
        }
    }

    false
}

pub fn find_subset(numbers: &Vec<u64>, search: u64, position: usize) -> (usize, usize) {
    let mut current_sum: u64 = numbers[0];
    let mut start: usize = 0;

    for i in 1..position {
        while current_sum > search && start < i - 1 {
            current_sum -= numbers[start];
            start += 1;
        }

        if current_sum == search {
            return (start, i);
        }

        current_sum += numbers[i];
    }

    panic!("Subset not found!");
}

pub fn find_invalid(numbers: &Vec<u64>, preamble: usize) -> (u64, usize) {
    for (position, subset) in numbers.windows(preamble).enumerate() {
        let next_number = numbers[(preamble + position)];
        let mut subset: Vec<u64> = subset.iter().map(|n| n.clone()).collect();
        subset.sort();

        if !pair_exists(&subset, next_number) {
            return (next_number, (preamble + position));
        }
    }

    panic!("Nothing wrong with the input!");
}
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let numbers: Vec<u64> = input.lines().map(|x| x.parse::<u64>().unwrap()).collect();
    let (invalid, position) = find_invalid(&numbers, 25);
    let (start, end) = find_subset(&numbers, invalid, position);

    let mut subset: Vec<u64> = numbers[start..end].into_iter().map(|n| n.clone()).collect();
    subset.sort();
    let part2 = subset[0] + subset[subset.len() - 1];

    println!("{:?}", invalid);
    println!("{:?}", part2);
}
