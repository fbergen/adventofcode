use recap::Recap;
use serde::Deserialize;

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r"(?P<min>\d+)-(?P<max>\d+) (?P<c>\w): (?P<pass>\w+)")]
pub struct PasswordPolicy {
    c: char,
    min: usize,
    max: usize,
    pass: String,
}

pub fn main() {
    let input: Vec<PasswordPolicy> = include_str!("../inputs/day2")
        .lines()
        .map(|p| p.parse::<PasswordPolicy>().unwrap())
        .collect();

    println!(
        "Num valid\npart 1: {}\npart 2: {}",
        input.iter().filter(|p| validate_password_part1(&p)).count(),
        input.iter().filter(|p| validate_password_part2(&p)).count()
    );
}

pub fn validate_password_part1(p: &PasswordPolicy) -> bool {
    let num: usize = p.pass.chars().filter(|c| *c == p.c).count();
    num >= p.min && num <= p.max
}

pub fn validate_password_part2(p: &PasswordPolicy) -> bool {
    (p.pass.chars().nth(p.min - 1).unwrap() == p.c)
        ^ (p.pass.chars().nth(p.max - 1).unwrap() == p.c)
}
