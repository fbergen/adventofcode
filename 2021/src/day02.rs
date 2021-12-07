use recap::Recap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Command {
    Forward,
    Down,
    Up,
}

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r"^(?P<op>\w+) (?P<arg>\d+)$")]
pub struct Instr {
    op: Command,
    arg: usize,
}

pub fn solve_part_1(input_str: &str) -> usize {
    let instrs: Vec<Instr> = input_str
        .lines()
        .map(|l| l.parse::<Instr>().unwrap())
        .collect();

    let mut h: usize = 0;
    let mut d: usize = 0;

    for i in instrs {
        match i.op {
            Command::Forward => h += i.arg,
            Command::Down => d += i.arg,
            Command::Up => d -= i.arg,
        }
    }
    return h * d;
}

pub fn solve_part_2(input_str: &str) -> usize {
    let instrs: Vec<Instr> = input_str
        .lines()
        .map(|l| l.parse::<Instr>().unwrap())
        .collect();

    let mut h: usize = 0;
    let mut d: usize = 0;
    let mut aim: usize = 0;

    for i in instrs {
        match i.op {
            Command::Forward => {
                h += i.arg;
                d += aim * i.arg
            }
            Command::Down => aim += i.arg,
            Command::Up => aim -= i.arg,
        }
    }
    return h * d;
}

#[cfg(test)]
mod test {
    #[test]
    fn test1() {
        let res = super::solve_part_1(
            "forward 5
down 5
forward 8
up 3
down 8
forward 2",
        );
        assert_eq!(res, 150);
    }

    #[test]
    fn test2() {
        let res = super::solve_part_2(
            "forward 5
down 5
forward 8
up 3
down 8
forward 2",
        );
        assert_eq!(res, 900);
    }
}
