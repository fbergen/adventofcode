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
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

aoc! {
    1 => day1,
    2 => day2,
    3 => day3,
    4 => day4,
    5 => day5,
    6 => day6
}
