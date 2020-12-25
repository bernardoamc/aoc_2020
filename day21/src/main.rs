use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref CONTAINS_RE: Regex = Regex::new(r"^(.*) \(contains (.*)\)$").unwrap();
}

fn parse(input: &str) -> (HashMap<&str, usize>, HashMap<&str, HashSet<&str>>) {
    let mut ingredients_frequency = HashMap::new();
    let mut allergens_to_ingredients: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in input.lines() {
        let matches = CONTAINS_RE.captures(line).unwrap();
        let ingredients = matches
            .get(1)
            .unwrap()
            .as_str()
            .split(' ')
            .collect::<HashSet<&str>>();

        let allergens = matches
            .get(2)
            .unwrap()
            .as_str()
            .split(", ")
            .collect::<Vec<&str>>();

        ingredients
            .iter()
            .for_each(|ingredient| *ingredients_frequency.entry(*ingredient).or_insert(0) += 1);

        for allergen in allergens {
            match allergens_to_ingredients.get(allergen) {
                Some(ingredients_per_allergen) => allergens_to_ingredients.insert(
                    allergen,
                    ingredients_per_allergen
                        .to_owned()
                        .intersection(&ingredients)
                        .copied()
                        .collect(),
                ),
                None => allergens_to_ingredients.insert(allergen, ingredients.clone()),
            };
        }
    }

    (ingredients_frequency, allergens_to_ingredients)
}

fn part1(
    ingredients_frequency: &HashMap<&str, usize>,
    allergens_to_ingredients: &HashMap<&str, HashSet<&str>>,
) -> usize {
    ingredients_frequency
        .iter()
        .filter(|&(ingredient, _)| {
            !allergens_to_ingredients
                .values()
                .any(|ingredients_per_allergen| ingredients_per_allergen.contains(ingredient))
        })
        .map(|(_, frequency)| *frequency)
        .sum()
}

fn allergen_with_multiple_ingredients(
    allergens_to_ingredients: &HashMap<&str, HashSet<&str>>,
) -> bool {
    allergens_to_ingredients
        .values()
        .any(|ingredients| ingredients.len() > 1)
}

fn part2(mut allergens_to_ingredients: HashMap<&str, HashSet<&str>>) -> String {
    while allergen_with_multiple_ingredients(&allergens_to_ingredients) {
        let ingredients_for_allergen = allergens_to_ingredients
            .values()
            .find(|set| set.len() == 1)
            .unwrap()
            .to_owned();

        allergens_to_ingredients = allergens_to_ingredients
            .into_iter()
            .map(|(i, s)| match s.len() {
                1 => (i, s),
                _ => (
                    i,
                    s.difference(&ingredients_for_allergen).copied().collect(),
                ),
            })
            .collect();
    }

    let mut matches = allergens_to_ingredients
        .iter()
        .map(|(k, v)| (k, v.iter().next().unwrap()))
        .collect::<Vec<(&&str, &&str)>>();

    matches.sort();

    matches.iter().fold(String::new(), |mut canonical, ing| {
        canonical = format!("{},{}", canonical, ing.1);
        canonical
    })[1..]
        .to_string()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (ingredients_frequency, allergens_to_ingredients) = parse(&input);

    println!(
        "{:?}",
        part1(&ingredients_frequency, &allergens_to_ingredients)
    );

    println!("{:?}", part2(allergens_to_ingredients));
}
