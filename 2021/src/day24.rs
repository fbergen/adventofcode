use recap::Recap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Command {
    Inp,
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

use crate::day24::Command::*;

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r"^(?P<op>\w+) (?P<arg1>\S) ?(?P<arg2>\S+)?$")]
pub struct Instr {
    op: Command,
    arg1: char,
    arg2: Option<String>,
}

pub fn parse(input: &str) -> Vec<Instr> {
    input.lines().map(|l| l.parse::<Instr>().unwrap()).collect()
}

pub fn get_var_idx(c: char) -> usize {
    let mut b = [0; 1];
    c.encode_utf8(&mut b);
    b[0] as usize - 119
}

// Case div z  1
// z = z * 26 + inp + i[15]  # Push i[15] on the "stack"
//
//
// Case div z 26
// x = z % 26 - i[5] # Pop the stack
// z = (z / 26) # Only if x == 0; if x != 0 z will continue to grow, so setting x == 0 yields
//
// w[1] = w[0] + i[0][15]+ i[1][5]
fn get_modal(instrs: &Vec<Instr>, highest: bool) -> usize {
    let repeat = instrs.len() / 14;

    // start with highest or lowest val, for most significant digits
    let start = match highest {
        true => 9,
        false => 1,
    };
    let mut modal: [i64; 14] = [start; 14];

    let mut stack = vec![];
    for i in 0..14 {
        let div_val = arg2(&instrs[i * repeat + 4]);
        if div_val == 1 {
            // push the stack
            let add_val = arg2(&instrs[i * repeat + 15]);
            stack.push((i, add_val));
        } else {
            // div is 26, pop the stack
            let (prev_i, prev_add) = stack.pop().unwrap();
            let add_val = arg2(&instrs[i * repeat + 5]);
            modal[i] = modal[prev_i] + prev_add + add_val;
            if modal[i] > 9 {
                // decrease prev_i with the least amount possibe.
                modal[prev_i] -= modal[i] - 9;
                modal[i] = 9;
            } else if modal[i] < 1 {
                // increase prev_i with the
                modal[prev_i] += 1 - modal[i];
                modal[i] = 1;
            }
        }
    }
    modal.into_iter().reduce(|acc, x| acc * 10 + x).unwrap() as usize
}

fn arg2(instr: &Instr) -> i64 {
    instr.arg2.as_ref().unwrap().parse::<i64>().unwrap()
}

#[allow(dead_code)]
fn validate(instrs: &Vec<Instr>, model_num: &[usize; 14]) -> bool {
    if model_num.iter().any(|&d| d == 0) {
        return false;
    }
    let mut inp_count = 0;

    let mut vars: [i32; 4] = [0; 4];
    for i in instrs {
        let arg1_idx = get_var_idx(i.arg1);
        let arg2 = match &i.arg2 {
            Some(x) => match x.parse::<i32>() {
                Ok(v) => v,
                _ => vars[get_var_idx(x.chars().nth(0).unwrap())],
            },
            None => 0,
        };
        match i.op {
            Inp => {
                vars[arg1_idx] = (model_num[inp_count] % 10) as i32;
                inp_count += 1;
            }
            Add => {
                vars[arg1_idx] = vars[arg1_idx] + arg2;
            }
            Mul => {
                vars[arg1_idx] = vars[arg1_idx] * arg2;
            }
            Div => {
                if arg2 == 0 {
                    return false;
                }
                vars[arg1_idx] = vars[arg1_idx] / arg2;
            }
            Mod => {
                if arg2 <= 0 || vars[arg1_idx] < 0 {
                    return false;
                }
                vars[arg1_idx] = vars[arg1_idx] % arg2;
            }
            Eql => {
                vars[arg1_idx] = match vars[arg1_idx] == arg2 {
                    true => 1,
                    false => 0,
                }
            }
        }
    }
    vars[3] == 0
}

pub fn solve_part_1(input_str: &str) -> usize {
    let instrs = parse(input_str);
    get_modal(&instrs, true)
}

pub fn solve_part_2(input_str: &str) -> usize {
    let instrs = parse(input_str);
    get_modal(&instrs, false)
}
