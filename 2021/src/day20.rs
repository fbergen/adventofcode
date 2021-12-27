use hashbrown::HashMap;

type EnhAlgo = Vec<usize>;
type Image = HashMap<(i32, i32), usize>;

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

    let img = image_str
        .lines()
        .enumerate()
        .map(|(row, s)| {
            s.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '#' => ((col as i32, row as i32), 1),
                    '.' => ((col as i32, row as i32), 0),
                    _ => panic!(""),
                })
                .collect::<Vec<((i32, i32), usize)>>()
        })
        .flatten()
        .collect();

    (algo, img)
}

const NEIGHBOURS: [(i32, i32); 9] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (0, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub fn enhance(input_str: &str, iter: usize) -> usize {
    let (iea, mut image) = parse(input_str);
    let img_size = *image.iter().map(|((r, _c), _v)| r).max().unwrap();

    let mut filling_pixels = 0;

    let mut img_growth = 1; // image grows with 2 rows & cols each iteration.
    for _ in 0..iter {
        let mut new_image: Image = HashMap::new();
        for i in (-img_growth)..=(img_size + img_growth) {
            for j in (-img_growth)..=(img_size + img_growth) {
                new_image.insert(
                    (i, j),
                    iea[NEIGHBOURS
                        .iter()
                        .map(|(dx, dy)| image.get(&(i + dx, j + dy)).unwrap_or(&filling_pixels))
                        .fold(0, |acc, x| acc * 2 + x) as usize],
                );
            }
        }

        image = new_image.clone();
        filling_pixels = match filling_pixels == 1 {
            true => iea[iea.len() - 1],
            false => iea[0],
        };
        img_growth += 1;
    }
    image.drain_filter(|_k, v| *v == 1).count()
}

pub fn solve_part_1(input_str: &str) -> usize {
    enhance(input_str, 2)
}

pub fn solve_part_2(input_str: &str) -> usize {
    enhance(input_str, 50)
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
        assert_eq!(res, 3351);
    }
}
