use recap::Recap;
use serde::Deserialize;

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r"^(?P<op>\w)(?P<arg>\d+)$")]
pub struct Instr {
    op: char,
    arg: isize,
}

fn main() {
    let input: &str = include_str!("../inputs/day12");
    println!("part1: {}", solve(input, true));
    println!("part2: {}", solve(input, false));
}

fn solve(input_str: &str, part1: bool) -> isize {
    let instrs: Vec<Instr> = input_str
        .lines()
        .map(|p| p.parse::<Instr>().unwrap())
        .collect();

    println!("{:?}", instrs);
    let mut dir = if part1 { (1, 0) } else { (10, 1) };
    let mut pos = (0, 0);

    for i in instrs {
        if part1 {
            match i.op {
                'N' => pos.1 += i.arg,
                'S' => pos.1 -= i.arg,
                'E' => pos.0 += i.arg,
                'W' => pos.0 -= i.arg,
                'R' => dir = rot(dir, -i.arg),
                'L' => dir = rot(dir, -i.arg),
                'F' => {
                    pos.0 += dir.0 * i.arg;
                    pos.1 += dir.1 * i.arg
                }
                _ => panic!("Not a valid input"),
            }
        } else {
            match i.op {
                'N' => dir.1 += i.arg,
                'S' => dir.1 -= i.arg,
                'E' => dir.0 += i.arg,
                'W' => dir.0 -= i.arg,
                'R' => dir = rot(dir, -i.arg),
                'L' => dir = rot(dir, i.arg),
                'F' => {
                    pos.0 += dir.0 * i.arg;
                    pos.1 += dir.1 * i.arg
                }
                _ => panic!("Not a valid input"),
            }
        }
    }
    pos.0.abs() + pos.1.abs()
}

fn rot((x, y): (isize, isize), deg: isize) -> (isize, isize) {
    let rot = [
        [
            [0, -1], // 90deg
            [1, 0],
        ],
        [
            [-1, 0], // 180deg
            [0, -1],
        ],
        [
            [0, 1], // 270deg
            [-1, 0],
        ],
    ];
    let r = ((deg + 360) / 90) as usize % 4;
    if r == 0 {
        // 0 or 360 degree is Noop
        return (x, y);
    }
    let m = rot[r - 1];

    return (x * m[0][0] + y * m[0][1], x * m[1][0] + y * m[1][1]);
}

#[cfg(test)]
mod test {
    use super::*;

    const TESTCASE: &str = "\
F10
N3
F7
R90
F11";

    #[test]
    fn test_day12() {
        assert_eq!(solve(TESTCASE, true), 25);
        assert_eq!(solve(TESTCASE, false), 286);

        assert_eq!(rot((10, 4), -90), (4, -10));
    }
}
