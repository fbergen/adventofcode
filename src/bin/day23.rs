fn main() {
    let input = "589174263";
    println!("part1: {}", solve(input, true, 100).unwrap());
    println!("part2: {}", solve(input, false, 10_000_000).unwrap());
}

fn solve(input_str: &str, part1: bool, iter: usize) -> Option<String> {
    let cards: Vec<usize> = input_str
        .chars()
        .map(|d| d.to_digit(10).unwrap() as usize)
        .collect();

    let len = if part1 { cards.len() } else { 1_000_000 };
    // card_[num] = next
    let mut card_ll: Vec<usize> = vec![0; len + 1];

    // Build LinkedList
    for i in 0..len {
        let cur = *cards.get(i).unwrap_or(&(i + 1));
        let mut next = *cards.get(i + 1).unwrap_or(&(i + 2));
        if next > len {
            next = cards[0];
        }

        card_ll[cur] = next;
    }

    let mut cur = cards[0];

    for _ in 0..iter {
        let mut to_move: Vec<usize> = vec![];
        to_move.push(card_ll[cur]);
        to_move.push(card_ll[*to_move.last()?]);
        to_move.push(card_ll[*to_move.last()?]);

        let mut dst: usize = 0;
        for j in 1..6 {
            if cur - j == 0 {
                continue;
            }
            let d = (cur as i32 - j as i32).rem_euclid((len + 1) as i32) as usize;
            if !to_move.contains(&d) {
                dst = d;
                break;
            }
        }

        // LinkedList remove
        card_ll[cur] = card_ll[to_move[2]];

        // Insert to_move list
        card_ll[to_move[2]] = card_ll[dst];
        card_ll[dst] = to_move[0];

        // Iterate cur
        cur = card_ll[cur];
    }

    if part1 {
        let mut cur = 1;
        let ret = (1..len).fold("".to_string(), |acc, _| {
            cur = card_ll[cur];
            acc + &cur.to_string()
        });

        Some(ret)
    } else {
        let first = card_ll[1];
        let second = card_ll[first];
        Some((first * second).to_string())
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
    }
}
