fn main() {
    let input: &str = include_str!("../inputs/day1");
    println!("part1: {}", solve(input, true).unwrap());
    println!("part1: {}", solve(input, false).unwrap());
}

fn solve(input_str: &str, part1: bool) -> Option<usize> {
    let input: Vec<usize> = input_str
        .lines()
        .map(|p| p.parse::<usize>().expect(&format!("Parse error {:?}", p)))
        .collect();

    if part1 {
        return Some(count_increasing(&input));
    }

    let iter = input.windows(3);
    let w = iter.map(|x| x.iter().product()).collect();

    return Some(count_increasing(&w));
}

fn count_increasing(input: &Vec<usize>) -> usize {
    let mut prev = input[0];
    let mut count = 0;
    for i in 1..input.len() {
        let v = input[i];
        if v > prev {
            count += 1;
        }
        prev = v;
    }

    count
}
