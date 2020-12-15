fn main() {
    let input: &str = "1,2,16,19,18,0";
    println!("part1: {}", solve(input, 2020));
    println!("part2: {}", solve(input, 30000000));
}

fn solve(input_str: &str, turns: usize) -> usize {
    let starting_nums: Vec<usize> = input_str
        .split(",")
        .map(|p| p.parse::<usize>().unwrap())
        .collect();

    let mut num_to_turn: Vec<usize> = vec![0; turns];

    let mut turn = 1;
    let mut last_spoken = 0;
    for v in starting_nums {
        num_to_turn[v] = turn;
        turn += 1;
        last_spoken = v;
    }
    num_to_turn[last_spoken] = 0;
    let mut to_speak: usize = 0;

    for turn in turn..=turns {
        to_speak = match num_to_turn[last_spoken] {
            0 => 0,
            x => turn - x - 1,
        };
        num_to_turn[last_spoken] = turn - 1;
        // println!("{}: last {}, now {}", turn, last_spoken, to_speak);
        last_spoken = to_speak;
    }
    to_speak
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day15() {
        assert_eq!(solve("0,3,6", true), 436);
        assert_eq!(solve("1,3,2", true), 1);
    }
}
