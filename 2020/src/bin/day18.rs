fn main() {
    let input: &str = include_str!("../inputs/day18");
    println!("part1: {}", solve(input, true).unwrap());
    println!("part2: {}", solve(input, false).unwrap());
}

#[derive(PartialEq)]
enum Op {
    ADD,
    MUL,
}

fn solve(input_str: &str, part1: bool) -> Option<u64> {
    let r = input_str
        .lines()
        .map(|l| {
            let s: Vec<char> = l.chars().filter(|c| !c.is_whitespace()).collect();
            parse_p(part1, &s, &mut 0)
        })
        .sum::<u64>();

    Some(r)
}

// Eval one parenthesis
fn parse_p(part1: bool, s: &Vec<char>, mut i: &mut usize) -> u64 {
    let mut nums: Vec<u64> = vec![];
    let mut ops: Vec<Op> = vec![];

    nums.push(match s[*i] {
        '0'..='9' => s[*i].to_digit(10).unwrap() as u64,
        '(' => {
            *i += 1;
            parse_p(part1, &s, &mut i)
        }
        _ => panic!(""),
    });

    *i += 1;
    loop {
        ops.push(match s[*i] {
            '*' => Op::MUL,
            '+' => Op::ADD,
            _ => panic!("{}, {}", *i, s[*i]),
        });
        *i += 1;
        nums.push(match s[*i] {
            '0'..='9' => s[*i].to_digit(10).unwrap() as u64,
            '(' => {
                *i += 1;
                parse_p(part1, &s, &mut i)
            }
            _ => panic!(""),
        });
        *i += 1;
        if *i >= s.len() || s[*i] == ')' {
            break;
        }
    }

    if part1 {
        // Eval left to right.
        let mut ret = nums[0];
        for io in 0..ops.len() {
            ret = match ops[io] {
                Op::ADD => ret + nums[io + 1],
                Op::MUL => ret * nums[io + 1],
            };
        }
        return ret;
    } else {
        // Eval all ADDs, then return .product()
        for io in (0..ops.len()).rev() {
            if ops[io] == Op::ADD {
                nums[io] = nums[io] + nums[io + 1];
                nums.remove(io + 1);
            }
        }
        return nums.iter().product();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TESTCASE: &str = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

    #[test]
    fn test_day18() {
        assert_eq!(solve(TESTCASE, true).unwrap(), 13632);
        assert_eq!(
            solve("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", false).unwrap(),
            669060
        );
    }
}
