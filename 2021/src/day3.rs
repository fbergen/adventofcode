pub fn solve_part_1(input_str: &str) -> usize {
    let report: Vec<Vec<usize>> = input_str
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '0' => 0,
                    '1' => 1,
                    _ => panic!(),
                })
                .collect()
        })
        .collect();

    let bit_len = report.first().unwrap().len();
    let num_inputs = report.len();

    // sum of all bits at position x
    let r = report.iter().fold(vec![0; bit_len], |res, x| {
        res.iter().zip(x.iter()).map(|(&a, &b)| a + b).collect()
    });

    let gamma = r.iter().fold(0, |acc, x| {
        if *x > num_inputs / 2 {
            return acc * 2 + 1;
        }
        return acc * 2;
    });
    let epsilon = r.iter().fold(0, |acc, x| {
        if *x < num_inputs / 2 {
            return acc * 2 + 1;
        }
        return acc * 2;
    });

    gamma * epsilon
}

fn get_majority(list: Vec<&Vec<isize>>, inverse: bool) -> isize {
    let bit_len = list.first().unwrap().len();
    let mut oxy: Vec<&Vec<isize>> = list.into_iter().collect();
    for i in 0..bit_len {
        let num_left = oxy.len();

        if num_left == 1 {
            break;
        }
        // majority of all bits at position x
        let maj = oxy.iter().fold(vec![0; bit_len], |res, x| {
            res.iter()
                .zip(x.iter())
                .map(|(&a, &b)| a + b * 2 - 1) // incrememt one for 1 and decrement for 0
                .collect()
        });

        let mut pick = match maj[i] >= 0 {
            true => 1,
            false => 0,
        };
        if inverse {
            pick ^= 1;
        }
        oxy.retain(|x| x[i] == pick);
    }

    oxy.first()
        .unwrap()
        .into_iter()
        .fold(0, |acc, x| acc * 2 + x)
}

pub fn solve_part_2(input_str: &str) -> isize {
    let report: Vec<Vec<isize>> = input_str
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '0' => 0,
                    '1' => 1,
                    _ => panic!(),
                })
                .collect()
        })
        .collect();

    let oxy = get_majority(report.iter().collect(), false);
    let co2 = get_majority(report.iter().collect(), true);

    oxy * co2
}

#[cfg(test)]
mod test {
    #[test]
    fn test1() {
        let res = super::solve_part_1(
            "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010",
        );
        assert_eq!(res, 198);
    }

    #[test]
    fn test2() {
        let res = super::solve_part_2(
            "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010",
        );
        assert_eq!(res, 230);
    }
}
