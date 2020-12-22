use std::collections::HashMap;
use std::fmt::Display;

fn main() {
    let input: &str = include_str!("../inputs/day20");
    println!("part1: {}", solve(input, true).unwrap());
    println!("part2: {}", solve(input, false).unwrap());
}

#[derive(PartialEq, Debug, Clone)]
struct Tile {
    id: usize,
    data: Vec<Vec<bool>>,
    edges: Vec<u32>,
    uniq_edges: usize,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Tile id: {}", self.id)?;
        for row in self.data.iter() {
            let s: String = row
                .iter()
                .map(|b| match *b {
                    true => "#",
                    false => ".",
                })
                .collect();
            writeln!(f, "{}", s)?;
        }
        Ok(())
    }
}

impl Tile {
    fn rotate_right(&mut self) {
        self.edges.rotate_right(1);
        self.edges[0] = self.edges[0].reverse_bits() >> 22;
        self.edges[2] = self.edges[2].reverse_bits() >> 22;

        let mut n_data = self.data.clone();
        let dim = self.data.len();
        for i in 0..dim {
            for j in 0..dim {
                n_data[j][dim - i - 1] = self.data[i][j];
            }
        }
        self.data = n_data;
    }

    fn flip_cols(&mut self) {
        self.edges.swap(0, 2);
        self.edges[1] = self.edges[1].reverse_bits() >> 22;
        self.edges[3] = self.edges[3].reverse_bits() >> 22;

        let mut n_data = self.data.clone();
        let dim = self.data.len();
        for i in 0..dim {
            for j in 0..dim {
                n_data[i][j] = self.data[dim - i - 1][j];
            }
        }
        self.data = n_data;
    }

    fn flip_rows(&mut self) {
        self.edges.swap(1, 3);
        self.edges[0] = self.edges[0].reverse_bits() >> 22;
        self.edges[2] = self.edges[2].reverse_bits() >> 22;

        let mut n_data = self.data.clone();
        let dim = self.data.len();
        for i in 0..dim {
            for j in 0..dim {
                n_data[i][j] = self.data[i][dim - j - 1];
            }
        }
        self.data = n_data;
    }
}

struct Puzzle<'a> {
    dim: usize,
    tiles: Vec<&'a mut Tile>,
}

impl Puzzle<'_> {
    fn get_tile(&self, row: isize, col: isize) -> Option<&Tile> {
        if row >= 0 && row < self.dim as isize && col >= 0 && col < self.dim as isize {
            return Some(*self.tiles.get(col as usize + row as usize * self.dim)?);
        }
        None
    }

    fn neighbor_up_edge(&self, row: isize, col: isize) -> Option<u32> {
        return match self.get_tile(row - 1, col) {
            Some(t) => Some(t.edges[2]),
            None => None,
        };
    }

    fn neighbor_left_edge(&self, row: isize, col: isize) -> Option<u32> {
        return match self.get_tile(row, col - 1) {
            Some(t) => Some(t.edges[1]),
            None => None,
        };
    }
}

fn read_tiles(input_str: &str) -> Vec<Tile> {
    let dim = 10;
    let sect = input_str.split("\n\n");
    let tiles: Vec<Tile> = sect
        .map(|r| {
            let id = r
                .lines()
                .next()
                .unwrap()
                .strip_prefix("Tile ")
                .unwrap()
                .strip_suffix(":")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let data: Vec<Vec<bool>> = r
                .lines()
                .skip(1)
                .map(|r| r.chars().map(|c| c == '#').collect())
                .collect();

            let mut edges: Vec<u32> = vec![];
            edges.push(data[0].iter().fold(0, |acc, x| match x {
                true => acc * 2 + 1,
                false => acc * 2,
            }));

            edges.push(data.iter().fold(0, |acc, x| match x[dim - 1] {
                true => acc * 2 + 1,
                false => acc * 2,
            }));

            edges.push(data[dim - 1].iter().fold(0, |acc, x| match x {
                true => acc * 2 + 1,
                false => acc * 2,
            }));

            edges.push(data.iter().fold(0, |acc, x| match x[0] {
                true => acc * 2 + 1,
                false => acc * 2,
            }));

            Tile {
                id,
                edges,
                data: data,
                uniq_edges: 0,
            }
        })
        .collect();

    tiles
}

fn init_puzzle<'a>(tiles: &'a mut Vec<Tile>, dim: usize) -> Puzzle<'a> {
    Puzzle {
        tiles: tiles.iter_mut().map(|t: &mut Tile| &mut *t).collect(),
        dim: dim,
    }
}

