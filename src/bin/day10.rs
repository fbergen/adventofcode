fn main() {
    let input: &str = include_str!("../inputs/day10");
    println!("part 1 {:?}", part1(input).unwrap());
    println!("part 2 {:?}", part2(input).unwrap());
}

fn part1(input_str: &str) -> Option<usize> {
    let mut input: Vec<usize> = input_str
        .lines()
        .map(|p| p.parse::<usize>().expect(&format!("Parse error {:?}", p)))
        .collect();

    input.sort();
    // Add the last element
    input.push(input.last()? + 3);

    let mut diffs = vec![0; 3];
    input.iter().fold(0, |x, y| {
        diffs[y - x - 1] += 1;
        *y
    });
    Some(diffs[0] * diffs[2])
}

fn part2(input_str: &str) -> Option<usize> {
    let mut input: Vec<usize> = input_str
        .lines()
        .map(|p| p.parse::<usize>().expect(&format!("Parse error {:?}", p)))
        .collect();

    // Add a zero element
    input.push(0);
    input.sort();
    // Add the last element
    input.push(input.last()? + 3);

    let mut c: Vec<usize> = vec![0; input.len()];
    // There is 1 way to get to 0 jolts.
    c[0] = 1;
    for i in 1..input.len() {
        let curr = input[i];

        c[i] = if curr <= input[i - 1] + 3 {
            c[i - 1]
        } else {
            0
        } + if i >= 2 && curr <= input[i - 2] + 3 {
            c[i - 2]
        } else {
            0
        } + if i >= 3 && curr <= input[i - 3] + 3 {
            c[i - 3]
        } else {
            0
        };
    }

    Some(c[input.len() - 1])
}

#[cfg(test)]
mod test {
    use super::*;

    const TESTCASE: &str = "\
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn test_day10() {
        assert_eq!(part1(TESTCASE).unwrap(), 220);
        assert_eq!(part2(TESTCASE).unwrap(), 19208);
    }
}
