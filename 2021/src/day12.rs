use itertools::Itertools;
use recap::Recap;
use serde::Deserialize;

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r"^(?P<from>\w+)-(?P<to>\w+)$")]
pub struct Links {
    from: String,
    to: String,
}

type AdjList = Vec<Vec<(bool, usize)>>;
struct Params {
    adj_list: AdjList,
    start_id: usize,
    end_id: usize,
    size: usize,
}

fn parse(input_str: &str) -> Params {
    let is_upper = |s: &String| -> bool { s.to_ascii_uppercase() == *s };

    let str_ids: Vec<String> = input_str
        .lines()
        .map(|l| l.parse::<Links>().unwrap())
        .map(|l| [l.to.to_string(), l.from.to_string()])
        .flatten()
        .unique()
        .collect();
    let get_id =
        |s: &String, str_id: &Vec<String>| -> usize { str_id.iter().position(|x| x == s).unwrap() };

    let mut adj_list: AdjList = vec![vec![]; str_ids.len()];
    let links: Vec<Links> = input_str
        .lines()
        .map(|l| l.parse::<Links>().unwrap())
        .collect();

    links.iter().for_each(|l| {
        adj_list[get_id(&l.from, &str_ids)].push((is_upper(&l.to), get_id(&l.to, &str_ids)));
        adj_list[get_id(&l.to, &str_ids)].push((is_upper(&l.from), get_id(&l.from, &str_ids)));
    });

    Params {
        adj_list: adj_list,
        start_id: get_id(&"start".to_string(), &str_ids),
        end_id: get_id(&"end".to_string(), &str_ids),
        size: str_ids.len(),
    }
}

fn dfs(part1: bool, params: &Params, last: usize, seen: &mut Vec<bool>, has_dupes: bool) -> usize {
    if last == params.end_id {
        return 1;
    }

    let mut num_paths = 0;
    params.adj_list[last].iter().for_each(|(is_upper, n)| {
        if *n != params.start_id {
            let contains_n = seen[*n];
            let next_has_dupes = has_dupes || (!*is_upper && contains_n);

            if *is_upper || !contains_n || (!part1 && !has_dupes) {
                seen[*n] = true;
                num_paths += dfs(part1, params, *n, seen, next_has_dupes);
                if !contains_n {
                    seen[*n] = false;
                }
            }
        }
    });
    num_paths
}
pub fn solve_part_1(input_str: &str) -> usize {
    solve(input_str, true)
}

pub fn solve_part_2(input_str: &str) -> usize {
    solve(input_str, false)
}

fn solve(input_str: &str, part1: bool) -> usize {
    let params = parse(input_str);
    let mut seen = vec![false; params.size];
    seen[params.start_id] = true;_id
    dfs(part1, &params, params.start_id, &mut seen, false)
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
