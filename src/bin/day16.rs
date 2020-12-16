use recap::Recap;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r"^(?P<name>[^:]+): (?P<from1>\d+)-(?P<to1>\d+) or (?P<from2>\d+)-(?P<to2>\d+)$")]
pub struct Rule {
    name: String,
    from1: usize,
    to1: usize,
    from2: usize,
    to2: usize,
}

impl Rule {
    fn is_valid(&self, num: &usize) -> bool {
        (self.from1..=self.to1).contains(num) || (self.from2..=self.to2).contains(num)
    }
}

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r"^(?P<vals>[\d,]+)$")]
pub struct Ticket {
    vals: Vec<usize>,
}

fn main() {
    let input: &str = include_str!("../inputs/day16");
    println!("part1: {}", solve(input, true).unwrap());
    println!("part2: {}", solve(input, false).unwrap());
}

fn solve(input_str: &str, part1: bool) -> Option<usize> {
    let mut sections = input_str.split("\n\n");

    let rules: Vec<Rule> = sections
        .next()?
        .lines()
        .map(|p| p.parse::<Rule>().unwrap())
        .collect();
    let my_ticket: Ticket = sections
        .next()?
        .lines()
        .skip(1)
        .next()?
        .parse::<Ticket>()
        .unwrap();
    let nearby_tickets: Vec<Ticket> = sections
        .next()?
        .lines()
        .skip(1)
        .map(|p| p.parse::<Ticket>().unwrap())
        .collect();

    if part1 {
        let mut sum = 0;
        for t in nearby_tickets {
            for v in &t.vals {
                if rules.iter().all(|r| !r.is_valid(v)) {
                    sum += v;
                }
            }
        }
        return Some(sum);
    }

    let valid_tickets: Vec<&Ticket> = nearby_tickets
        .iter()
        .filter(|t| t.vals.iter().all(|v| rules.iter().any(|r| r.is_valid(v))))
        .collect();

    let mut idx_to_rule: HashMap<usize, &Rule> = HashMap::new();
    while idx_to_rule.len() < my_ticket.vals.len() {
        for r in &rules {
            let mut count = 0;
            let mut idx = 0;
            for i in 0..my_ticket.vals.len() {
                if !idx_to_rule.contains_key(&i) {
                    let all_valid = valid_tickets.iter().all(|t| r.is_valid(&t.vals[i]));
                    if all_valid {
                        count += 1;
                        idx = i;
                    }
                }
            }
            if count == 1 {
                idx_to_rule.insert(idx, r);
            }
        }
    }

    let ret = my_ticket
        .vals
        .iter()
        .enumerate()
        .filter(|(i, _v)| idx_to_rule[i].name.starts_with("departure"))
        .map(|(_i, v)| v)
        .product();

    Some(ret)
}

#[cfg(test)]
mod test {
    use super::*;

    const TESTCASE: &str = "\
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    #[test]
    fn test_day16() {
        assert_eq!(solve(TESTCASE, true).unwrap(), 71);
    }
}
