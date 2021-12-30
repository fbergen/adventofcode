type Prog = Vec<u64>;

fn parse_inp(input: &str) -> Prog {
    let to_bin = |c: char| format!("{:0>4b}", c.to_digit(16).unwrap());
    let mut s: String = String::new();
    for i in input.lines().next().unwrap().chars() {
        s.push_str(&to_bin(i));
    }
    s.chars().map(|c| c.to_digit(2).unwrap() as u64).collect()
}

fn to_num<'a>(v: impl Iterator<Item = u64>) -> u64 {
    v.fold(0, |acc, x| acc * 2 + x)
}

fn parse_packet(iter: &mut std::vec::IntoIter<u64>, part2: bool) -> (u64, u64, u64) {
    let mut consumed = 0;
    let num;
    let mut version = to_num(iter.by_ref().take(3));
    let type_id = to_num(iter.by_ref().take(3));
    consumed += 6;
    if type_id == 4 {
        let mut val: Vec<u64> = vec![];
        while to_num(iter.by_ref().take(1)) == 1 {
            val.extend(iter.by_ref().take(4).map(|c| c));
            consumed += 5;
        }
        val.extend(iter.by_ref().take(4).map(|c| c));
        consumed += 5;
        num = to_num(val.into_iter());
    } else {
        // operator
        let mut rets = vec![];
        if to_num(iter.by_ref().take(1)) == 0 {
            let mut sub_len = to_num(iter.by_ref().take(15));
            consumed += 16 + sub_len;
            while sub_len > 0 {
                let (cons, vers, ret) = parse_packet(iter.by_ref(), part2);
                rets.push(ret);
                version += vers;
                sub_len -= cons;
            }
        } else {
            let num_sub = to_num(iter.by_ref().take(11));
            consumed += 12;
            for _ in 0..num_sub {
                let (cons, vers, ret) = parse_packet(iter.by_ref(), part2);
                rets.push(ret);
                version += vers;
                consumed += cons;
            }
        };
        num = match type_id {
            0 => rets.iter().sum(),
            1 => rets.iter().product(),
            2 => *rets.iter().min().unwrap(),
            3 => *rets.iter().max().unwrap(),
            5 => (rets[0] > rets[1]) as u64,
            6 => (rets[0] < rets[1]) as u64,
            7 => (rets[0] == rets[1]) as u64,
            _ => panic!(),
        };
    }
    (consumed, version, num)
}

pub fn solve_part_1(input_str: &str) -> u64 {
    let prog = parse_inp(input_str);
    parse_packet(&mut prog.into_iter(), false).1
}

pub fn solve_part_2(input_str: &str) -> u64 {
    let prog = parse_inp(input_str);
    parse_packet(&mut prog.into_iter(), true).2
}

#[cfg(test)]
mod test {

    #[test]
    fn test1() {
        assert_eq!(super::solve_part_1("8A004A801A8002F478"), 16);
        assert_eq!(super::solve_part_1("620080001611562C8802118E34"), 12);
        assert_eq!(super::solve_part_1("C0015000016115A2E0802F182340"), 23);
        assert_eq!(super::solve_part_1("A0016C880162017C3686B18A3D4780"), 31);
    }

    #[test]
    fn test2() {
        assert_eq!(super::solve_part_2("C200B40A82"), 3);
        assert_eq!(super::solve_part_2("04005AC33890"), 54);
        assert_eq!(super::solve_part_2("880086C3E88112"), 7);
        assert_eq!(super::solve_part_2("CE00C43D881120"), 9);
        assert_eq!(super::solve_part_2("D8005AC2A8F0"), 1);
        assert_eq!(super::solve_part_2("F600BC2D8F"), 0);
        assert_eq!(super::solve_part_2("9C005AC2F8F0"), 0);
        assert_eq!(super::solve_part_2("9C0141080250320F1802104A08"), 1);
    }
}
