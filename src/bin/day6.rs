use itertools::Itertools;
use std::collections::HashMap;

pub fn main() {
    let part1: usize = include_str!("../inputs/day6")
        .split("\n\n")
        .map(|s| s.chars().filter(|c| *c != '\n').unique().count())
        .sum();

    let part2: usize = include_str!("../inputs/day6")
        .split("\n\n")
        .map(|s| {
            let group_size = s.lines().count();
            let mut sc = s.chars().filter(|c| *c != '\n').collect::<Vec<char>>();
            sc.sort();
            sc.iter()
                .group_by(|c| **c)
                .into_iter()
                .map(|(_c, group)| group.count())
                .filter(|group| *group == group_size)
                .count()
        })
        .sum();
    println!("with itertools\tpart 1: {:?}\tpart 2: {:?}", part1, part2);

    let part1_no_itertool: usize = include_str!("../inputs/day6")
        .split("\n\n")
        .map(|s| count_answers(s))
        .map(|(_group_size, mut counter)| counter.drain().filter(|(_k, v)| *v >= 1).count())
        .sum();
    let part2_no_itertool: usize = include_str!("../inputs/day6")
        .split("\n\n")
        .map(|s| count_answers(s))
        .map(|(group_size, mut counter)| counter.drain().filter(|(_k, v)| *v == group_size).count())
        .sum();

    println!(
        "No itertools\tpart 1: {:?}\tpart 2: {:?}",
        part1_no_itertool, part2_no_itertool
    );
}

fn count_answers(group: &str) -> (usize, HashMap<char, usize>) {
    let group_size = group.lines().count();
    let mut m = HashMap::new();
    group
        .chars()
        .filter(|c| *c != '\n')
        .for_each(|ch| *m.entry(ch).or_insert(0) += 1);
    (group_size, m)
}
