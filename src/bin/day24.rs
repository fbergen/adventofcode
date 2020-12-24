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

fn move_tile(row: isize, col: isize, d: &Dir) -> (isize, isize) {
    let mut nrow = row;
    let mut ncol = col;

    match d {
        Dir::NE | Dir::NW => nrow -= 1,
        Dir::SE | Dir::SW => nrow += 1,
        _ => {}
    }
    match d {
        Dir::E => ncol += 1,
        Dir::W => ncol -= 1,
        _ => {}
    }
    if row % 2 == 0 {
        // even
        match d {
            Dir::NW | Dir::SW => ncol -= 1,
            _ => {}
        }
    } else {
        // odd
        match d {
            Dir::NE | Dir::SE => ncol += 1,
            _ => {}
        }
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
        let coord = t
            .dir
            .iter()
            .fold((0, 0), |acc, x| move_tile(acc.0, acc.1, x));

        *tile_map.entry(coord).or_insert(false) ^= true;
    }
    if part1 {
        return tile_map.values().filter(|&&x| x).count();
    }

    for _ in 0..100 {
        let mut neighbor_black: HashMap<(isize, isize), usize> = HashMap::new();
        tile_map.iter().for_each(|((r, c), v)| {
            let inc = if *v { 1 } else { 0 };
            neighbor_black.entry((*r, *c)).or_insert(0);
            *neighbor_black
                .entry(move_tile(*r, *c, &Dir::E))
                .or_insert(0) += inc;
            *neighbor_black
                .entry(move_tile(*r, *c, &Dir::SE))
                .or_insert(0) += inc;
            *neighbor_black
                .entry(move_tile(*r, *c, &Dir::SW))
                .or_insert(0) += inc;
            *neighbor_black
                .entry(move_tile(*r, *c, &Dir::W))
                .or_insert(0) += inc;
            *neighbor_black
                .entry(move_tile(*r, *c, &Dir::NW))
                .or_insert(0) += inc;
            *neighbor_black
                .entry(move_tile(*r, *c, &Dir::NE))
                .or_insert(0) += inc;
        });
        neighbor_black.iter().for_each(|(coord, &num)| {
            if *tile_map.get(coord).unwrap_or(&false) {
                if num == 0 || num > 2 {
                    tile_map.insert(*coord, false);
                }
            } else {
                if num == 2 {
                    tile_map.insert(*coord, true);
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
