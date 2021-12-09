use itertools::Itertools;

type Board = Vec<Vec<u32>>;

fn try_get(board: &Board, x: i32, y: i32) -> Option<&u32> {
    if x < 0 || y < 0 {
        return None;
    }
    board.get(x as usize)?.get(y as usize)
}

fn get_neighbours(
    board: &Board,
    x: i32,
    y: i32,
) -> Box<dyn Iterator<Item = ((i32, i32), &u32)> + '_> {
    const NEIGHBOURS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    Box::new(
        NEIGHBOURS
            .iter()
            .map(move |(dx, dy)| ((x + dx, y + dy), try_get(&board, x + dx, y + dy)))
            .filter(|(_point, val)| val.is_some())
            .map(|(point, val)| (point, val.unwrap())),
    )
}

fn get_low_points(board: &Board) -> Vec<(usize, usize)> {
    (0..board.len())
        .map(|i| (0..board[0].len()).map(move |j| (i, j)))
        .flatten()
        .filter(|(i, j)| {
            let curr = board[*i][*j];
            get_neighbours(board, *i as i32, *j as i32).all(|(_point, val)| *val > curr)
        })
        .collect()
}

fn get_basin_size(board: &Board, x: usize, y: usize) -> usize {
    let insert = |seen: &mut [bool; 100 * 100], x: usize, y: usize| -> bool {
        let idx = x + y * 100;
        let s = !seen[idx];
        seen[idx] = true;
        s
    };
    let mut seen = [false; 100 * 100];
    insert(&mut seen, x, y);
    let mut queue = vec![(x, y)];
    queue.reserve(1000);

    let mut cnt = 1;
    while queue.len() > 0 {
        let curr = queue.pop().unwrap();
        let curr_v = board[x][y];

        queue.extend(
            get_neighbours(board, curr.0 as i32, curr.1 as i32)
                .filter(|(p, v)| {
                    **v > curr_v && **v < 9 && insert(&mut seen, p.0 as usize, p.1 as usize)
                })
                .map(|(p, _)| {
                    cnt += 1;
                    (p.0 as usize, p.1 as usize)
                }),
        );
    }
    cnt
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
