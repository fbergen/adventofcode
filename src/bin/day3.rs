#[derive(Debug)]
pub struct TreeMap<'a> {
    map: Vec<&'a str>,
    num_rows: usize,
    num_cols: usize,
}

impl TreeMap<'_> {
    fn is_tree(&self, x: usize, y: usize) -> bool {
        match self.map[y].chars().nth(x % self.num_cols) {
            Some('#') => true,
            _ => false,
        }
    }
}

pub fn main() {
    let input: Vec<&str> = include_str!("../inputs/day3").lines().collect();
    let map = TreeMap {
        num_rows: input.len(),
        num_cols: input[0].len(),
        map: input,
    };

    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut product: u64 = 1;
    for slope in slopes {
        let (mut x, mut y, mut num_trees) = (0, 0, 0);
        loop {
            x += slope.0;
            y += slope.1;
            if y >= map.num_rows {
                break;
            }
            if map.is_tree(x, y) {
                num_trees += 1;
            }
        }
        product *= num_trees;
    }
    println!("{:?}", product);
}
