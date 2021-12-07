use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Board {
    rows: Vec<Vec<usize>>,
    cols: Vec<Vec<usize>>,
}

pub fn solve_part_1(input_str: &str) -> usize {
    solve(input_str, true)
}

pub fn solve_part_2(input_str: &str) -> usize {
    solve(input_str, false)
}

fn solve(input_str: &str, part_1: bool) -> usize {
    let mut sections = input_str.split("\n\n");

    let draw_sequence: Vec<usize> = sections
        .next()
        .unwrap()
        .split(",")
        .map(|l| l.parse::<usize>().unwrap())
        .collect();

    let board_vecs: Vec<Vec<Vec<usize>>> = sections
        .map(|x| {
            x.lines()
                .map(|r| {
                    r.split_whitespace()
                        .map(|e| e.parse::<usize>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    let mut boards: Vec<Board> = board_vecs
        .into_iter()
        .map(|b| Board {
            cols: (0..b.len())
                .map(|i| (0..b[i].len()).map(|j| b[j][i]).collect())
                .collect(),
            rows: b,
        })
        .collect();

    for d in draw_sequence {
        boards.iter_mut().for_each(|b| {
            b.rows.iter_mut().for_each(|r| r.retain(|&x| x != d));
            b.cols.iter_mut().for_each(|r| r.retain(|&x| x != d))
        });

        // Winning?
        for bi in (0..boards.len()).rev() {
            let b = &boards[bi];
            if winning(b) {
                if part_1 {
                    return b.rows.iter().flatten().sum::<usize>() * d;
                } else {
                    if boards.len() == 1 {
                        // last board to win
                        return b.rows.iter().flatten().sum::<usize>() * d;
                    }
                    boards.remove(bi);
                }
            }
        }
    }

    panic!();
}

// winning if any row or col is empty.
fn winning(b: &Board) -> bool {
    b.rows.iter().any(|r| r.len() == 0) || b.cols.iter().any(|c| c.len() == 0)
}

#[cfg(test)]
mod test {
    #[test]
    fn test1() {
        let res = super::solve_part_1(
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7 ",
        );
        assert_eq!(res, 4512);
    }

    #[test]
    fn test2() {
        let res = super::solve_part_2(
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7 ",
        );
        assert_eq!(res, 1924);
    }
}
