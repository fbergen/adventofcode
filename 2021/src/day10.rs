use itertools::Itertools;
pub fn solve_part_1(input_str: &str) -> usize {
    let progs = input_str.lines().map(|l| l.chars());

    let pushtrue = |exp: &mut Vec<char>, chr: char| {
        exp.push(chr);
        true
    };
    progs
        .filter_map(|mut p| {
            let mut exp = vec![];
            let c = p.find(|chr| {
                let accept = match chr {
                    '(' => pushtrue(&mut exp, '('),
                    '[' => pushtrue(&mut exp, '['),
                    '{' => pushtrue(&mut exp, '{'),
                    '<' => pushtrue(&mut exp, '<'),
                    ')' if exp.pop().unwrap_or('(') == '(' => true,
                    ']' if exp.pop().unwrap_or('[') == '[' => true,
                    '}' if exp.pop().unwrap_or('{') == '{' => true,
                    '>' if exp.pop().unwrap_or('<') == '<' => true,
                    _ => false,
                };
                !accept
            });
            c
        })
        .sorted()
        .dedup_with_count()
        .map(|(count, chr)| {
            let points = match chr {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => panic!(),
            };
            points * count
        })
        .sum()
}
pub fn solve_part_2(input_str: &str) -> usize {
    let progs = input_str.lines().map(|l| l.chars());

    let pushtrue = |exp: &mut Vec<char>, chr: char| {
        exp.push(chr);
        true
    };
    let scores: Vec<usize> = progs
        .filter_map(|mut p| {
            let mut exp = vec![];
            let c = p.find(|chr| {
                let accept = match chr {
                    '(' => pushtrue(&mut exp, '('),
                    '[' => pushtrue(&mut exp, '['),
                    '{' => pushtrue(&mut exp, '{'),
                    '<' => pushtrue(&mut exp, '<'),
                    ')' if exp.pop().unwrap_or('(') == '(' => true,
                    ']' if exp.pop().unwrap_or('[') == '[' => true,
                    '}' if exp.pop().unwrap_or('{') == '{' => true,
                    '>' if exp.pop().unwrap_or('<') == '<' => true,
                    _ => false,
                };
                !accept
            });
            match c {
                None => Some(exp),
                _ => None,
            }
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
