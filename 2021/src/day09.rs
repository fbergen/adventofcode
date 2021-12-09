use itertools::Itertools;
use std::collections::HashSet;

type Board = Vec<Vec<u32>>;

fn try_get(board: &Board, x: i32, y: i32) -> Option<&u32> {
    if x < 0 || y < 0 {
        ()
    }
    board.get(x as usize)?.get(y as usize)
}

fn get_neighbours(board: &Board, x: i32, y: i32) -> Vec<((i32, i32), &u32)> {
    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
        .into_iter()
        .map(|(xx, yy)| ((xx, yy), try_get(&board, xx, yy)))
        .filter(|(_point, val)| val.is_some())
        .map(|(point, val)| (point, val.unwrap()))
        .collect()
}

fn get_low_points(board: &Board) -> Vec<(usize, usize)> {
    (0..board.len())
        .map(|i| (0..board[0].len()).map(move |j| (i, j)))
        .flatten()
        .filter(|(i, j)| {
            get_neighbours(board, *i as i32, *j as i32)
                .iter()
                .all(|(_point, val)| **val > board[*i][*j])
        })
        .collect()
}

fn get_basin_size(board: &Board, x: usize, y: usize) -> usize {
    let mut seen = HashSet::from([(x as i32, y as i32)]);
    let mut queue = vec![(x, y)];

    while queue.len() > 0 {
        let curr = queue.pop().unwrap();
        let curr_v = board[x][y];

        get_neighbours(board, curr.0 as i32, curr.1 as i32)
            .into_iter()
            .filter(|(_p, v)| **v > curr_v && **v < 9)
            .for_each(|(p, _)| {
                if !seen.contains(&p) {
                    queue.push((p.0 as usize, p.1 as usize));
                    seen.insert(p);
                }
            });
    }
    seen.len()
}

pub fn solve_part_1(input_str: &str) -> u32 {
    let board: Board = input_str
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    get_low_points(&board)
        .into_iter()
        .map(|(i, j)| board[i][j] + 1)
        .sum()
}
pub fn solve_part_2(input_str: &str) -> usize {
    let board: Board = input_str
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    get_low_points(&board)
        .into_iter()
        .map(|(x, y)| get_basin_size(&board, x, y))
        .sorted()
        .rev()
        .take(3)
        .product()
}

#[cfg(test)]
mod test {

    const TESTCASE: &str = "\
2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test1() {
        let res = super::solve_part_1(TESTCASE);
        assert_eq!(res, 15);
    }

    #[test]
    fn test2() {
        let res = super::solve_part_2(TESTCASE);
        assert_eq!(res, 1134);
    }
}
