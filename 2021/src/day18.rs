use itertools::Itertools;
use std::iter;

type Snum = Vec<Option<usize>>;

// Indexes in the array
//            0
//      1           2
//   3     4     5    6
// 7   8  9 10 11 12 13 14
//
fn parse(input_str: &str) -> Snum {
    let mut ret: Snum = vec![];
    let mut idx = 0;
    for chr in input_str.chars() {
        match chr {
            '[' => idx = idx * 2 + 1,
            ',' => idx += 1,
            ']' => idx = idx / 2 - 1,
            x => {
                if idx >= ret.len() {
                    ret.resize(idx + 1, None);
                }
                ret[idx] = Some(x.to_digit(10).unwrap() as usize)
            }
        }
    }
    return ret;
}

fn add(a: &Snum, b: &Snum) -> Snum {
    let mut ret: Snum = Vec::with_capacity(64);
    ret.push(None); // root element is None

    // too much, but hey
    let mut start = 0;
    let none = None;
    for i in 0..5 {
        let take = 1 << i;
        ret.extend(
            a.iter()
                .skip(start)
                .chain(iter::repeat(&none).take(take))
                .take(take),
        );
        ret.extend(
            b.iter()
                .skip(start)
                .chain(iter::repeat(&none).take(take))
                .take(take),
        );
        start = start + take;
    }

    ret
}

fn next_left(mut idx: usize, a: &Snum) -> usize {
    if idx % 2 == 0 {
        idx -= 1;
        while idx <= 32 && a[idx].is_none() {
            idx = idx * 2 + 2;
        }
        return idx;
    } else {
        return (idx - 1) / 2;
    }
}

fn next_right(mut idx: usize, a: &Snum) -> usize {
    if idx <= 32 && a[idx].is_none() {
        return idx * 2 + 1;
    }
    while idx % 2 == 0 {
        if idx == 0 {
            return 0;
        }
        idx = (idx - 1) / 2;
    }
    return idx + 1;
}

fn next(mut idx: usize, a: &Snum, f: &dyn Fn(usize, &Snum) -> usize) -> Option<usize> {
    loop {
        let next = f(idx, a);
        if next == 0 {
            return None;
        }
        idx = next;
        if idx < a.len() && a[idx].is_some() {
            return Some(idx);
        }
    }
}

fn _print(a: &Snum) {
    let mut n = 0;
    while let Some(next_id) = next(n, a, &next_right) {
        println!("PRINT {}, {:?},", next_id, a[next_id]);

        n = next_id;
    }
}

fn magnitude(root: usize, a: &Snum) -> usize {
    if root >= a.len() {
        return 0;
    }
    if a[root].is_some() {
        return a[root].unwrap();
    }
    if root > 62 {
        return a[root].unwrap_or(0);
    }
    return 3 * magnitude(root * 2 + 1, a) + 2 * magnitude(root * 2 + 2, a);
}

// Odd is left children
//
// left children: 1, 3, 7, 15
// right children 2, 4, 6, 16
fn reduce(a: &mut Snum) -> bool {
    let mut iter = (31..a.len()).into_iter();
    while let Some((l, r)) = iter.next_tuple() {
        let mut exploaded = false;
        if a[l].is_none() || a[r].is_none() {
            continue;
        }
        if let Some(left) = next(l, a, &next_left) {
            a[left] = Some(a[left].unwrap() + a[l].unwrap());
            exploaded = true;
        }
        if let Some(right) = next(r, a, &next_right) {
            a[right] = Some(a[right].unwrap() + a[r].unwrap());
            exploaded = true;
        }
        if exploaded {
            a[l] = None;
            a[r] = None;
            a[l / 2] = Some(0);
            return true;
        }
    }
    // check split
    let mut next_id = 0;
    while let Some(right) = next(next_id, a, &next_right) {
        let val = a[right].unwrap();
        if val >= 10 {
            // let val = a[right].unwrap();
            a[right] = None;
            a[right * 2 + 1] = Some(val / 2);
            a[right * 2 + 2] = Some((val + 1) / 2);
            return true;
        }
        next_id = right;
    }
    false
}

pub fn solve_part_1(input_str: &str) -> usize {
    let snums: Vec<Snum> = input_str.lines().map(|l| parse(l)).collect();

    let sum = snums
        .into_iter()
        .reduce(|acc, item| {
            let mut m = add(&acc, &item);
            while reduce(&mut m) {}
            m
        })
        .unwrap();

    magnitude(0, &sum)
}

pub fn solve_part_2(input_str: &str) -> usize {
    let snums: Vec<Snum> = input_str.lines().map(|l| parse(l)).collect();

    let mut max_mag = 0;
    for i in 0..snums.len() {
        for j in 0..snums.len() {
            if i != j {
                let mut m = add(&snums[i], &snums[j]);
                while reduce(&mut m) {}
                let mag = magnitude(0, &m);
                if mag > max_mag {
                    max_mag = mag;
                }
            }
        }
    }
    max_mag
}

#[cfg(test)]
mod test {

    const TESTCASE: &str = "\
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    #[test]
    fn test1() {
        let res = super::solve_part_1(TESTCASE);
        assert_eq!(res, 4140);
    }

    #[test]
    fn test_magnitudes() {
        assert_eq!(super::solve_part_1("[[1,2],[[3,4],5]]"), 143);
        assert_eq!(
            super::solve_part_1("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"),
            1384
        );
        assert_eq!(super::solve_part_1("[[[[1,1],[2,2]],[3,3]],[4,4]]"), 445);
        assert_eq!(super::solve_part_1("[[[[3,0],[5,3]],[4,4]],[5,5]]"), 791);
        assert_eq!(super::solve_part_1("[[[[5,0],[7,4]],[5,5]],[6,6]]"), 1137);
        assert_eq!(
            super::solve_part_1("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"),
            3488
        );
    }

    #[test]
    fn test2() {
        let res = super::solve_part_2(TESTCASE);
        assert_eq!(res, 3993);
    }
}
