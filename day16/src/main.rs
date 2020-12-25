use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::Into;
use std::io::{self, Read};
use std::ops::RangeInclusive;
use std::str::FromStr;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref TICKET_RULE_RE: Regex = Regex::new(r"^(.*): (\d+)\-(\d+) or (\d+)-(\d+)$").unwrap();
}

type Ticket = Vec<u32>;

#[derive(Debug, Clone)]
struct Rule {
    first: RangeInclusive<u32>,
    second: RangeInclusive<u32>,
}

#[derive(Debug, Clone)]
struct Notes {
    rules: Vec<(String, Rule)>,
    your: Ticket,
    nearby: Vec<Ticket>,
}

impl Notes {
    fn in_any_rule(&self, target: &u32) -> bool {
        self.rules
            .iter()
            .any(|(_, rule)| rule.first.contains(target) || rule.second.contains(target))
    }
}

impl Into<Notes> for &str {
    fn into(self) -> Notes {
        let mut notes = Notes {
            rules: Vec::new(),
            your: Ticket::new(),
            nearby: Vec::new(),
        };

        let mut your_ticket_start = false;
        let mut nearby_tickets_start = false;

        for line in self.lines().map(|line| line.trim()) {
            match TICKET_RULE_RE.captures(line) {
                Some(capture) => notes.rules.push((
                    capture[1].to_string(),
                    Rule {
                        first: u32::from_str(&capture[2]).unwrap()
                            ..=u32::from_str(&capture[3]).unwrap(),
                        second: u32::from_str(&capture[4]).unwrap()
                            ..=u32::from_str(&capture[5]).unwrap(),
                    },
                )),
                None => match line {
                    "your ticket:" => your_ticket_start = true,
                    "nearby tickets:" => nearby_tickets_start = true,
                    _ if line.is_empty() => continue,
                    _ => {
                        let ticket: Ticket = line
                            .split(",")
                            .filter_map(|entry| u32::from_str(entry).ok())
                            .collect();

                        if nearby_tickets_start {
                            notes.nearby.push(ticket);
                        } else if your_ticket_start {
                            notes.your = ticket;
                        }
                    }
                },
            };
        }

        notes
    }
}

pub fn part1(src: &str) -> u32 {
    let notes: Notes = src.into();

    notes
        .nearby
        .iter()
        .map(|ticket| {
            ticket
                .iter()
                .filter(|value| !notes.in_any_rule(value))
                .sum::<u32>()
        })
        .sum::<u32>()
}

pub fn part2(src: &str) -> u64 {
    let notes: Notes = src.into();

    let valid_tickets: Vec<Ticket> = notes
        .nearby
        .iter()
        .filter(|ticket| ticket.iter().all(|value| notes.in_any_rule(value)))
        .map(|ticket| ticket.clone())
        .collect::<Vec<Ticket>>();

    let total_positions = notes.your.len();
    let mut inferred_positions = vec![false; total_positions];
    let mut inferred_rules: HashMap<&str, u32> = HashMap::new();
    let mut positions_inferred_count = 0;

    while positions_inferred_count != total_positions {
        for position in 0..total_positions {
            if inferred_positions[position] {
                continue;
            }

            let mut matching_rules = 0;
            let mut last_rule_matched = "";

            for (name, rule) in notes.rules.iter() {
                if inferred_rules.contains_key(name.as_str()) {
                    continue;
                }

                let any_invalid = valid_tickets.iter().any(|ticket| {
                    !rule.first.contains(&ticket[position])
                        && !rule.second.contains(&ticket[position])
                });

                if any_invalid {
                    continue;
                }

                matching_rules += 1;
                last_rule_matched = &name;
            }

            if matching_rules == 1 {
                inferred_rules.insert(last_rule_matched.clone(), notes.your[position]);
                inferred_positions[position] = true;
                positions_inferred_count += 1;
            }
        }
    }

    inferred_rules
        .iter()
        .filter(|(name, _)| name.starts_with("departure"))
        .fold(1, |acc, (_, &value)| acc * value as u64)
}

// 1. Find all valid tickets
// 2. Find the list of valid positions for each rule and store it in a vector
// 3. Sort from lower to higher the previous vector
// 4. For each rule (starting with the one with the lowest amount of valid positions)
//  - Pick a position from the valid list if it hasn't been picked yet
//  - Check If rule starts with "departure" and multiply accordingly
//  - Mark position as picked
pub fn part2_alternative(src: &str) -> u64 {
    let notes: Notes = src.into();

    let valid_tickets: Vec<Ticket> = notes
        .nearby
        .iter()
        .filter(|ticket| ticket.iter().all(|value| notes.in_any_rule(value)))
        .map(|ticket| ticket.clone())
        .collect::<Vec<Ticket>>();

    let mut inferred_rules_matches: Vec<(&str, Vec<u64>)> = Vec::new();

    for (name, rule) in notes.rules.iter() {
        let mut valid_positions = Vec::new();

        for position in 0..notes.your.len() {
            let any_invalid = valid_tickets.iter().any(|ticket| {
                !rule.first.contains(&ticket[position]) && !rule.second.contains(&ticket[position])
            });

            if any_invalid {
                continue;
            }

            valid_positions.push(position as u64);
        }

        inferred_rules_matches.push((name, valid_positions));
    }

    inferred_rules_matches.sort_by(|(_, count_a), (_, count_b)| count_a.len().cmp(&count_b.len()));

    let mut filled_positions: HashSet<u64> = HashSet::new();
    let mut total: u64 = 1;

    for position in 0..notes.your.len() {
        let selected_position = inferred_rules_matches[position]
            .1
            .iter()
            .find(|matched_pos| !filled_positions.contains(&matched_pos))
            .unwrap();

        if inferred_rules_matches[position].0.starts_with("departure") {
            total *= notes.your[*selected_position as usize] as u64;
        }

        filled_positions.insert(*selected_position);
    }

    total
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{:?}", part1(&input));
    println!("{:?}", part2(&input));
    println!("{:?}", part2_alternative(&input));
}
