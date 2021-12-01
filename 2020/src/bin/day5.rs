pub fn main() {
    let ids: Vec<u64> = include_str!("../inputs/day5")
        .lines()
        .map(|s| {
            u64::from_str_radix(
                &s.replace('F', "0")
                    .replace('B', "1")
                    .replace('L', "0")
                    .replace('R', "1"),
                2,
            )
            .unwrap()
        })
        .collect();
    let (min, max) = (*ids.iter().min().unwrap(), *ids.iter().max().unwrap());

    println!(
        "answ part1 {:?}, part2 {:?}",
        max,
        (min..=max).sum::<u64>() - ids.iter().sum::<u64>()
    );
}
