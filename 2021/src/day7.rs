pub fn solve_part_1(input_str: &str) -> isize {
    let mut pos: Vec<isize> = input_str
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse::<isize>().unwrap())
        .collect();

    pos.sort();
    let median: isize = pos[(pos.len() / 2)];
    pos.iter().map(|x| (x - median).abs()).sum::<isize>()
}
pub fn solve_part_2(input_str: &str) -> isize {
    let pos: Vec<isize> = input_str
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse::<isize>().unwrap())
        .collect();

    let mean = (pos.iter().sum::<isize>() as f32 / pos.len() as f32).round() as isize;
    pos.iter()
        .map(|x| {
            let s = (x - mean).abs();
            (s * (s + 1) / 2).abs()
        })
        .sum::<isize>()
}

#[cfg(test)]
mod test {

    const TESTCASE: &str = "\
16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test1() {
        let res = super::solve_part_1(TESTCASE);
        assert_eq!(res, 37);
    }

    #[test]
    fn test2() {
        let res = super::solve_part_2(TESTCASE);
        assert_eq!(res, 168);
    }
}