fn solve(input_str: &str, part1: bool) -> Option<usize> {
    let mut tiles = read_tiles(input_str);

    let len = tiles.len();
    let dim = (len as f64).sqrt() as usize;

    let mut adj_list: HashMap<u32, Vec<usize>> = HashMap::new();
    tiles.iter().for_each(|t| {
        t.edges.iter().for_each(|e| {
            adj_list.entry(*e).or_insert(vec![]).push(t.id);
            adj_list
                .entry(e.reverse_bits() >> 22)
                .or_insert(vec![])
                .push(t.id);
        })
    });

    tiles.iter_mut().for_each(|t| {
        t.uniq_edges = t
            .edges
            .iter()
            .filter(|e| adj_list[*e].len() == 1 || adj_list[&(e.reverse_bits() >> 22)].len() == 1)
            .count();
    });

    let mut puzzle = init_puzzle(&mut tiles, dim);

    let d = dim as isize - 1;

    solve_puzzle(&adj_list, &mut puzzle, 0);

    if part1 {
        return Some(
            puzzle.get_tile(0, 0)?.id
                * puzzle.get_tile(d, 0)?.id
                * puzzle.get_tile(d, d)?.id
                * puzzle.get_tile(0, d)?.id,
        );
    }

    let mut p: Vec<Vec<bool>> = vec![];
    for r in 0..puzzle.dim * 10 {
        if r % 10 != 0 && r % 10 != 9 {
            let mut row = vec![];
            for c in 0..puzzle.dim * 10 {
                if c % 10 != 0 && c % 10 != 9 {
                    row.push(
                        puzzle.get_tile((r / 10) as isize, (c / 10) as isize)?.data[r % 10][c % 10],
                    );
                }
            }
            p.push(row);
        }
    }

    let mut tile = Tile {
        data: p,
        id: 0,
        edges: vec![0, 0, 0, 0],
        uniq_edges: 0,
    };

    let monster_idxs: Vec<(usize, usize)> = "\
.                 # 
#    ##    ##    ###
 #  #  #  #  #  #"
        .split("\n")
        .enumerate()
        .flat_map(|(row, s)| {
            s.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(col, _)| (row, col))
        })
        .collect();

    let num_monsters = count_monster_configs(&mut tile, &monster_idxs);
    let num_non_monsters = tile.data.iter().flat_map(|b| b).filter(|b| **b).count()
        - num_monsters * monster_idxs.len();
    Some(num_non_monsters)
}

fn count_monsters(tile: &Tile, monster_idxs: &Vec<(usize, usize)>) -> usize {
    let max_row = monster_idxs.iter().map(|(row, _)| row).max().unwrap();
    let max_col = monster_idxs.iter().map(|(_, col)| col).max().unwrap();

    let mut count = 0;
    for irow in 0..tile.data.len() - max_row {
        for icol in 0..tile.data.len() - max_col {
            // check if monster starts at (row, col)
            if monster_idxs
                .iter()
                .all(|(r, c)| tile.data[irow + r][icol + c])
            {
                count += 1;
            }
        }
    }
    count
}

fn count_monster_configs(t: &mut Tile, monster_idxs: &Vec<(usize, usize)>) -> usize {
    (0..2)
        .map(|_| {
            let mut count = count_monsters(t, monster_idxs);

            t.flip_cols();
            count += count_monsters(t, monster_idxs);

            t.flip_rows();
            count += count_monsters(t, monster_idxs);

            t.flip_cols();
            count += count_monsters(t, monster_idxs);
            // back to normal maybe not needed
            t.flip_rows();

            t.rotate_right();
            count
        })
        .sum()
}

fn solve_puzzle(adj_list: &HashMap<u32, Vec<usize>>, mut puzzle: &mut Puzzle, i: usize) -> bool {
    let len = puzzle.tiles.len();

    if i >= len {
        return true;
    }
    // find the list of matching tiles
    let up = puzzle.neighbor_up_edge((i / puzzle.dim) as isize, (i % puzzle.dim) as isize);
    let left = puzzle.neighbor_left_edge((i / puzzle.dim) as isize, (i % puzzle.dim) as isize);

    let tids_to_try: Vec<usize> = puzzle
        .tiles
        .iter()
        .skip(i)
        .filter(|t| match up {
            Some(x) => adj_list[&x].contains(&t.id),
            None => t.uniq_edges >= 1,
        })
        .filter(|t| match left {
            Some(x) => adj_list[&x].contains(&t.id),
            None => t.uniq_edges >= 1,
        })
        .filter(|t| {
            if left.is_none() && up.is_none() {
                t.uniq_edges == 2
            } else {
                true
            }
        })
        .map(|t| t.id)
        .collect();

    for tid in tids_to_try {
        // Move the tid to current index
        let idx = puzzle.tiles.iter().position(|t| t.id == tid).unwrap();
        puzzle.tiles.swap(i, idx);
        if (0..2)
            .find(|_| {
                {
                    let t: &mut Tile = puzzle.tiles.get_mut(i).unwrap();
                    t.rotate_right();
                    if is_matching(&puzzle.tiles.get(i).unwrap(), up, left)
                        && solve_puzzle(adj_list, &mut puzzle, i + 1)
                    {
                        return true;
                    }
                }
                {
                    // vertical flip
                    let t: &mut Tile = puzzle.tiles.get_mut(i).unwrap();
                    t.flip_cols();
                    if is_matching(&t, up, left) && solve_puzzle(adj_list, &mut puzzle, i + 1) {
                        return true;
                    }
                }
                {
                    // both flip
                    let t: &mut Tile = puzzle.tiles.get_mut(i).unwrap();
                    t.flip_rows();
                    if is_matching(&t, up, left) && solve_puzzle(adj_list, &mut puzzle, i + 1) {
                        return true;
                    }
                }
                {
                    // horiz flip
                    let t: &mut Tile = puzzle.tiles.get_mut(i).unwrap();
                    t.flip_cols();
                    if is_matching(&t, up, left) && solve_puzzle(adj_list, &mut puzzle, i + 1) {
                        return true;
                    }
                }
                {
                    // back to normal maybe not needed
                    let t: &mut Tile = puzzle.tiles.get_mut(i).unwrap();
                    t.flip_rows();
                }

                false
            })
            .is_some()
        {
            return true;
        }
    }
    return false;
}

fn is_matching(t: &Tile, up: Option<u32>, left: Option<u32>) -> bool {
    let up_match = up.is_none() || up.unwrap() == t.edges[0];
    let left_match = left.is_none() || left.unwrap() == t.edges[3];
    up_match && left_match
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day20() {
        let input: &str = include_str!("../inputs/day20test");
        assert_eq!(solve(input, true).unwrap(), 20899048083289);
        assert_eq!(solve(input, false).unwrap(), 273);
    }
}
