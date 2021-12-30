use hashbrown::HashMap;

use std::fmt::Display;

pub fn print_2d_map<V: Display>(a: &HashMap<(i32, i32), V>) {
    let minx = *a.iter().map(|((x, _y), _v)| x).min().unwrap();
    let maxx = *a.iter().map(|((x, _y), _v)| x).max().unwrap();
    let miny = *a.iter().map(|((_x, y), _v)| y).min().unwrap();
    let maxy = *a.iter().map(|((_x, y), _v)| y).max().unwrap();
    for y in miny..=maxy {
        for x in minx..=maxx {
            match a.get(&(x, y)) {
                Some(v) => print!("{}", v),
                _ => print!("."),
            }
        }
        println!("");
    }
}

pub fn get_neighbours_4<'a>(pos: &'a (i32, i32)) -> Box<dyn Iterator<Item = (i32, i32)> + 'a> {
    const NEIGHBOURS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    Box::new(NEIGHBOURS.iter().map(|(dx, dy)| (pos.0 + dx, pos.1 + dy)))
}
