use std::collections::HashMap;
use std::io::{self, Read};

fn atoi(constraint: &str) -> usize {
    constraint.parse().unwrap()
}

#[derive(Debug, Clone)]
enum Rule {
    Unit(char),
    SubRules(Vec<usize>),
    Or(Vec<usize>, Vec<usize>),
}

fn parse_rules(rules: &str) -> HashMap<usize, Rule> {
    rules
        .lines()
        .map(|s| {
            let parts: Vec<&str> = s.split(": ").collect();
            (atoi(parts[0]), parse_rule(parts[1]))
        })
        .collect()
}

fn parse_rule(rule: &str) -> Rule {
    if rule.contains('"') {
        Rule::Unit(rule.chars().nth(1).unwrap())
    } else if rule.contains('|') {
        let sub_rules: Vec<Vec<usize>> = rule
            .split(" | ")
            .map(|sub_rule| sub_rule.split(" ").map(atoi).collect())
            .collect();

        Rule::Or(sub_rules[0].clone(), sub_rules[1].clone())
    } else {
        Rule::SubRules(rule.split(" ").map(atoi).collect())
    }
}

fn match_unit(
    unit: char,
    message: &str,
    rules: &HashMap<usize, Rule>,
    stack: &mut Vec<usize>,
) -> bool {
    match message.chars().next() {
        Some(candidate) if candidate == unit => is_match(&message[1..], rules, stack),
        _ => false,
    }
}

fn match_sub_rules(
    sub_rules: &[usize],
    message: &str,
    rules: &HashMap<usize, Rule>,
    stack: &mut Vec<usize>,
) -> bool {
    sub_rules
        .iter()
        .rev()
        .for_each(|rule_number| stack.push(*rule_number));

    is_match(message, rules, stack)
}

fn is_match(message: &str, rules: &HashMap<usize, Rule>, stack: &mut Vec<usize>) -> bool {
    if stack.is_empty() && message.is_empty() {
        return true;
    }
    if stack.is_empty() || message.is_empty() {
        return false;
    }

    let rule_number = stack.pop().unwrap();
    let rule = rules.get(&rule_number).unwrap();

    match rule {
        Rule::Unit(unit) => match_unit(*unit, message, rules, stack),
        Rule::SubRules(sub_rules) => match_sub_rules(sub_rules, message, rules, stack),
        Rule::Or(sub_rules_group1, sub_rules_group2) => {
            match_sub_rules(sub_rules_group1, message, rules, &mut stack.clone())
                || match_sub_rules(sub_rules_group2, message, rules, &mut stack.clone())
        }
    }
}

fn part1(input: &str) -> usize {
    let groups: Vec<&str> = input.split("\n\n").collect();
    let rules = parse_rules(groups[0]);

    groups[1]
        .lines()
        .filter(|message| is_match(message, &rules, &mut vec![0]))
        .count()
}

fn part2(input: &str) -> usize {
    let groups: Vec<&str> = input.split("\n\n").collect();
    let mut rules = parse_rules(groups[0]);

    rules.insert(8, Rule::Or(vec![42], vec![42, 8]));
    rules.insert(11, Rule::Or(vec![42, 31], vec![42, 11, 31]));

    groups[1]
        .lines()
        .filter(|message| is_match(message, &rules, &mut vec![0]))
        .count()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{:?}", part1(&input));
    println!("{:?}", part2(&input));
}
