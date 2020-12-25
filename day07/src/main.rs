use petgraph::Directed;
use petgraph::{algo::has_path_connecting, graphmap::GraphMap};
use regex::Regex;
use std::io::{self, Read};

#[macro_use]
extern crate lazy_static;

const GOAL: &str = "shiny gold";

lazy_static! {
    static ref LINE_RE: Regex = Regex::new(r"(\w+ \w+) bags contain (.*)").unwrap();
    static ref ITEM_RE: Regex = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();
}

pub fn parse<'a, I>(lines: I) -> GraphMap<&'a str, u32, Directed>
where
    I: IntoIterator<Item = &'a str>,
{
    let mut graph: GraphMap<&str, u32, Directed> = GraphMap::new();

    for line in lines {
        if let Some((item, items)) = LINE_RE
            .captures(&line)
            .and_then(|captures| Some((captures.get(1)?.as_str(), captures.get(2)?.as_str())))
        {
            ITEM_RE.captures_iter(items).for_each(|captures| {
                graph.add_edge(
                    item,
                    captures.get(2).unwrap().as_str(),
                    captures.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                );
            });
        }
    }

    graph
}

fn part1(graph: &GraphMap<&str, u32, Directed>) -> usize {
    let count = graph
        .nodes()
        .filter(|&node| node != GOAL && has_path_connecting(&graph, node, GOAL, None))
        .count();

    count
}

fn part2(graph: &GraphMap<&str, u32, Directed>, node: &str) -> u32 {
    graph
        .edges(node)
        .map(|(_outer, inner, count)| part2(graph, inner) * count + count)
        .sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let graph = parse(input.lines());
    println!("{:?}", part1(&graph));
    println!("{:?}", part2(&graph, GOAL));
}
