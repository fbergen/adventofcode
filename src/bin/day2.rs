#[derive(Debug)]
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
            .map(|p| parse_password_row(p).unwrap())
            .filter(|p| validate_password_part1(p))
            .into_iter()
            .count()
    );

    println!(
        "Num valid part 2: {}",
        input
            .into_iter()
            .map(|p| parse_password_row(p).unwrap())
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

pub fn parse_password_row(p_str: &str) -> Option<PasswordPolicy> {
    // input example "3-7 g: gdgtnfggq",
    // format min-max c: s

    let components: Vec<&str> = p_str.split(' ').collect();
    let min_max: Vec<&str> = components[0].split('-').collect();
    let c: char = components[1].chars().collect::<Vec<char>>()[0];
    let pass = components[2];

    Some(PasswordPolicy {
        min: min_max[0].parse::<usize>().ok()?,
        max: min_max[1].parse::<usize>().ok()?,
        c: c,
        pass: pass.to_string(),
    })
}
