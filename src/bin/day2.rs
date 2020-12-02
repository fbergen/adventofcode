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
    let input: Vec<&str> = include_str!("..//inputs/day2").lines().collect();

    println!(
        "Num valid part 1: {}",
        input
            .iter()
            .map(|p| p.parse::<PasswordPolicy>().unwrap())
            .filter(|p| validate_password_part1(p))
            .into_iter()
            .count()
    );

    println!(
        "Num valid part 2: {}",
        input
            .into_iter()
            .map(|p| p.parse::<PasswordPolicy>().unwrap())
            .filter(|p| validate_password_part2(p))
            .into_iter()
            .count()
    );
}

pub fn validate_password_part1(p: &PasswordPolicy) -> bool {
    let num: usize = p.pass.chars().filter(|c| *c == p.c).count();
    let ret = num >= p.min && num <= p.max;
    // println!("{:?}, {}, {}", p, num, ret);
    ret
}

pub fn validate_password_part2(p: &PasswordPolicy) -> bool {
    let ret = (p.pass.chars().nth(p.min - 1).unwrap() == p.c)
        ^ (p.pass.chars().nth(p.max - 1).unwrap() == p.c);
    // println!("{:?}, {}", p, ret);
    ret
}
