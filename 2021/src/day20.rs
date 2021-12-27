use hashbrown::HashMap;

type EnhAlgo = Vec<u8>;
type Image = HashMap<(i32, i32), u8>;

fn parse(input: &str) -> (EnhAlgo, Image) {
    let (iea, image_str) = input.split_once("\n\n").unwrap();
    let algo: EnhAlgo = iea
        .chars()
        .map(|c| match c {
            '#' => 1,
            '.' => 0,
            _ => panic!(""),
        })
        .collect();

        let img =
            image_str.lines().enumerate().map(
    |row, s| {
    s.chars().enumerate().map(|col, c| 
        match c {
            '#' => ((row, col), 1),
            '.' => ((row, col), 0),
            _ => panic!("");
        }
    ).collect()
    });
}

pub fn solve_part_1(input_str: &str) -> usize {
    let (iea, imagage) = parse(input_str);

    0
}
pub fn solve_part_2(_input_str: &str) -> usize {
    0
}

#[cfg(test)]
mod test {

    const TESTCASE: &str = "\
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    #[test]
    fn test1() {
        let res = super::solve_part_1(TESTCASE);
        assert_eq!(res, 35);
    }

    #[test]
    fn test2() {
        let res = super::solve_part_2(TESTCASE);
        assert_eq!(res, 0);
    }
}
