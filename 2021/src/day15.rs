use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::iter::repeat;

type Grid = Vec<usize>;

fn parse(input_str: &str, nrepeat: usize) -> Grid {
    let g1: Vec<usize> = input_str
        .lines()
        .flat_map(|l| {
            repeat(l.chars())
                .take(nrepeat)
                .enumerate()
                .flat_map(|(i, iter)| iter.map(move |e| (e.to_digit(10).unwrap() as usize + i)))
        })
        .collect();
    repeat(g1)
        .take(nrepeat)
        .enumerate()
        .flat_map(|(i, vec)| vec.iter().map(move |e| e + i).collect::<Vec<usize>>())
        .map(|i| if i > 9 { i - 9 } else { i })
        .collect()
}

pub fn _print(grid: &Grid) {
    let size = (grid.len() as f64).sqrt().round() as usize;
    for j in 0..size {
        for i in 0..size {
            let idx = i + j * size;
            print!("{:>4}", grid[idx]);
            // print!("{:}", grid[idx]);
        }
        println!("");
    }
    println!("");
    println!("");
}

fn get_neighbours(pos: usize, size: usize) -> Box<dyn Iterator<Item = usize>> {
    let (i, j) = ((pos % size) as i32, (pos / size) as i32);
    const NEIGHBOURS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    Box::new(
        NEIGHBOURS
            .iter()
            .map(move |(dx, dy)| (i + dx, j + dy))
            .filter(move |(x, y)| *x >= 0 && *x < size as i32 && *y >= 0 && *y < size as i32)
            .map(move |p| p.0 as usize + p.1 as usize * size),
    )
}

fn shortest_path(grid: &Grid, start: usize) -> usize {
    let size = (grid.len() as f64).sqrt().round() as usize;

    let mut min_risk: Grid = vec![usize::MAX; grid.len()];
    let mut heap = BinaryHeap::new();

    min_risk[start] = 0;
    heap.push((Reverse(0), start));

    while let Some((Reverse(risk), pos)) = heap.pop() {
        if pos == (size * size - 1) {
            return risk;
        }

        for n_pos in get_neighbours(pos, size) {
            let n_risk = risk + grid[n_pos];

            if n_risk < min_risk[n_pos] {
                heap.push((Reverse(n_risk), n_pos));
                min_risk[n_pos] = n_risk;
            }
        }
    }
    0
}

pub fn solve_part_1(input_str: &str) -> usize {
    let grid = parse(input_str, 1);
    shortest_path(&grid, 0)
}
pub fn solve_part_2(input_str: &str) -> usize {
    let grid = parse(input_str, 5);
    shortest_path(&grid, 0)
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
        assert_eq!(res, 315);
        // assert_eq!(res, 314);
    }
}
