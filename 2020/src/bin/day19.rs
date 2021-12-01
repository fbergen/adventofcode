#[macro_use]
extern crate lazy_static;

use parse_display::FromStr;
use pcre2;
use regex::Captures;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input: &str = include_str!("../inputs/day19");
    println!("part1: {}", solve(input, true).unwrap());
    println!("part2: {}", solve(input, false).unwrap());
}

#[derive(FromStr, PartialEq, Debug)]
#[from_str(regex = r"^(?P<id>\d+): (?P<m>.*)$")]
struct Rule {
    id: usize,
    m: String,
}

fn solve(input_str: &str, part1: bool) -> Option<usize> {
    let mut sect = input_str.split("\n\n");
    let mut rules: HashMap<usize, String> = sect
        .next()?
        .lines()
        .map(|r| {
            let r: Rule = r.parse().unwrap();
            (r.id, r.m.chars().filter(|c| *c != '"').collect::<String>())
        })
        .collect();

    if !part1 {
        rules.insert(8, "42+".to_string());
        // (?&name) where “name” is the name of a capturing group
        // Recursion of a capturing group to a capturing group.
        rules.insert(11, "(?<x>42(?&x)?31)".to_string());
    }
    let re = pcre2::bytes::Regex::new(&format!(
        "^{}$",
        get_re(&rules, 0)
            .chars()
            .filter(|c| *c != ' ')
            .collect::<String>()
    ))
    .unwrap();

    let messages: Vec<&str> = sect.next()?.lines().collect();
    Some(
        messages
            .iter()
            .filter(|m| re.is_match(m.as_bytes()).unwrap())
            .count(),
    )
}

fn get_re(rules: &HashMap<usize, String>, id: usize) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)").unwrap();
    }

    return RE
        .replace_all(&rules[&id], |c: &Captures| {
            format!("({})", get_re(rules, c[1].parse::<usize>().ok().unwrap()))
        })
        .to_string();
}

#[cfg(test)]
mod test {
    use super::*;

    const TESTCASE: &str = "\
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";

    #[test]
    fn test_day19() {
        assert_eq!(solve(TESTCASE, true).unwrap(), 2);
    }
}
