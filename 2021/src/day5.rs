use recap::Recap;
use serde::Deserialize;
use std::collections::HashMap;
use std::iter;

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r"^(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)$")]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

fn range(a: usize, b: usize) -> Box<dyn Iterator<Item = usize>> {
    if a < b {
        return Box::new(a..=b);
    }
    if a == b {
        return Box::new(iter::repeat(a));
    }
    Box::new((b..=a).rev())
}

impl Line {
    fn to_vec(&self) -> Vec<(usize, usize)> {
        range(self.x1, self.x2)
            .zip(range(self.y1, self.y2))
            .collect()
    }
}

pub fn solve_part_1(input_str: &str) -> usize {
    let lines: Vec<Line> = input_str
        .lines()
        .map(|l| l.parse::<Line>().unwrap())
        .filter(|Line { x1, y1, x2, y2 }| x1 == x2 || y1 == y2)
        .collect();
    solve(lines)
}

pub fn solve_part_2(input_str: &str) -> usize {
    let lines = input_str
        .lines()
        .map(|l| l.parse::<Line>().unwrap())
        .collect();
    solve(lines)
}

fn solve(lines: Vec<Line>) -> usize {
    let mut points: HashMap<(usize, usize), usize> = HashMap::new();
    lines.iter().for_each(|l| {
        l.to_vec().iter().for_each(|p| {
            *points.entry(*p).or_insert(0) += 1;
        })
    });
    points.values().filter(|v| **v >= 2).count()
}

#[cfg(test)]
mod test {

    const TESTCASE: &str = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test1() {
        let res = super::solve_part_1(TESTCASE);
        assert_eq!(res, 5);
    }

    #[test]
    fn test2() {
        let res = super::solve_part_2(TESTCASE);
        assert_eq!(res, 12);
    }
}
