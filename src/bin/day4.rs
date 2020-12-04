#[macro_use]
extern crate lazy_static;

use recap::Recap;
use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r"(?x)^
    ((
        (byr:(?P<byr>[^\s\n]+))|
        (iyr:(?P<iyr>[^\s\n]+))|
        (eyr:(?P<eyr>[^\s\n]+))|
        (hgt:(?P<hgt>[^\s\n]+))|
        (hcl:(?P<hcl>[^\s\n]+))|
        (ecl:(?P<ecl>[^\s\n]+))|
        (pid:(?P<pid>[^\s\n]+))|
        (cid:(?P<cid>[^\s\n]+))
    )[\s\n]*)+$")]
pub struct PassportData {
    byr: String,         // (Birth Year)
    iyr: String,         // (Issue Year)
    eyr: String,         // (Expiration Year)
    hgt: String,         // (Height)
    hcl: String,         // (Hair Color)
    ecl: String,         // (Eye Color)
    pid: String,         // (Passport ID)
    cid: Option<String>, // (Country ID)
}

pub fn main() {
    let input: Vec<PassportData> = include_str!("../inputs/day4")
        .split("\n\n")
        .filter_map(|p| p.parse::<PassportData>().ok())
        .collect();

    let part1 = input.len();

    let part2 = input
        .into_iter()
        .filter(validate_passport_data_part2)
        .count();

    println!("Num valid\npart 1: {:?}\npart 2: {:?}", part1, part2);
}

pub fn validate_passport_data_part2(p: &PassportData) -> bool {
    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    // hgt (Height) - a number followed by either cm or in:
    //      If cm, the number must be at least 150 and at most 193.
    //      If in, the number must be at least 59 and at most 76.
    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    // cid (Country ID) - ignored, missing or not.
    let byr = p.byr.parse::<i32>().unwrap();
    if byr < 1920 || byr > 2002 {
        return false;
    }
    let iyr = p.iyr.parse::<i32>().unwrap();
    if iyr < 2010 || iyr > 2020 {
        return false;
    }
    let eyr = p.eyr.parse::<i32>().unwrap();
    if eyr < 2020 || eyr > 2030 {
        return false;
    }
    if p.hgt.ends_with("cm") {
        let hgt_cm = p.hgt.strip_suffix("cm").unwrap().parse::<i32>().unwrap();
        if hgt_cm < 150 || hgt_cm > 193 {
            return false;
        }
    } else if p.hgt.ends_with("in") {
        let hgt_in = p.hgt.strip_suffix("in").unwrap().parse::<i32>().unwrap();
        if hgt_in < 59 || hgt_in > 76 {
            return false;
        }
    } else {
        return false;
    }

    lazy_static! {
        static ref HCL_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        static ref ECL_RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        static ref PID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
    }

    if !HCL_RE.is_match(&p.hcl) {
        return false;
    }
    if !ECL_RE.is_match(&p.ecl) {
        return false;
    }
    if !PID_RE.is_match(&p.pid) {
        return false;
    }
    true
}
