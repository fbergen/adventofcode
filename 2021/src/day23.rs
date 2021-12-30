use hashbrown::HashMap;
use hashbrown::HashSet;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

use aoc2021::get_neighbours_4;

type Index = (i32, i32);
type Val = char;
type Game = HashMap<Index, Val>;
type State = Vec<(Index, Val)>;

const STRING_ADD: &str = "\
###D#C#B#A#
###D#B#A#C#";

pub fn parse(input_str: &str, part2: bool) -> (Game, State) {
    let mut s: String = input_str.to_string();
    if part2 {
        s = format!("{}\n{}{}", &s[0..41], STRING_ADD, &s[41..]);
    }

    let mut game: Game = s
        .lines()
        .enumerate()
        .map(|(row, s)| {
            s.chars()
                .enumerate()
                .map(|(col, c)| ((col as i32, row as i32), c))
                .collect::<Vec<(Index, Val)>>()
        })
        .flatten()
        .collect();

    let state = game
        .iter()
        .filter(|(_p, v)| ['A', 'B', 'C', 'D'].contains(*v))
        .map(|(p, v)| (*p, *v))
        .collect();

    game.values_mut().for_each(|v| {
        if ['A', 'B', 'C', 'D'].contains(v) {
            *v = '.';
        }
    });

    (game, state)
}

fn all_moves(state: &State, game: &Game, pos: &Index) -> Vec<Index> {
    let mut queue = vec![*pos];
    let mut seen = HashSet::new();
    seen.insert(*pos);
    while let Some(next) = queue.pop() {
        get_neighbours_4(&next)
            .filter(|p| *game.get(p).unwrap_or(&'#') == '.' && state.iter().all(|(sp, _)| sp != p))
            .for_each(|n| {
                if seen.insert(n) {
                    queue.push(n)
                }
            });
    }
    seen.remove(pos);
    seen.drain().collect()
}

fn possible_moves(idx: usize, state: &State, game: &Game) -> Vec<Index> {
    let (pos, val) = state[idx];
    if is_done(&state[idx], state, game) {
        return vec![];
    }
    let mut moves = all_moves(state, game, &pos);

    if pos.1 == 1 {
        // in corridor, can only go back to a done position
        moves.retain(|&p| is_done(&(p, val), state, game));
        return moves;
    }

    // Not in corridor, add corridor positions
    moves.retain(|p| p.1 == 1 && ![3, 5, 7, 9].contains(&p.0));
    return moves;
}

fn make_move(idx: usize, to: &Index, state: &mut State) {
    state[idx] = (*to, state[idx].1);
}

fn get_cost_for_move(from: &Index, to: &Index, c: char) -> usize {
    (((from.0 - to.0).abs() + (from.1 - to.1).abs())
        * match c {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1000,
            _ => panic!(),
        }) as usize
}

fn next_states(game: &Game, state: &State) -> Vec<(usize, State)> {
    let mut ret = vec![];

    for i in (0..state.len()).rev() {
        let moves = possible_moves(i, state, game);
        for m in &moves {
            let mut new_state = state.clone();
            make_move(i, m, &mut new_state);
            if state[i].0 .1 == 1 {
                return vec![(get_cost_for_move(&state[i].0, m, state[i].1), new_state)];
            }
            ret.push((get_cost_for_move(&state[i].0, m, state[i].1), new_state));
        }
    }
    return ret;
}

fn is_finished(s: &State, g: &Game) -> bool {
    for i in 0..s.len() {
        if !is_done(&s[i], s, g) {
            return false;
        }
    }
    true
}

fn is_done(player: &(Index, Val), s: &State, g: &Game) -> bool {
    let (p, v) = *player;

    let target_col = match v {
        'A' => 3,
        'B' => 5,
        'C' => 7,
        'D' => 9,
        _ => panic!(),
    };

    p.0 == target_col
        && ((p.1 + 1)..6)
            .take_while(|y| *g.get(&(target_col, *y)).unwrap() == '.')
            .all(|y| s.iter().any(|(sp, sv)| *sp == (target_col, y) && *sv == v))
}

fn dijkstra(game: &Game, state: &State) -> usize {
    let mut min_cost: HashMap<State, usize> = HashMap::new();
    let mut heap = BinaryHeap::new();

    min_cost.insert(state.clone(), 0);
    heap.push((Reverse(0), state.clone()));

    while let Some((Reverse(cost), curr_state)) = heap.pop() {
        if is_finished(&curr_state, game) {
            return cost;
        }

        for (n_cost, n_state) in next_states(game, &curr_state) {
            let n_cost = cost + n_cost;

            if n_cost < *min_cost.get(&n_state).unwrap_or(&usize::MAX) {
                heap.push((Reverse(n_cost), n_state.clone()));
                min_cost.insert(n_state.clone(), n_cost);
            }
        }
    }
    0
}

#[allow(dead_code)]
fn print_game(a: &Game, state: &State) {
    let state_map: HashMap<Index, Val> = state.iter().map(|(p, v)| (*p, *v)).collect();
    let minx = *a.iter().map(|((x, _y), _v)| x).min().unwrap();
    let maxx = *a.iter().map(|((x, _y), _v)| x).max().unwrap();
    let miny = *a.iter().map(|((_x, y), _v)| y).min().unwrap();
    let maxy = *a.iter().map(|((_x, y), _v)| y).max().unwrap();
    for y in miny..=maxy {
        for x in minx..=maxx {
            match (a.get(&(x, y)), state_map.get(&(x, y))) {
                (_, Some(v)) => print!("{}", v),
                (Some(v), _) => print!("{}", v),
                _ => print!("."),
            }
        }
        println!("");
    }
}

pub fn solve_part_1(input_str: &str) -> usize {
    let (game, state) = parse(input_str, false);
    dijkstra(&game, &state)
}

pub fn solve_part_2(input_str: &str) -> usize {
    let (game, state) = parse(input_str, true);
    dijkstra(&game, &state)
}

#[cfg(test)]
mod test {

    const TESTCASE: &str = "\
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

    #[test]
    fn test1() {
        let res = super::solve_part_1(TESTCASE);
        assert_eq!(res, 12521);
    }

    #[test]
    fn test2() {
        let res = super::solve_part_2(TESTCASE);
        assert_eq!(res, 44169);
    }
}
