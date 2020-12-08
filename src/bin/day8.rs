use recap::Recap;
use serde::Deserialize;

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r"^(?P<op>\w+) (?P<arg>[+-]\d+)$")]
pub struct Instr {
    op: String,
    arg: i32,
}

#[derive(Debug)]
pub struct Program<'a> {
    pc: usize,
    acc: i32,
    instr: &'a Vec<Instr>,
    visited: Vec<bool>,
    has_swapped: bool,
}

impl Iterator for Program<'_> {
    // (finished, last_acc)
    type Item = (bool, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let curr_pc = self.pc;

        // End of program
        if curr_pc >= self.instr.len() {
            return None;
        }
        let i = &self.instr[curr_pc];

        if self.visited[curr_pc] {
            // Infinite loop
            return None;
        }
        self.visited[curr_pc] = true;

        let new_pc = match i.op.as_str() {
            "nop" => curr_pc + 1,
            "acc" => {
                self.acc += i.arg;
                curr_pc + 1
            }
            "jmp" => (curr_pc as i32 + i.arg) as usize,
            _ => panic!("Panic!"),
        };

        self.pc = new_pc;

        // Try to see if we should "swap" the current instruction.
        if !self.has_swapped {
            let alt_pc = match i.op.as_str() {
                "jmp" => Some(curr_pc + 1),
                "nop" => Some((curr_pc as i32 + i.arg) as usize),
                _ => None,
            };

            if let Some(a) = alt_pc {
                if a >= self.instr.len() {
                    return Some((true, self.acc));
                }
                // Swap the jmp and nop instuction, evalutate the new program.
                let alt_prg = Program {
                    pc: a,
                    acc: self.acc,
                    instr: self.instr,
                    visited: self.visited.clone(),
                    has_swapped: true,
                }
                .into_iter()
                .last()
                .unwrap_or((false, 0));
                // It finished, not in an infinite loop
                // Update main programs PC and mark it as swapped
                if alt_prg.0 == true {
                    self.has_swapped = true;
                    self.pc = a;
                }
            }
        }

        Some((self.pc >= self.instr.len(), self.acc))
    }
}

fn main() {
    let input: &str = include_str!("../inputs/day8");
    println!("part1: {}", solve(input, true));
    println!("part2: {}", solve(input, false));
}

fn solve(input_str: &str, part1: bool) -> i32 {
    let input: Vec<Instr> = input_str
        .lines()
        .map(|p| p.parse::<Instr>().unwrap())
        .collect();

    let p = Program {
        pc: 0,
        acc: 0,
        visited: {
            let mut v = Vec::<bool>::new();
            v.resize(input.len(), false);
            v
        },
        instr: &input,
        has_swapped: part1,
    };

    p.into_iter().last().unwrap().1
}

#[cfg(test)]
mod test {
    use super::*;

    const TESTCASE: &str = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn test_day8() {
        assert_eq!(solve(TESTCASE, true), 5);
        assert_eq!(solve(TESTCASE, false), 8);
    }
}
