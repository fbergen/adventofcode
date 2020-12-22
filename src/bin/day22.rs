use std::collections::{HashSet, VecDeque};

fn main() {
    let input: &str = include_str!("../inputs/day22");
    println!("part1: {}", solve(input, true).unwrap());
    println!("part2: {}", solve(input, false).unwrap());
}

fn is_p1_winner(p1_deck: &mut VecDeque<usize>, p2_deck: &mut VecDeque<usize>) -> bool {
    let mut seen_games: HashSet<(VecDeque<usize>, VecDeque<usize>)> = HashSet::new();
    while !p2_deck.is_empty() && !p1_deck.is_empty() {
        if !seen_games.insert((p1_deck.clone(), p2_deck.clone())) {
            return true;
        }

        let p1_card = p1_deck.pop_front().unwrap();
        let p2_card = p2_deck.pop_front().unwrap();

        // cannot recurse
        if p1_deck.len() < p1_card || p2_deck.len() < p2_card {
            if p1_card > p2_card {
                p1_deck.push_back(p1_card);
                p1_deck.push_back(p2_card);
            } else {
                p2_deck.push_back(p2_card);
                p2_deck.push_back(p1_card);
            }
        } else {
            let mut next_p1_deck = p1_deck.clone();
            next_p1_deck.resize(p1_card, 0);

            let mut next_p2_deck = p2_deck.clone();
            next_p2_deck.resize(p2_card, 0);

            if is_p1_winner(&mut next_p1_deck, &mut next_p2_deck) {
                p1_deck.push_back(p1_card);
                p1_deck.push_back(p2_card);
            } else {
                p2_deck.push_back(p2_card);
                p2_deck.push_back(p1_card);
            }
        }
    }

    return p2_deck.is_empty();
}

fn solve(input_str: &str, part1: bool) -> Option<usize> {
    let mut sect = input_str.split("\n\n");
    let mut p1_deck: VecDeque<usize> = sect
        .next()?
        .lines()
        .skip(1)
        .map(|l| usize::from_str_radix(l, 10).unwrap())
        .collect();
    let mut p2_deck: VecDeque<usize> = sect
        .next()?
        .lines()
        .skip(1)
        .map(|l| usize::from_str_radix(l, 10).unwrap())
        .collect();

    if part1 {
        while !p2_deck.is_empty() && !p1_deck.is_empty() {
            let p1_card = p1_deck.pop_front()?;
            let p2_card = p2_deck.pop_front()?;
            if p1_card > p2_card {
                p1_deck.push_back(p1_card);
                p1_deck.push_back(p2_card);
            } else {
                p2_deck.push_back(p2_card);
                p2_deck.push_back(p1_card);
            }
        }
    } else {
        is_p1_winner(&mut p1_deck, &mut p2_deck);
    }

    Some(
        p1_deck
            .iter()
            .rev()
            .enumerate()
            .map(|(i, x)| (i + 1) * x)
            .sum::<usize>()
            + p2_deck
                .iter()
                .rev()
                .enumerate()
                .map(|(i, x)| (i + 1) * x)
                .sum::<usize>(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    const TESTCASE: &str = "\
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    #[test]
    fn test_day22() {
        assert_eq!(solve(TESTCASE, true).unwrap(), 306);
        assert_eq!(solve(TESTCASE, false).unwrap(), 291);
    }
}
