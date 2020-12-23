use std::collections::{HashSet, VecDeque};

fn main() {
    let input = "589174263";
    println!("part1: {}", solve(input, true, 100).unwrap());
    println!("part2: {}", solve(input, false, 10_000_000).unwrap());
}

fn solve(input_str: &str, part1: bool, iter: usize) -> Option<String> {
    let mut cards: Vec<i32> = input_str
        .chars()
        .map(|d| d.to_digit(10).unwrap() as i32)
        .collect();

    if !part1 {
        for i in 10..=1_000_000 {
            cards.push(i);
        }
    }
    let len = cards.len() as i32;

    let mut curri = 0;
    for i in 0..iter {
        if i % 10_000 == 0 {
            println!("it: {}", i)
        }
        let curr: i32 = cards[curri % len as usize];

        // println!("Move {}, curr: {}", i + 1, curr);
        // println!("Cups {:?}", cards);
        let to_move: Vec<i32> = (0..3)
            .map(|idx| {
                let mv = cards.remove((curri + 1) % cards.len());
                if curri >= cards.len() {
                    curri -= 1;
                }
                mv
            })
            .collect();

        // println!("Pick up {:?}", to_move);
        let dst = {
            let mut destination: i32 = -1;
            for j in 1..6 {
                if curr - j == 0 {
                    continue;
                }
                if !to_move.contains(&(curr - j).rem_euclid(len + 1)) {
                    destination = (curr - j).rem_euclid(len + 1);
                    break;
                }
            }

            cards.iter().position(|&x| x == destination).unwrap() + 1
        };

        cards.splice(dst..dst, to_move.iter().cloned());
        if dst <= curri {
            curri += 3;
        }

        curri = (curri + 1) % cards.len();
    }
    //  println!("Final {:?}", cards);
    let one_pos = cards.iter().position(|&x| x == 1).unwrap();
    cards.rotate_left(one_pos);
    if part1 {
        Some(
            cards
                .iter()
                .skip(1)
                .fold("".to_string(), |acc, x| format!("{}{}", acc, x)),
        )
    } else {
        println!("Final {:?}, {:?}", cards[1], cards[2]);
        Some((cards[1] * cards[2]).to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TESTCASE: &str = "389125467";

    #[test]
    fn test_day23() {
        assert_eq!(solve(TESTCASE, true, 10).unwrap(), "92658374");
        assert_eq!(solve(TESTCASE, true, 100).unwrap(), "67384529");
        assert_eq!(solve(TESTCASE, false, 10_000_000).unwrap(), "149245887792");
        // assert_eq!(solve(TESTCASE, false).unwrap(), 291);
    }
}
