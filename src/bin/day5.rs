pub fn main() {
    let mut ids: Vec<u64> = include_str!("../inputs/day5")
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
    println!("part1 {:?}", ids.iter().max().unwrap());

    ids.sort();
    println!(
        "part2 {:?}",
        ids.iter()
            .zip((*ids.first().unwrap())..)
            .find(|(id, i)| *id != i)
            .unwrap()
            .1
    );
}
