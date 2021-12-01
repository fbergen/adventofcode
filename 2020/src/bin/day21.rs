use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input: &str = include_str!("../inputs/day21");
    println!("part1: {}", part1(input).unwrap());
    println!("part2: {}", part2(input).unwrap());
}

fn get_dishes(input_str: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    let dishes: Vec<(Vec<&str>, Vec<&str>)> = input_str
        .lines()
        .map(|l| {
            let mut segments = l.split(" (contains ");
            (
                segments.next().unwrap().split(" ").collect(),
                segments
                    .next()
                    .unwrap()
                    .strip_suffix(")")
                    .unwrap()
                    .split(",")
                    .map(|s| s.trim())
                    .collect(),
            )
        })
        .collect();
    dishes
}

fn get_map(dishes: &Vec<(Vec<&str>, Vec<&str>)>) -> HashMap<String, HashSet<String>> {
    let mut m: HashMap<String, HashSet<String>> = HashMap::new();
    dishes.iter().for_each(|(is, allergenes)| {
        let set: HashSet<String> = is.iter().map(|i| i.to_string()).collect();
        allergenes.iter().for_each(|a| {
            let intersection = m
                .entry(a.to_string())
                .or_insert(set.clone())
                .intersection(&set)
                .map(|x| x.to_string())
                .collect();
            m.insert(a.to_string(), intersection);
        })
    });
    m
}

fn part1(input_str: &str) -> Option<usize> {
    let dishes = get_dishes(input_str);
    let m = get_map(&dishes);

    let mut can_contain: HashSet<String> = HashSet::new();
    m.values().for_each(|is| {
        is.iter().for_each(|i| {
            can_contain.insert(i.to_string());
        })
    });

    let ret: usize = dishes
        .iter()
        .map(|(is, _)| {
            is.iter()
                .filter(|i| !can_contain.contains(&i.to_string()))
                .count()
        })
        .sum();

    Some(ret)
}

fn part2(input_str: &str) -> Option<String> {
    let dishes = get_dishes(input_str);
    let mut m = get_map(&dishes);

    let mut allergenes: Vec<(String, String)> = vec![];
    for _ in 0..m.len() {
        let e = m.iter().find(|(_, y)| y.len() == 1).unwrap();
        let i = e.1.iter().next().unwrap().to_string();
        allergenes.push((e.0.to_string(), i.clone()));

        m.iter_mut().for_each(|y| {
            y.1.remove(&i);
        });
    }

    allergenes.sort();
    let ret = allergenes
        .iter()
        .map(|x| x.1.clone())
        .collect::<Vec<String>>()
        .join(",")
        .to_string();

    Some(ret)
}

#[cfg(test)]
mod test {
    use super::*;

    const TESTCASE: &str = "\
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    fn test_day21() {
        assert_eq!(part1(TESTCASE).unwrap(), 5);
        assert_eq!(part2(TESTCASE).unwrap(), "mxmxvkd,sqjhc,fvjkl");
    }
}
