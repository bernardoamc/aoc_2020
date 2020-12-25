use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref REQUIRED_FIELDS: HashSet<&'static str> = {
        let mut required_fields = HashSet::new();
        required_fields.insert("byr");
        required_fields.insert("iyr");
        required_fields.insert("eyr");
        required_fields.insert("hgt");
        required_fields.insert("hcl");
        required_fields.insert("ecl");
        required_fields.insert("pid");
        required_fields
    };
}

macro_rules! validate_or_return {
    ( $e:expr ) => {
        match $e {
            true => true,
            false => return false,
        }
    };
}

pub fn rule(data: &str, constraint: fn(u64) -> bool) -> bool {
    match data.parse::<u64>() {
        Ok(value) => constraint(value),
        _ => false,
    }
}

pub fn valid(data: &HashMap<&str, &str>) -> bool {
    lazy_static! {
        static ref HGT_RE: Regex = Regex::new(r"^(\d{3})cm$|^(\d{2})in$").unwrap();
        static ref HCL_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        static ref ECL_RE: Regex =
            Regex::new(r"^amb$|^blu$|^brn$|^gry$|^grn$|^hzl$|^oth$").unwrap();
        static ref PID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
    }

    if !HGT_RE.is_match(&data["hgt"]) {
        return false;
    }

    if !HCL_RE.is_match(&data["hcl"]) {
        return false;
    }

    if !ECL_RE.is_match(&data["ecl"]) {
        return false;
    }

    if !PID_RE.is_match(&data["pid"]) {
        return false;
    }

    validate_or_return!(rule(data["byr"], |year| year >= 1920 && year <= 2002));
    validate_or_return!(rule(data["iyr"], |year| year >= 2010 && year <= 2020));
    validate_or_return!(rule(data["eyr"], |year| year >= 2020 && year <= 2030));

    let caps = HGT_RE.captures(&data["hgt"]).unwrap();
    match caps.get(1) {
        Some(height_cm) => {
            validate_or_return!(rule(height_cm.as_str(), |size| (size >= 150 && size <= 193)));
        }
        None => {
            let height_in = caps.get(2).unwrap();
            validate_or_return!(rule(height_in.as_str(), |size| (size >= 59 && size <= 76)));
        }
    };

    true
}

pub fn part1(input: &str) -> u64 {
    let mut passport_fields: HashSet<&str> = HashSet::new();
    let mut count: u64 = 0;

    for line in input.lines() {
        if line.is_empty() {
            if REQUIRED_FIELDS.is_subset(&passport_fields) {
                count += 1;
            }
            passport_fields = HashSet::new();
            continue;
        }

        for entry in line.split_whitespace() {
            let field: &str = entry.split(":").collect::<Vec<&str>>().first().unwrap();
            passport_fields.insert(field);
        }
    }

    count
}

pub fn part2(input: &str) -> u64 {
    let mut passport_fields: HashSet<&str> = HashSet::new();
    let mut passport_data: HashMap<&str, &str> = HashMap::new();
    let mut count: u64 = 0;

    for line in input.lines() {
        if line.is_empty() {
            if REQUIRED_FIELDS.is_subset(&passport_fields) && valid(&passport_data) {
                count += 1;
            }

            passport_fields = HashSet::new();
            passport_data = HashMap::new();

            continue;
        }

        for entry in line.split_whitespace() {
            let data = entry.split(":").collect::<Vec<&str>>();
            let field = data.first().unwrap();
            let value = data.last().unwrap();
            passport_fields.insert(field);
            passport_data.insert(field, value);
        }
    }

    count
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    input.push('\n');
    println!("{:?}", part1(&input));
    println!("{:?}", part2(&input));
}
