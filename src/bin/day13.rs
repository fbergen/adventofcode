fn main() {
    let input: &str = include_str!("../inputs/day13");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn part1(input_str: &str) -> usize {
    let mut lines = input_str.lines();

    let time: usize = lines.next().unwrap().parse().ok().unwrap();
    let busses: Vec<usize> = lines
        .next()
        .unwrap()
        .split(',')
        .filter(|id| *id != "x")
        .map(|id| id.parse().ok().unwrap())
        .collect();

    let mut first_b = 0;
    let mut lowest_wait = usize::MAX;

    for b in busses {
        let wait = (b - (time % b)) % b;
        if wait < lowest_wait {
            lowest_wait = wait;
            first_b = b;
        }
    }
    lowest_wait * first_b
}

fn bezout(a: i64, b: i64) -> i64 {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);

    while r != 0 {
        let q = old_r / r;

        let mut tmp = old_r;
        old_r = r;
        r = tmp - q * r;

        tmp = old_s;
        old_s = s;
        s = tmp - q * s;
    }
    old_s
}

fn chinese_remainder(vals: Vec<(i64, i64)>) -> i64 {
    let prod: i64 = vals.iter().map(|(_, id)| id).product();

    let mut sum = 0;

    for (a, n) in vals {
        let q = prod / n;
        sum += -a * bezout(q, n) * q
    }

    sum.rem_euclid(prod)
}

fn part2(input_str: &str) -> i64 {
    let mut lines = input_str.lines().skip(1);

    let busses: Vec<(i64, i64)> = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, id)| *id != "x")
        .map(|(i, id)| (i as i64, id.parse::<i64>().ok().unwrap()))
        .collect();

    return chinese_remainder(busses);
}

#[cfg(test)]
mod test {
    use super::*;

    const TESTCASE: &str = "\
939
7,13,x,x,59,x,31,19";
    const TEST2: &str = "\
xxx
17,x,13,19";

    const TEST3: &str = "\
xxx
67,7,59,61";

    #[test]
    fn test_day13() {
        assert_eq!(part1(TESTCASE), 295);
        assert_eq!(part2(TEST2), 3417);
        assert_eq!(part2(TEST3), 754018);
        assert_eq!(part2(TESTCASE), 1068781);
    }
}
