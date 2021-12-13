use hashbrown::HashMap;
use itertools::Itertools;
use recap::Recap;
use serde::Deserialize;

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r"^(?P<from>\w+)-(?P<to>\w+)$")]
pub struct Links {
    from: String,
    to: String,
}

type BitArr = u64;
type Cache = HashMap<(BitArr, bool, usize), usize>;
type AdjList = Vec<Vec<(bool, usize)>>;

struct Params {
    part1: bool,
    adj_list: AdjList,
    start_id: usize,
    end_id: usize,
    size: usize,
}

fn parse(input_str: &str) -> Params {
    let is_upper = |s: &String| -> bool { s.to_ascii_uppercase() == *s };
    let get_id =
        |s: &String, str_id: &Vec<String>| -> usize { str_id.iter().position(|x| x == s).unwrap() };

    let str_ids: Vec<String> = input_str
        .lines()
        .map(|l| l.parse::<Links>().unwrap())
        .map(|l| [l.to.to_string(), l.from.to_string()])
        .flatten()
        .unique()
        .collect();
    let mut adj_list: AdjList = vec![vec![]; str_ids.len()];
    input_str
        .lines()
        .map(|l| l.parse::<Links>().unwrap())
        .for_each(|l| {
            let from_id = get_id(&l.from, &str_ids);
            let to_id = get_id(&l.to, &str_ids);
            adj_list[from_id].push((is_upper(&l.to), to_id));
            adj_list[to_id].push((is_upper(&l.from), from_id));
        });

    Params {
        part1: true,
        adj_list: adj_list,
        start_id: get_id(&"start".to_string(), &str_ids),
        end_id: get_id(&"end".to_string(), &str_ids),
        size: str_ids.len(),
    }
}

fn dfs(params: &Params, last: usize, seen: BitArr, has_dupes: bool, cache: &mut Cache) -> usize {
    let cache_key = (seen, has_dupes, last);
    if let Some(num_paths) = cache.get(&cache_key) {
        return *num_paths;
    } else {
        if last == params.end_id {
            return 1;
        }

        let mut num_paths = 0;
        params.adj_list[last].iter().for_each(|(is_upper, n)| {
            if *n != params.start_id {
                let contains_n = seen & (1 << *n) != 0;
                let next_has_dupes = has_dupes || (!*is_upper && contains_n);

                if *is_upper || !contains_n || (!params.part1 && !has_dupes) {
                    num_paths += dfs(params, *n, seen | (1 << *n), next_has_dupes, cache);
                }
            }
        });

        cache.insert(cache_key, num_paths);
        num_paths
    }
}
pub fn solve_part_1(input_str: &str) -> usize {
    solve(input_str, true)
}

pub fn solve_part_2(input_str: &str) -> usize {
    solve(input_str, false)
}

fn solve(input_str: &str, part1: bool) -> usize {
    let mut params = parse(input_str);
    params.part1 = part1;
    let mut seen = vec![false; params.size];
    seen[params.start_id] = true;

    let mut cache: Cache = HashMap::new();
    dfs(&params, params.start_id, 0, false, &mut cache)
}

#[cfg(test)]
mod test {

    const TESTCASE: &str = "\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    #[test]
    fn test1() {
        let res = super::solve_part_1(TESTCASE);
        assert_eq!(res, 19);
    }

    #[test]
    fn test2() {
        let res = super::solve_part_2(TESTCASE);
        assert_eq!(res, 103);
    }
}
