use hashbrown::HashSet;

type DotMatrix = HashSet<(i32, i32)>;

fn parse(input_str: &str) -> (DotMatrix, Vec<(char, i32)>, i32, i32) {
    let mut xsize = 0;
    let mut ysize = 0;
    let (coords, folds) = input_str.split_once("\n\n").unwrap();
    let matrix: DotMatrix = coords
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            let (xx, yy) = (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap());
            if xx > xsize {
                xsize = xx;
            }
            if yy > ysize {
                ysize = yy;
            }
            (xx, yy)
        })
        .collect();

    let fold_vec: Vec<(char, i32)> = folds
        .lines()
        .map(|l| {
            let instr = l.rsplit_once(" ").unwrap().1;
            let (dim, num) = instr.split_once("=").unwrap();
            (dim.chars().next().unwrap(), num.parse::<i32>().unwrap())
        })
        .collect();

    (matrix, fold_vec, xsize + 1, ysize + 1)
}

fn fold(m: &mut DotMatrix, xsize: &mut i32, ysize: &mut i32, fold: (char, i32)) {
    if fold.0 == 'y' {
        let to_add: Vec<(i32, i32)> = m
            .iter()
            .filter(|(_x, y)| y > &fold.1)
            .map(|(x, y)| (*x, fold.1 - (y - fold.1)))
            .collect();
        m.extend(to_add);
        m.retain(|(_x, y)| y < &fold.1);

        *ysize = fold.1;
    } else {
        let to_add: Vec<(i32, i32)> = m
            .iter()
            .filter(|(x, _y)| x > &fold.1)
            .map(|(x, y)| (fold.1 - (x - fold.1), *y))
            .collect();
        m.extend(to_add);
        m.retain(|(x, _y)| x < &fold.1);

        *xsize = fold.1;
    }
}

pub fn solve_part_1(input_str: &str) -> usize {
    let (mut matrix, folds, mut xsize, mut ysize) = parse(input_str);
    fold(&mut matrix, &mut xsize, &mut ysize, *folds.first().unwrap());

    matrix.len()
}
pub fn solve_part_2(input_str: &str) -> usize {
    let (mut matrix, folds, mut xsize, mut ysize) = parse(input_str);
    folds.iter().for_each(|f| {
        fold(&mut matrix, &mut xsize, &mut ysize, *f);
    });

    let mut res: Vec<Vec<char>> = vec![vec!['.'; xsize as usize]; ysize as usize];
    matrix.iter().for_each(|(x, y)| {
        if x < &xsize && y < &ysize {
            res[*y as usize][*x as usize] = '#'
        }
    });
    res.iter().for_each(|row| {
        println!("{:?}", row);
    });

    0
}

#[cfg(test)]
mod test {

    const TESTCASE: &str = "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn test1() {
        let res = super::solve_part_1(TESTCASE);
        assert_eq!(res, 17);
    }

    #[test]
    fn test2() {
        let res = super::solve_part_2(TESTCASE);
        assert_eq!(res, 0);
    }
}
