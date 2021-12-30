#![allow(unused_imports)]
use aoc2021::print_2d_map;

use hashbrown::HashMap;

type Index = (i32, i32);
type Val = char;
type Map = HashMap<(i32, i32), char>;

fn parse(input: &str) -> Map {
    input
        .lines()
        .enumerate()
        .map(|(row, s)| {
            s.chars()
                .enumerate()
                .map(|(col, c)| ((col as i32, row as i32), c))
                .collect::<Vec<(Index, Val)>>()
        })
        .flatten()
        .collect()
}

fn get_moves(m: &Map, east: bool) -> Vec<(i32, i32)> {
    m.iter()
        .filter(|((x, y), val)| {
            **val == if east { '>' } else { 'v' }
                && match m.get(&(x + if east { 1 } else { 0 }, y + if east { 0 } else { 1 })) {
                    Some(p) => *p == '.',
                    _ => {
                        if east {
                            *m.get(&(0, *y)).unwrap() == '.'
                        } else {
                            *m.get(&(*x, 0)).unwrap() == '.'
                        }
                    }
                }
        })
        .map(|v| *v.0)
        .collect()
}

fn make_moves(xsize: i32, ysize: i32, map: &mut Map, east: bool) -> usize {
    let moves = get_moves(map, east);
    for (x, y) in &moves {
        map.insert((*x, *y), '.');
        let nx = if east && *x >= xsize - 1 { -1 } else { *x };
        let ny = if !east && *y >= ysize - 1 { -1 } else { *y };
        map.insert(
            (nx + if east { 1 } else { 0 }, ny + if east { 0 } else { 1 }),
            if east { '>' } else { 'v' },
        );
    }
    moves.len()
}

pub fn solve_part_1(input_str: &str) -> usize {
    let mut m = parse(input_str);
    let xsize = (m.iter().map(|((x, _), _)| x).max().unwrap() + 1) as i32;
    let ysize = (m.iter().map(|((_, y), _)| y).max().unwrap() + 1) as i32;

    println!("\n\n");
    for i in 1.. {
        if make_moves(xsize, ysize, &mut m, true) + make_moves(xsize, ysize, &mut m, false) == 0 {
            return i;
        }
    }
    0
}
pub fn solve_part_2(_input_str: &str) -> usize {
    0
}

#[cfg(test)]
mod test {

    const TESTCASE: &str = "\
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    #[test]
    fn test1() {
        let res = super::solve_part_1(TESTCASE);
        assert_eq!(res, 58);
    }
}
