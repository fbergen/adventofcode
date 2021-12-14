use hashbrown::HashMap;
use itertools::Itertools;

type InsertionRules = HashMap<(usize, usize), usize>;

fn parse(input_str: &str) -> (Vec<usize>, InsertionRules, usize) {
    let (template, insertions) = input_str.split_once("\n\n").unwrap();
    let rules: HashMap<(char, char), char> = insertions
        .lines()
        .map(|l| {
            let (from, to) = l.split_once(" -> ").unwrap();
            (
                (from.chars().nth(0).unwrap(), from.chars().nth(1).unwrap()),
                to.chars().next().unwrap(),
            )
        })
        .collect();

    let chr_idx: Vec<&char> = rules.iter().map(|(_k, v)| v).unique().collect();
    let chr_to_idx =
        |c: char, chr_idx: &Vec<&char>| -> usize { chr_idx.iter().position(|&&x| x == c).unwrap() };

    (
        template.chars().map(|c| chr_to_idx(c, &chr_idx)).collect(),
        rules
            .iter()
            .map(|((k1, k2), v)| {
                (
                    (chr_to_idx(*k1, &chr_idx), chr_to_idx(*k2, &chr_idx)),
                    chr_to_idx(*v, &chr_idx),
                )
            })
            .collect(),
        chr_idx.len(),
    )
}

type Map = Vec<usize>;

fn get_at_depth(
    ins_rules: &InsertionRules,
    a: usize,
    b: usize,
    depth: usize,
    cache: &mut HashMap<(usize, usize, usize), Map>,
    size: usize,
) -> Map {
    if depth == 0 {
        let mut e = vec![0; size];
        e[a] = 1;
        return e;
    } else {
        let ins = ins_rules[&(a, b)];

        let mut map: Map = get_at_depth_cache(ins_rules, a, ins, depth - 1, cache, size);
        get_at_depth_cache(ins_rules, ins, b, depth - 1, cache, size)
            .into_iter()
            .enumerate()
            .for_each(|(k, v)| {
                map[k] += v;
            });
        return map;
    }
}

fn get_at_depth_cache(
    ins_rules: &InsertionRules,
    a: usize,
    b: usize,
    depth: usize,
    cache: &mut HashMap<(usize, usize, usize), Map>,
    size: usize,
) -> Map {
    if let Some(x) = cache.get(&(a, b, depth)) {
        return x.clone();
    }

    let map = get_at_depth(ins_rules, a, b, depth, cache, size);
    cache.insert((a, b, depth), map.clone());
    return map;
}

pub fn solve(input_str: &str, num: usize) -> usize {
    let (template, ins_rules, size) = parse(input_str);

    let mut map: Map = vec![0; size];
    let mut cache: HashMap<(usize, usize, usize), Map> = HashMap::new();

    template.windows(2).for_each(|x| {
        get_at_depth(&ins_rules, x[0], x[1], num, &mut cache, size)
            .iter()
            .enumerate()
            .for_each(|(k, v)| {
                map[k] += v;
            });
    });
    map[template[template.len() - 1]] += 1;

    map.iter().max().unwrap() - map.iter().min().unwrap()
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
