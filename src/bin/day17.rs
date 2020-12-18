use std::collections::HashMap;

fn main() {
    let input: &str = include_str!("../inputs/day17");
    println!("part1: {}", part1(input).unwrap());
    println!("part2: {}", part2(input).unwrap());
}

type Map = HashMap<(isize, isize, isize), bool>;
type Map2 = HashMap<(isize, isize, isize, isize), bool>;

fn part2(input_str: &str) -> Option<usize> {
    let mut map: Map2 = HashMap::new();

    let z_dim = 0;
    let w_dim = 0;
    let x_dim = input_str.lines().next()?.len() as isize - 1;
    let y_dim = input_str.lines().count() as isize - 1;
    input_str.lines().enumerate().for_each(|(x, row)| {
        row.chars().enumerate().for_each(|(y, chr)| {
            if chr == '#' {
                map.insert(
                    (x as isize, y as isize, z_dim as isize, w_dim as isize),
                    true,
                );
            }
        })
    });

    let mut dim = ((0, x_dim), (0, y_dim), (0, z_dim), (0, w_dim));

    let steps = 6;
    for _i in 0..steps {
        dim = (
            (dim.0 .0 - 1, dim.0 .1 + 1),
            (dim.1 .0 - 1, dim.1 .1 + 1),
            (dim.2 .0 - 1, dim.2 .1 + 1),
            (dim.3 .0 - 1, dim.3 .1 + 1),
        );
        let mut new_map: Map2 = HashMap::new();
        for x in dim.0 .0..=dim.0 .1 {
            for y in dim.1 .0..=dim.1 .1 {
                for z in dim.2 .0..=dim.2 .1 {
                    for w in dim.3 .0..=dim.3 .1 {
                        new_map.insert((x, y, z, w), new_val2(&map, x, y, z, w));
                    }
                }
            }
        }
        map = new_map.clone();
    }

    let active = map.values().filter(|v| **v == true).count();

    Some(active)
}

fn new_val2(map: &Map2, x: isize, y: isize, z: isize, w: isize) -> bool {
    let v = *map.get(&(x, y, z, w)).unwrap_or(&false);

    let mut active_neigbors = 0;
    if v {
        active_neigbors -= 1;
    }

    for ix in x - 1..=x + 1 {
        for iy in y - 1..=y + 1 {
            for iz in z - 1..=z + 1 {
                for iw in w - 1..=w + 1 {
                    if map.get(&(ix, iy, iz, iw)) == Some(&true) {
                        active_neigbors += 1;
                    }
                }
                if active_neigbors > 3 {
                    break;
                }
            }
            if active_neigbors > 3 {
                break;
            }
        }
        if active_neigbors > 3 {
            break;
        }
    }
    if v {
        return active_neigbors == 2 || active_neigbors == 3;
    }
    active_neigbors == 3
}

fn part1(input_str: &str) -> Option<usize> {
    let mut map: Map = HashMap::new();

    let z_dim = 0;
    let x_dim = input_str.lines().next()?.len() as isize - 1;
    let y_dim = input_str.lines().count() as isize - 1;
    input_str.lines().enumerate().for_each(|(x, row)| {
        row.chars().enumerate().for_each(|(y, chr)| {
            if chr == '#' {
                map.insert((x as isize, y as isize, z_dim as isize), true);
            }
        })
    });

    let mut dim = ((0, x_dim), (0, y_dim), (0, z_dim));

    let steps = 6;
    for _i in 0..steps {
        dim = (
            (dim.0 .0 - 1, dim.0 .1 + 1),
            (dim.1 .0 - 1, dim.1 .1 + 1),
            (dim.2 .0 - 1, dim.2 .1 + 1),
        );
        let mut new_map: Map = HashMap::new();
        for x in dim.0 .0..=dim.0 .1 {
            for y in dim.1 .0..=dim.1 .1 {
                for z in dim.2 .0..=dim.2 .1 {
                    new_map.insert((x, y, z), new_val(&map, x, y, z));
                }
            }
        }
        map = new_map.clone();
    }

    let active = map.values().filter(|v| **v == true).count();

    Some(active)
}

fn new_val(map: &Map, x: isize, y: isize, z: isize) -> bool {
    let v = *map.get(&(x, y, z)).unwrap_or(&false);

    let mut active_neigbors = 0;

    for ix in x - 1..=x + 1 {
        for iy in y - 1..=y + 1 {
            for iz in z - 1..=z + 1 {
                if map.get(&(ix, iy, iz)) == Some(&true) {
                    active_neigbors += 1;
                }
            }
        }
    }
    if v {
        // I count the node itself too, hence inc by one on req.
        return active_neigbors == 3 || active_neigbors == 4;
    }
    active_neigbors == 3
}

#[cfg(test)]
mod test {
    use super::*;

    const TESTCASE: &str = "\
.#.
..#
###";

    #[test]
    fn test_day17() {
        assert_eq!(part1(TESTCASE).unwrap(), 112);
        assert_eq!(part2(TESTCASE).unwrap(), 848);
    }
}
