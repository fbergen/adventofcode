use itertools::iproduct;

fn parse(input: &str) -> (usize, usize) {
    let mut lines = input.lines();
    (
        lines
            .next()
            .unwrap()
            .rsplit_once(" ")
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap(),
        lines
            .next()
            .unwrap()
            .rsplit_once(" ")
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap(),
    )
}

pub fn solve_part_1(input_str: &str) -> usize {
    let (mut p1_pos, mut p2_pos) = parse(input_str);
    let (mut p1_score, mut p2_score) = (0, 0);

    p1_pos -= 1;
    p2_pos -= 1;

    let mut die = 1;
    loop {
        let roll = die * 3 + 3;
        die += 3;
        p1_pos += roll;
        p1_score += (p1_pos % 10) + 1;
        if p1_score >= 1000 {
            return p2_score * (die - 1);
        }

        let roll = die * 3 + 3;
        die += 3;
        p2_pos += roll;
        p2_score += (p2_pos % 10) + 1;
        if p2_score >= 1000 {
            return p1_score * (die - 1);
        }
    }
}

pub fn play_rec(
    p1_pos: usize,
    p1_score: usize,
    p2_pos: usize,
    p2_score: usize,
    universes: usize,
    depth: usize,
    freq: &Vec<usize>,
) -> (usize, usize) {
    if p1_score >= 21 {
        return (universes, 0);
    } else if p2_score >= 21 {
        return (0, universes);
    }

    let mut sum = (0, 0);
    for (idx, f) in freq.iter().enumerate() {
        let step = idx + 3;
        let wins: (usize, usize);
        if depth % 2 == 0 {
            wins = play_rec(
                p1_pos + step,
                p1_score + ((p1_pos + step) % 10) + 1,
                p2_pos,
                p2_score,
                universes * f,
                depth + 1,
                &freq,
            );
        } else {
            wins = play_rec(
                p1_pos,
                p1_score,
                p2_pos + step,
                p2_score + ((p2_pos + step) % 10) + 1,
                universes * f,
                depth + 1,
                &freq,
            );
        }
        sum.0 += wins.0;
        sum.1 += wins.1;
    }
    sum
}

pub fn solve_part_2(input_str: &str) -> usize {
    let (mut p1_pos, mut p2_pos) = parse(input_str);
    let (p1_score, p2_score) = (0, 0);

    p1_pos -= 1;
    p2_pos -= 1;

    let mut freq = vec![0; 7];
    iproduct!(1..=3, 1..=3, 1..=3).for_each(|x| freq[x.0 + x.1 + x.2 - 3] += 1);

    let wins = play_rec(p1_pos, p1_score, p2_pos, p2_score, 1, 0, &freq);
    if wins.0 > wins.1 {
        wins.0
    } else {
        wins.1
    }
}

#[cfg(test)]
mod test {

    const TESTCASE: &str = "\
Player 1 starting position: 4
Player 2 starting position: 8";

    #[test]
    fn test1() {
        let res = super::solve_part_1(TESTCASE);
        assert_eq!(res, 739785);
    }

    #[test]
    fn test2() {
        let res = super::solve_part_2(TESTCASE);
        assert_eq!(res, 444356092776315);
    }
}
