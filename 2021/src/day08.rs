#[derive(Debug)]
struct Entry {
    signal: Vec<String>,
    output: Vec<String>,
}

fn parse(input_str: &str) -> Box<dyn Iterator<Item = Entry> + '_> {
    Box::new(input_str.lines().map(|l| {
        let s = l.split_once(" | ").unwrap();
        Entry {
            signal: s.0.split(" ").map(|x| x.to_string()).collect(),
            output: s.1.split(" ").map(|x| x.to_string()).collect(),
        }
    }))
}

pub fn solve_part_1(input_str: &str) -> usize {
    parse(input_str)
        .map(|e| {
            e.output
                .iter()
                .filter(|o| [2, 3, 4, 7].contains(&o.len()))
                .count()
        })
        .sum()
}

pub fn solve_part_2(input_str: &str) -> usize {
    let overlap = |a: &String, b: &String| a.chars().filter(|c| b.contains(*c)).count();
    parse(input_str)
        .map(|e| {
            let seven = e.signal.iter().find(|x| x.len() == 3).unwrap();
            let four = e.signal.iter().find(|x| x.len() == 4).unwrap();
            e.output
                .iter()
                .map(|o| match o.len() {
                    2 => 1,
                    3 => 7,
                    4 => 4,
                    5 if overlap(o, seven) == 3 => 3,
                    5 if overlap(o, four) == 2 => 2,
                    5 => 5,
                    6 if overlap(o, four) == 4 => 9,
                    6 if overlap(o, seven) == 3 => 0,
                    6 => 6,
                    7 => 8,
                    _ => panic!(),
                })
                .fold(0, |acc, x| acc * 10 + x)
        })
        .sum()
}

#[cfg(test)]
mod test {

    const TESTCASE: &str = "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test1() {
        let res = super::solve_part_1(TESTCASE);
        assert_eq!(res, 26);
    }

    #[test]
    fn test2() {
        let res = super::solve_part_2(TESTCASE);
        assert_eq!(res, 61229);
    }
}
