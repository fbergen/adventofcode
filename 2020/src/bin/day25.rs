fn main() {
    let pub_keys: Vec<usize> = include_str!("../inputs/day25")
        .lines()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    println!("part1: {}", iter(pub_keys[1], get_loop_size(pub_keys[0])));
}

fn iter(key: usize, l: usize) -> usize {
    let mut num = 1;
    for _ in 0..l {
        num = (num * key) % 20201227;
    }
    num
}

fn get_loop_size(key: usize) -> usize {
    let mut num = 1;
    let mut l = 0;
    while num != key {
        num = (num * 7) % 20201227;
        l += 1;
    }
    l
}
