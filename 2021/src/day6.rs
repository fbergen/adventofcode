use std::collections::VecDeque;

pub fn solve_part_1(input_str: &str) -> usize {
    solve(input_str, 80)
}
pub fn solve_part_2(input_str: &str) -> usize {
    solve(input_str, 256)
}
pub fn solve(input_str: &str, days: usize) -> usize {
    let inp = input_str
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap());

    let mut deq: VecDeque<usize> = VecDeque::from(vec![0; 9 + days]);
    inp.for_each(|x| deq[x] += 1);

    for _d in 0..days {
        let num_spawn = deq.pop_front().unwrap();
        deq[6] += num_spawn;
        deq[8] += num_spawn;
    }
    deq.iter().sum()
}

#[cfg(test)]
mod test {

    const TESTCASE: &str = "\
3,4,3,1,2";

    #[test]
    fn test1() {
        let res = super::solve_part_1(TESTCASE);
        assert_eq!(res, 5934);
    }

    #[test]
    fn test2() {
        let res = super::solve_part_2(TESTCASE);
        assert_eq!(res, 26984457539);
    }
}
