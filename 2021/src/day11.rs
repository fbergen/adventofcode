type Grid = Vec<u32>;
const SIZE: usize = 10;

fn get_neighbours<'a>(p: &'a (i32, i32)) -> Box<dyn Iterator<Item = (i32, i32)> + 'a> {
    const NEIGHBOURS: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    Box::new(
        NEIGHBOURS
            .iter()
            .map(move |(dx, dy)| (p.0 + dx, p.1 + dy))
            .filter(|(x, y)| *x >= 0 && *x < SIZE as i32 && *y >= 0 && *y < SIZE as i32),
    )
}

fn update_neighbours(grid: &mut Grid, p: &(i32, i32), flashed: &mut Vec<bool>) -> bool {
    let idx = p.0 as usize + p.1 as usize * SIZE;
    if grid[idx] > 9 && !flashed[idx] {
        flashed[idx] = true;
        get_neighbours(&p).for_each(|p| {
            let idx = p.0 as usize + p.1 as usize * SIZE;
            grid[idx] += 1;
        });
        return true;
    }
    false
}

fn iter(grid: &mut Grid) -> usize {
    grid.iter_mut().for_each(|i| *i += 1);

    let mut flashed: Vec<bool> = vec![false; SIZE * SIZE];
    let mut updated = true;
    let mut count = 0;
    while updated {
        updated = false;

        for x in 0..SIZE {
            for y in 0..SIZE {
                let p = (x as i32, y as i32);
                if update_neighbours(grid, &p, &mut flashed) {
                    updated = true;
                    count += 1;
                }
            }
        }
    }
    grid.iter_mut().for_each(|val| {
        if *val > 9 {
            *val = 0;
        }
    });
    count
}

pub fn solve_part_1(input_str: &str) -> usize {
    let mut grid: Grid = input_str
        .lines()
        .flat_map(|row| row.chars().map(move |energy| energy.to_digit(10).unwrap()))
        .collect();

    (0..100).map(|_| iter(&mut grid)).sum()
}
pub fn solve_part_2(input_str: &str) -> usize {
    let mut grid: Grid = input_str
        .lines()
        .flat_map(|row| row.chars().map(move |energy| energy.to_digit(10).unwrap()))
        .collect();

    (1..).find(|_| iter(&mut grid) == SIZE * SIZE).unwrap()
}

#[cfg(test)]
mod test {

    const TESTCASE: &str = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test1() {
        let res = super::solve_part_1(TESTCASE);
        assert_eq!(res, 1656);
    }

    #[test]
    fn test2() {
        let res = super::solve_part_2(TESTCASE);
        assert_eq!(res, 195);
    }
}
