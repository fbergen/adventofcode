use std::mem;

#[derive(Debug, PartialEq)]
pub struct SeatMap {
    seats: Vec<Vec<Seat>>,
}

#[derive(Debug, PartialEq, Copy)]
pub enum Seat {
    Empty,
    Occupied,
    Floor,
}

impl Clone for Seat {
    fn clone(&self) -> Seat {
        match self {
            Seat::Empty => Seat::Empty,
            Seat::Occupied => Seat::Occupied,
            Seat::Floor => Seat::Floor,
        }
    }
}

impl SeatMap {
    fn get(&self, x: usize, y: usize) -> Option<&Seat> {
        if x >= self.seats.len() || y >= self.seats[0].len() {
            return None;
        }

        Some(&self.seats[x][y])
    }

    fn geti(&self, x: isize, y: isize) -> Option<&Seat> {
        if x < 0 || y < 0 {
            return None;
        }

        self.get(x as usize, y as usize)
    }

    fn first_seat(&self, from: (usize, usize), dir: &(isize, isize)) -> Option<&Seat> {
        let mut x: isize = from.0 as isize;
        let mut y: isize = from.1 as isize;

        x += dir.0;
        y += dir.1;

        let mut s = self.geti(x, y);

        while s == Some(&Seat::Floor) {
            x += dir.0;
            y += dir.1;
            s = self.geti(x, y);
        }
        return s;
    }

    fn next_val_2(&self, x: usize, y: usize) -> Seat {
        let s = self.get(x, y);

        let dirs = vec![
            (-1, 0),
            (-1, 1),
            (-1, -1),
            (0, 1),
            (0, -1),
            (1, 1),
            (1, 0),
            (1, -1),
        ];

        let seen_occ = dirs
            .iter()
            .map(|dir| self.first_seat((x, y), dir))
            .filter(|s| *s == Some(&Seat::Occupied))
            .count();

        match s {
            Some(Seat::Occupied) => {
                if seen_occ >= 5 {
                    return Seat::Empty;
                } else {
                    return Seat::Occupied;
                }
            }
            Some(Seat::Empty) => {
                if seen_occ == 0 {
                    return Seat::Occupied;
                } else {
                    return Seat::Empty;
                }
            }
            _ => {
                return Seat::Floor;
            }
        }
    }

    // - If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
    // - If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
    // - Otherwise, the seat's state does not change.
    fn next_val_1(&self, x: usize, y: usize) -> Seat {
        let s = self.get(x, y);
        let fromx = if x >= 1 { x - 1 } else { 0 };
        let fromy = if y >= 1 { y - 1 } else { 0 };

        let occ_neighbors = (fromx..=x + 1)
            .map(|nx| {
                (fromy..=y + 1)
                    .filter(|ny| self.get(nx, *ny) == Some(&Seat::Occupied))
                    .count()
            })
            .sum::<usize>();

        match s {
            Some(Seat::Occupied) => {
                if occ_neighbors >= 5 {
                    return Seat::Empty;
                } else {
                    return Seat::Occupied;
                }
            }
            Some(Seat::Empty) => {
                if occ_neighbors == 0 {
                    return Seat::Occupied;
                } else {
                    return Seat::Empty;
                }
            }
            _ => {
                return Seat::Floor;
            }
        }
    }
}

fn main() {
    let input: &str = include_str!("../inputs/day11");
    // println!("Part 1: {}", solve(input, true));
    println!("Part 2: {}", solve(input, false));
}

fn next_seat_map(old: &SeatMap, part1: bool, new_seats: &mut Vec<Vec<Seat>>) {
    for i in 0..old.seats.len() {
        for j in 0..old.seats[i].len() {
            if part1 {
                new_seats[i][j] = old.next_val_1(i, j);
            } else {
                new_seats[i][j] = old.next_val_2(i, j);
            }
        }
    }
}

fn solve(input: &str, part1: bool) -> usize {
    let inv = input
        .lines()
        .map(|p| {
            p.to_string()
                .chars()
                .map(|x| match x {
                    'L' => Seat::Empty,
                    '#' => Seat::Occupied,
                    '.' => Seat::Floor,
                    _ => panic!(""),
                })
                .collect()
        })
        .collect();
    let mut sm = SeatMap { seats: inv };

    let mut new_seats: Vec<Vec<Seat>> = vec![vec![Seat::Floor; sm.seats[0].len()]; sm.seats.len()];
    let mut changed = true;
    while changed {
        next_seat_map(&sm, part1, &mut new_seats);
        changed = new_seats != sm.seats;

        // Swap the old vector with the new
        new_seats = mem::replace(&mut sm.seats, new_seats);
    }

    return sm
        .seats
        .iter()
        .map(|row| row.iter().filter(|s| **s == Seat::Occupied).count())
        .sum::<usize>();

    // println!("{:?}", input);
}

#[cfg(test)]
mod test {
    use super::*;

    const TESTCASE: &str = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn test_day11() {
        assert_eq!(solve(TESTCASE, true), 37);
        assert_eq!(solve(TESTCASE, false), 26);
    }
}
