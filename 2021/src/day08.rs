#[derive(Debug)]
struct Entry {
    signal: Vec<String>,
    output: Vec<String>,
}

fn sorted_str(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_by(|a, b| b.cmp(a));
    String::from_iter(chars)
}

fn parse(input_str: &str) -> Vec<Entry> {
    input_str
        .lines()
        .map(|l| {
            let s = l.split_once(" | ").unwrap();
            Entry {
                signal: s.0.split(" ").map(|x| sorted_str(x)).collect(),
                output: s.1.split(" ").map(|x| sorted_str(x)).collect(),
            }
        })
        .collect()
}

pub fn solve_part_1(input_str: &str) -> usize {
    let entries: Vec<Entry> = parse(input_str);

    entries
        .iter()
        .map(|e| {
            e.output
                .iter()
                .filter(|o| o.len() == 2 || o.len() == 3 || o.len() == 4 || o.len() == 7)
                .count()
        })
        .sum()
}

fn overlap(a: &String, b: &String) -> usize {
    a.chars().filter(|c| b.contains(|x| x == *c)).count()
}
pub fn solve_part_2(input_str: &str) -> usize {
    let entries: Vec<Entry> = parse(input_str);

    entries
        .iter()
        .map(|e| {
            let seven = e.signal.iter().filter(|x| x.len() == 3).next().unwrap();
            let four = e.signal.iter().filter(|x| x.len() == 4).next().unwrap();

            e.output
                .iter()
                .map(|o| {
                    match o.len() {
                        2 => 1,
                        3 => 7,
                        4 => 4,
                        5 => {
                            // 2,3 or 5
                            if overlap(o, seven) == 3 {
                                3
                            } else if overlap(o, four) == 2 {
                                2
                            } else {
                                5
                            }
                        }
                        6 => {
                            // 0, 6 or 9
                            if overlap(o, four) == 4 {
                                9
                            } else if overlap(o, seven) == 3 {
                                0
                            } else {
                                6
                            }
                        }
                        7 => 8,
                        _ => panic!(),
                    }
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
