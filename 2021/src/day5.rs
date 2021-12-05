use recap::Recap;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r"^(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)$")]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

fn range(a: usize, b: usize) -> Box<dyn Iterator<Item = usize>> {
    if a <= b {
        return Box::new(a..=b);
    }
    Box::new((b..=a).rev())
}

impl Line {
    fn to_vec(&self, diag: bool) -> Vec<(usize, usize)> {
        if self.x1 == self.x2 {
            return range(self.y1, self.y2).map(|y| (self.x1, y)).collect();
        } else if self.y1 == self.y2 {
            return range(self.x1, self.x2).map(|x| (x, self.y1)).collect();
        }
        if diag {
            return range(self.x1, self.x2)
                .zip(range(self.y1, self.y2))
                .collect();
        }

        vec![]
    }
}

pub fn solve_part_1(input_str: &str) -> usize {
    solve(input_str, false)
}

pub fn solve_part_2(input_str: &str) -> usize {
    solve(input_str, true)
}

fn solve(input_str: &str, part2: bool) -> usize {
    let lines = input_str.lines().map(|l| l.parse::<Line>().unwrap());

    let mut points: HashMap<(usize, usize), usize> = HashMap::new();

    lines.for_each(|l| {
        l.to_vec(part2).iter().for_each(|p| {
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
