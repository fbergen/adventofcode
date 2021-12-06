use std::collections::VecDeque;

const DAYS_1: usize = 80;
const DAYS_2: usize = 256;

pub fn solve_part_1(input_str: &str) -> usize {
    let inp = input_str
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap());

    let mut deq = [0; 9 + DAYS_1];
    inp.for_each(|x| deq[x] += 1);

    for d in 0..DAYS_1 {
        let num_spawn = deq[d];
        deq[d + 7] += num_spawn;
        deq[d + 9] += num_spawn;
    }
    ((DAYS_1)..(DAYS_1 + 9)).map(|x| deq[x]).sum()
}
pub fn solve_part_2(input_str: &str) -> usize {
    let inp = input_str
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap());

    let mut deq = [0; 9 + DAYS_2];
    inp.for_each(|x| deq[x] += 1);

    for d in 0..DAYS_2 {
        let num_spawn = deq[d];
        deq[d + 7] += num_spawn;
        deq[d + 9] += num_spawn;
    }
    ((DAYS_2)..(DAYS_2 + 9)).map(|x| deq[x]).sum()
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
