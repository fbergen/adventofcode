macro_rules! aoc {
    ( $($m:expr => $module:ident),* ) => {

        fn main() {
            let args: Vec<_> = std::env::args().collect();
            if args.len() == 0 {
                panic!("Need to specify a problem number");
            }

            let problem_num: u32 = args[1].parse::<u32>().unwrap();

            use std::time::Instant;
            match problem_num {
                $(
                    $m => {
                        let start = Instant::now();
                        let part1 = $module::solve_part_1(include_str!(concat!("inputs/day", $m)));
                        println!("Part 1: {} ({}µs)", part1, start.elapsed().as_micros());
                        let start = Instant::now();
                        let part2 = $module::solve_part_2(include_str!(concat!("inputs/day", $m)));
                        println!("Part 2: {} ({}µs)", part2, start.elapsed().as_micros());
                    }
                )*
                _ => panic!("Unknown problem number")
            }
        }
    }
}

mod day1;
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
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

aoc! {
    1 => day1,
    2 => day2,
    3 => day3,
    4 => day4,
    5 => day5,
    6 => day6,
    7 => day7,
    8 => day8,
    9 => day9,
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
    20=> day20,
    21=> day21,
    22=> day22,
    23=> day23,
    24=> day24,
    25=> day25
}
