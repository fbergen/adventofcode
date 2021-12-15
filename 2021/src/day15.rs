type Grid = Vec<usize>;
use std::cmp::min;

fn parse(input_str: &str) -> Grid {
    input_str
        .lines()
        .flat_map(|l| l.chars())
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

pub fn print(grid: &Grid) {
    let size = (grid.len() as f64).sqrt().round() as usize;
    for j in 0..size {
        for i in 0..size {
            let idx = i + j * size;
            print!("{:>4}", grid[idx]);
        }
        println!("");
    }
    println!("");
    println!("");
}

pub fn solve_part_1(input_str: &str) -> usize {
    let mut grid = parse(input_str);
    let size = (grid.len() as f64).sqrt().round() as usize;

    let start = grid[0];
    print(&grid);
    for i in (0..size - 1).rev() {
        let idx = i + size * (size - 1);
        grid[idx] += grid[idx + 1];
    }
    for j in (0..size - 1).rev() {
        let idx = size - 1 + size * j;
        grid[idx] += grid[idx + size];
    }
    print(&grid);
    for i in (0..size - 1).rev() {
        for j in (0..size - 1).rev() {
            let idx = i + j * size;
            grid[idx] += min(grid[idx + 1], grid[idx + size]);
        }
    }
    grid[0] - start;

    print(&grid);
    0
}
pub fn solve_part_2(_input_str: &str) -> usize {
    0
}

#[cfg(test)]
mod test {

    const TESTCASE: &str = "\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn test1() {
        let res = super::solve_part_1(TESTCASE);
        assert_eq!(res, 40);
    }

    #[test]
    fn test2() {
        let res = super::solve_part_2(TESTCASE);
        assert_eq!(res, 0);
    }
}
