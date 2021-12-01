#[macro_use]
extern crate lazy_static;

use regex::Regex;
use serde::Deserialize;
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct Node {
    parent: String,
    child: Vec<(u64, String)>,
}

impl FromStr for Node {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REPARENT: Regex = Regex::new(r"^(\w+\s\w+) bags contain").unwrap();
            static ref RECHILD: Regex = Regex::new(r"(\d) (\w+\s\w+) bags?[,.]").unwrap();
        }

        Ok(Node {
            parent: REPARENT
                .captures(s)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .to_string(),
            child: RECHILD
                .captures_iter(s)
                .map(|c| {
                    (
                        c.get(1).unwrap().as_str().parse::<u64>().unwrap(),
                        c.get(2).unwrap().as_str().to_string(),
                    )
                })
                .collect(),
        })
    }
}

type Graph = HashMap<String, Vec<(u64, String)>>;
type Cache = HashMap<String, u64>;

fn main() {
    part1();
    part2();
}

fn part2() {
    let input: Vec<Node> = include_str!("../inputs/day7")
        .lines()
        .map(|p| p.parse::<Node>().unwrap())
        .collect();

    let graph: Graph = input
        .iter()
        .map(|x| (x.parent.to_string(), x.child.clone()))
        .collect();

    let mut cache: Cache = HashMap::new();

    println!("{}", num_inside_bag(&mut cache, &graph, "shiny gold") - 1);
}

fn num_inside_bag(cache: &mut Cache, graph: &Graph, node: &str) -> u64 {
    if let Some(c) = cache.get(node) {
        return *c;
    }

    let mut ret = 1;
    for (num, child) in graph.get(node).unwrap() {
        ret += num * num_inside_bag(cache, graph, child);
    }

    cache.insert(node.to_string(), ret);
    ret
}

fn part1() {
    let input: Vec<Node> = include_str!("../inputs/day7")
        .lines()
        .map(|p| p.parse::<Node>().unwrap())
        .collect();
    let mut contained_in: HashMap<String, Vec<String>> = HashMap::new();
    for n in input.iter() {
        for c in n.child.iter() {
            contained_in
                .entry(c.1.to_string())
                .or_insert(vec![])
                .push(n.parent.to_string());
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back("shiny gold");
    let empty_vec: Vec<String> = vec![];

    let mut visited = vec!["shiny gold".to_string()];
    while !queue.is_empty() {
        let next = queue.pop_front().unwrap();

        for n in contained_in.get(next).unwrap_or(&empty_vec) {
            if !visited.contains(n) {
                visited.push(n.to_string());
                queue.push_back(n);
            }
        }
    }

    println!("{:?}", visited.len() - 1);
}
