use hashbrown::HashMap;
use itertools::Itertools;

type InsertionRules = HashMap<(char, char), char>;

fn parse(input_str: &str) -> (String, InsertionRules) {
    let (template, insertions) = input_str.split_once("\n\n").unwrap();
    (
        template.to_string(),
        insertions
            .lines()
            .map(|l| {
                let (from, to) = l.split_once(" -> ").unwrap();
                (
                    (from.chars().nth(0).unwrap(), from.chars().nth(1).unwrap()),
                    to.chars().next().unwrap(),
                )
            })
            .collect(),
    )
}

type Map = HashMap<char, usize>;

fn get_at_depth(
    ins_rules: &InsertionRules,
    a: char,
    b: char,
    depth: usize,
    cache: &mut HashMap<(char, char, usize), Map>,
) -> Map {
    if depth == 0 {
        return HashMap::from_iter([(a, 1)]);
    } else {
        let mut map: Map = HashMap::new();
        let ins = ins_rules[&(a, b)];

        get_at_depth_cache(ins_rules, a, ins, depth - 1, cache)
            .iter()
            .for_each(|(k, v)| {
                map.entry(*k).and_modify(|e| *e += v).or_insert(*v);
            });
        get_at_depth_cache(ins_rules, ins, b, depth - 1, cache)
            .iter()
            .for_each(|(k, v)| {
                map.entry(*k).and_modify(|e| *e += v).or_insert(*v);
            });
        return map;
    }
}

fn get_at_depth_cache(
    ins_rules: &InsertionRules,
    a: char,
    b: char,
    depth: usize,
    cache: &mut HashMap<(char, char, usize), Map>,
) -> Map {
    if let Some(x) = cache.get(&(a, b, depth)) {
        return x.clone();
    }

    let map = get_at_depth(ins_rules, a, b, depth, cache);
    cache.insert((a, b, depth), map.clone());
    return map;
}

pub fn solve(input_str: &str, num: usize) -> usize {
    let (template, ins_rules) = parse(input_str);

    let mut map: Map = HashMap::new();
    let mut cache: HashMap<(char, char, usize), Map> = HashMap::new();

    let chars = template.chars().collect::<Vec<char>>();
    chars.windows(2).for_each(|x| {
        get_at_depth(&ins_rules, x[0], x[1], num, &mut cache)
            .iter()
            .for_each(|(k, v)| {
                map.entry(*k).and_modify(|e| *e += v).or_insert(*v);
            });
    });
    map.entry(*chars.last().unwrap())
        .and_modify(|e| *e += 1)
        .or_insert(1);

    let counts: Vec<(usize, char)> = map.into_iter().map(|(a, b)| (b, a)).sorted().collect();

    counts[counts.len() - 1].0 - counts[0].0
}
pub fn solve_part_1(input_str: &str) -> usize {
    solve(input_str, 10)
}

pub fn solve_part_2(input_str: &str) -> usize {
    solve(input_str, 40)
}

#[cfg(test)]
mod test {

    const TESTCASE: &str = "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn test1() {
        let res = super::solve_part_1(TESTCASE);
        assert_eq!(res, 1588);
    }

    #[test]
    fn test2() {
        let res = super::solve_part_2(TESTCASE);
        assert_eq!(res, 2188189693529);
    }
}
