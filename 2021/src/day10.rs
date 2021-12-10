use itertools::Itertools;

fn match_seq(s: &str) -> (Vec<char>, Option<char>) {
    let mut exp = Vec::with_capacity(s.len());

    let c = s.chars().find(|chr| {
        let accept = match chr {
            '(' | '[' | '{' | '<' => {
                exp.push(*chr);
                true
            }
            ')' => exp.pop() == Some('('),
            ']' => exp.pop() == Some('['),
            '}' => exp.pop() == Some('{'),
            '>' => exp.pop() == Some('<'),
            _ => false,
        };
        !accept
    });
    (exp, c)
}

pub fn solve_part_1(input_str: &str) -> usize {
    input_str
        .lines()
        .filter_map(|p| match_seq(p).1)
        .map(|chr| match chr {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!(),
        })
        .sum()
}
pub fn solve_part_2(input_str: &str) -> usize {
    let scores: Vec<usize> = input_str
        .lines()
        .filter_map(|p| match match_seq(p) {
            (x, None) => Some(x),
            _ => None,
        })
        .map(|exp| {
            exp.iter().rev().fold(0, |acc, chr| {
                let points = match chr {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => panic!(),
                };
                acc * 5 + points
            })
        })
        .sorted()
        .collect();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod test {

    const TESTCASE: &str = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test1() {
        let res = super::solve_part_1(TESTCASE);
        assert_eq!(res, 26397);
    }

    #[test]
    fn test2() {
        let res = super::solve_part_2(TESTCASE);
        assert_eq!(res, 288957);
    }
}
