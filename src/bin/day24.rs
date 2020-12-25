use recap::Recap;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Dir {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}
use Dir::*;
static DIRS: [Dir; 6] = [E, SE, SW, W, NW, NE];

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r"^(?P<dir>.*)$")]
pub struct Tile {
    dir: Vec<Dir>,
}

fn main() {
    let input: &str = include_str!("../inputs/day24");
    println!("part1: {}", solve(input, true));
    println!("part2: {}", solve(input, false));
}

fn mv_tile(row: isize, col: isize, d: &Dir) -> (isize, isize) {
    let mut nrow = row;
    let mut ncol = col;

    match d {
        NE | NW => nrow -= 1,
        SE | SW => nrow += 1,
        E => ncol += 1,
        W => ncol -= 1,
    }
    match d {
        NW | SW if row % 2 == 0 => ncol -= 1, // even
        NE | SE if row % 2 != 0 => ncol += 1, // odd
        _ => {}
    }

    (nrow, ncol)
}

fn solve(input_str: &str, part1: bool) -> usize {
    let tiles: Vec<Tile> = input_str
        .lines()
        .map(|s| {
            let mut css = s.replace("e", "e,").replace("w", "w,");
            css.remove(css.len() - 1);
            css.parse::<Tile>().unwrap()
        })
        .collect();

    let mut tile_map: HashMap<(isize, isize), bool> = HashMap::new();
    for t in tiles {
        let coord = t.dir.iter().fold((0, 0), |acc, x| mv_tile(acc.0, acc.1, x));

        *tile_map.entry(coord).or_insert(false) ^= true;
    }
    if part1 {
        return tile_map.values().filter(|&&x| x).count();
    }

    let mut neighbor_black: HashMap<(isize, isize), usize> = HashMap::new();
    for _ in 0..100 {
        tile_map
            .iter()
            .filter(|(_, v)| **v)
            .for_each(|((r, c), _)| {
                neighbor_black.entry((*r, *c)).or_insert(0);
                for d in DIRS.iter() {
                    *neighbor_black.entry(mv_tile(*r, *c, d)).or_insert(0) += 1;
                }
            });

        neighbor_black.drain().for_each(|(coord, num)| {
            if *tile_map.get(&coord).unwrap_or(&false) {
                if num == 0 || num > 2 {
                    tile_map.insert(coord, false);
                }
            } else {
                if num == 2 {
                    tile_map.insert(coord, true);
                }
            }
        });
    }

    return tile_map.values().filter(|&&x| x).count();
}

#[cfg(test)]
mod test {
    use super::*;

    const TESTCASE: &str = "\
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

    #[test]
    fn test_day24() {
        assert_eq!(solve(TESTCASE, true), 10);
        assert_eq!(solve(TESTCASE, false), 2208);
    }
}
