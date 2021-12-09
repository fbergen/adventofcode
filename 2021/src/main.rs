macro_rules! aoc {
    ( $($m:expr => $module:ident),* ) => {

        fn main() {
            let args: Vec<_> = std::env::args().collect();
            if args.len() == 1 {
                panic!("Need to specify a problem number");
            }

            let problem_num: u32 = args[1].parse::<u32>().unwrap();
            let iter: u32 = if args.len() >= 3 { args[2].parse::<u32>().unwrap() } else {1};

            use std::time::Instant;
            match problem_num {
                $(
                    $m => {
                        let start = Instant::now();
                        let mut part1 = 0;
                        for _ in 0..iter {
                            part1 = $module::solve_part_1(include_str!(concat!("inputs/day", $m)));
                        }
                        println!("Part 1: {} ({}µs)", part1, start.elapsed().as_micros());
                        let start = Instant::now();
                        let mut part2 = 0;
                        for _ in 0..iter {
                            part2 = $module::solve_part_2(include_str!(concat!("inputs/day", $m)));
                        }
                        println!("Part 2: {} ({}µs)", part2, start.elapsed().as_micros());
                    }
                )*
                _ => panic!("Unknown problem number")
            }
        }
    }
}

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

aoc! {
    1 => day01,
    2 => day02,
    3 => day03,
    4 => day04,
    5 => day05,
    6 => day06,
    7 => day07,
    8 => day08,
    9 => day09,
    10 => day10,
    11 => day11,
    12 => day12,
    13=> day13,
    14 => day14,
    15 => day15,
    16 => day16,
    17 => day17,
    18 => day18,
    19 => day19,
    20 => day20,
    21 => day21,
    22 => day22,
    23 => day23,
    24 => day24,
    25 => day25
}
