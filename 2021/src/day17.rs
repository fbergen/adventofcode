use recap::Recap;
use serde::Deserialize;
use std::cmp::{max, min};

#[derive(Debug, Deserialize, Recap)]
#[recap(
    regex = r"^target area: x=(?P<x0>\d+)\.\.(?P<x1>\d+), y=(?P<y0>-?\d+)\.\.(?P<y1>-?\d+)\n?$"
)]
struct TargetArea {
    x0: i32,
    x1: i32,
    y0: i32,
    y1: i32,
}

pub fn solve_part_1(input_str: &str) -> i32 {
    let ta = input_str.parse::<TargetArea>().unwrap();
    ta.y0 * (ta.y0 + 1) / 2
}

// Return the number of steps (iterations) (min, max) when the initial x velocity hits the target
fn hit_steps_x(xv: i32, t: &TargetArea) -> Option<(i32, i32)> {
    if xv > t.x1 || xv < 1 {
        return None;
    }

    let x_coef = (2 * xv + 1) as f64 / 2 as f64;
    let min = (x_coef - (x_coef.powf(2.0) - t.x0 as f64 * 2.0).sqrt()).ceil() as i32;
    let max = (x_coef - (x_coef.powf(2.0) - t.x1 as f64 * 2.0).sqrt()).floor() as i32;

    if min > 0 && max == 0 {
        Some((min, i32::MAX))
    } else {
        Some((min, max))
    }
}

fn hit_steps_y(yv: i32, t: &TargetArea) -> Option<(i32, i32)> {
    if yv < t.y0 || yv >= t.y0.abs() {
        return None;
    }

    let y_coef = (2 * yv + 1) as f64 / 2 as f64;
    let min = (y_coef + (y_coef.powf(2.0) - t.y1 as f64 * 2.0).sqrt()).ceil() as i32;
    let max = (y_coef + (y_coef.powf(2.0) - t.y0 as f64 * 2.0).sqrt()).floor() as i32;

    if min == 0 || max == 0 || min > max {
        None
    } else {
        Some((min, max))
    }
}

// For each x and y-velocity, compute the number of steps (range) at which the projectile is within
// the Target area.
fn build_step_vecs(t: &TargetArea) -> (Vec<(i32, i32)>, Vec<(i32, i32)>) {
    let xv_min = (((t.x0 * 8 + 1) as f64).sqrt() / 2.0 - 0.5).ceil() as i32;
    let yv_max = (t.y0 + 1).abs();
    let xvs = (xv_min..=t.x1).filter_map(|i| hit_steps_x(i, &t)).collect();
    let yvs = (t.y0..=yv_max).filter_map(|i| hit_steps_y(i, &t)).collect();
    (xvs, yvs)
}

pub fn solve_part_2(input_str: &str) -> usize {
    let t = input_str.parse::<TargetArea>().unwrap();

    let (xvs, yvs) = build_step_vecs(&t);

    // count xs and ys that are "overlapping" in number of steps.
    let mut num = 0;
    for xv in &xvs {
        for yv in &yvs {
            let max_start = max(xv.0, yv.0);
            let min_end = min(xv.1, yv.1);
            if max_start <= min_end {
                num += 1;
            }
        }
    }
    num
}

#[cfg(test)]
mod test {

    const TESTCASE: &str = "\
target area: x=20..30, y=-10..-5";

    #[test]
    fn test1() {
        let res = super::solve_part_1(TESTCASE);
        assert_eq!(res, 45);
    }

    #[test]
    fn test2() {
        let res = super::solve_part_2(TESTCASE);
        assert_eq!(res, 112);
    }
}
