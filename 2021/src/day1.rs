pub fn solve_part_1(input_str: &str) -> usize {
    let input: Vec<usize> = input_str
        .lines()
        .map(|p| p.parse::<usize>().expect(&format!("Parse error {:?}", p)))
        .collect();

    count_increasing(&input)
}
pub fn solve_part_2(input_str: &str) -> usize {
    let input: Vec<usize> = input_str
        .lines()
        .map(|p| p.parse::<usize>().expect(&format!("Parse error {:?}", p)))
        .collect();

    let iter = input.windows(3);
    let w = iter.map(|x| x.iter().sum()).collect();

    count_increasing(&w)
}

fn count_increasing(input: &Vec<usize>) -> usize {
    let mut prev = input[0];
    let mut count = 0;
    for i in 1..input.len() {
        let v = input[i];
        if v > prev {
            count += 1;
        }
        prev = v;
    }

    count
}
