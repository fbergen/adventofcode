use recap::Recap;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r"^mem\[(?P<addr>\d+)\] = (?P<val>\d+)$")]
pub struct Memset {
    addr: u64,
    val: u64,
}

fn main() {
    let input: &str = include_str!("../inputs/day14");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn part1(input_str: &str) -> u64 {
    let mut ormask: u64 = 0;
    let mut andmask: u64 = u64::MAX;

    let mut vals: HashMap<u64, u64> = HashMap::new();

    for line in input_str.lines() {
        if line.starts_with("mask") {
            let mask = line.strip_prefix("mask = ").unwrap();
            ormask = u64::from_str_radix(&mask.replace("X", "0"), 2)
                .ok()
                .unwrap();
            andmask = u64::from_str_radix(&mask.replace("X", "1"), 2)
                .ok()
                .unwrap();
        } else {
            let m: Memset = line.parse().ok().unwrap();
            vals.insert(m.addr, (m.val | ormask) & andmask);
        }
    }
    vals.values().sum()
}

fn part2(input_str: &str) -> u64 {
    let mut ormask: u64 = 0;

    let mut xbits: Vec<u64> = vec![];
    let mut vals: HashMap<u64, u64> = HashMap::new();

    for line in input_str.lines() {
        if line.starts_with("mask") {
            let mask = line.strip_prefix("mask = ").unwrap();
            ormask = u64::from_str_radix(&mask.replace("X", "0"), 2)
                .ok()
                .unwrap();
            xbits = mask
                .chars()
                .rev()
                .enumerate()
                .filter(|(_i, c)| *c == 'X')
                .map(|(i, _c)| i as u64)
                .collect();
        } else {
            let m: Memset = line.parse().ok().unwrap();

            set_bits(m.val, &xbits, m.addr | ormask, 0, &mut vals);
        }
    }
    vals.values().sum()
}

// bloody closure syntax too complex for me :P
fn set_bits(val: u64, xbits: &Vec<u64>, curr: u64, i: usize, vals: &mut HashMap<u64, u64>) {
    if i >= xbits.len() {
        vals.insert(curr, val);
        return;
    }
    let m = 1 << xbits[i];
    set_bits(val, &xbits, curr | m, i + 1, vals);
    set_bits(val, &xbits, curr & !m, i + 1, vals);
}

#[cfg(test)]
mod test {
    use super::*;

    const TESTCASE: &str = "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    const TESTCASE2: &str = "\
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    #[test]
    fn test_day14() {
        assert_eq!(part1(TESTCASE), 165);
        assert_eq!(part2(TESTCASE2), 208);
    }
}
