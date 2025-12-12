use bitvec::prelude::*;
use itertools::Itertools;
use nom::Parser;
use nom::branch::alt;
use nom::character::complete::{self, newline, space1};
use nom::multi::{fold_many1, separated_list1};
use nom::{IResult, sequence::delimited};

use std::collections::{HashMap, HashSet};
use std::fmt::Display;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u64> {
    let machines = parse_input(input);
    // dbg!(&machines);

    let mut shortests = Vec::new();
    for m in machines {
        let state = bitvec![usize, Lsb0; 0; m.goal.len()];
        // let mut memo = HashMap::new();
        // let s = shortest_path(&m, state, None, &mut memo);

        let mut states: HashSet<BitVec<usize>> = HashSet::new();
        states.insert(state);
        let mut i = 0;
        let mut found = false;

        loop {
            i += 1;
            let mut next_states = HashSet::new();
            for s in states.iter() {
                for b in m.buttons.iter() {
                    let ns = push_button(&b.bv, s.clone());
                    if ns == m.goal {
                        found = true;
                    }
                    next_states.insert(ns);
                }
            }
            if found {
                break;
            }
            states = next_states;
        }

        shortests.push(i);
    }

    Some(shortests.iter().sum::<usize>() as u64)
}

// #[memoize(Ignore: m)]
// fn shortest_path(
//     m: &Machine,
//     state: BitVec<usize>,
//     last: Option<&BitVec<usize>>,
//     memo: &mut HashMap<BitVec<usize>, usize>,
// ) -> usize {
//     if m.goal == state {
//         return 0;
//     }
//     if let Some(val) = memo.get(&state) {
//         return *val;
//     }
//     let next_states = m
//         .buttons_as_bv
//         .iter()
//         .filter(|&b| Some(b) != last)
//         .map(|b| (b, push_button(b.clone(), state.clone())));
//     let min = next_states
//         .map(|(b, s)| 1 + shortest_path(m, s, Some(b), memo))
//         .min()
//         .unwrap();
//     memo.insert(state, min);
//     min
// }

fn push_button(button: &BitVec<usize>, state: BitVec<usize>) -> BitVec<usize> {
    state ^ button
}

// insight from part1 that we initially didn't figure out :
//  - pressing any key twice == NOOP (this bit we knew)
//  - pressing ABC == ACB == CBA : order doesn't matter
// => the entire problem space for a given goal bitvec is #(buttons)^2
// i.e: for any button we either press it (2n-1) times or 2n times (so 0 or 1).
// So for any machine we can easily compute all the button press combinations (#(buttons)^2) and check which
// give a valid goal and which is shortest (for part1)
//
// 2nd insight is that joltage targets are similar : any even number ~ false, uneven to a true
// in a new goal bitvec to calculate from joltage.
//
// So joltage => new_goal => all valid button combos for finish.
// => press those, and subtract.
// => for_each run the current algo with step_size == 2.
// This should GREATLY reduce the size of the possible states set, because
// state set size grow as a power of #(steps)
// (There is also reddit magic to divide by 2 and recurse but I couldn't be arse let's check
// if step_size == 2 doesn't work.
pub fn part_two(input: &str) -> Option<u64> {
    let machines = parse_input(input);
    let mut answer = 0;

    for m in machines {
        let mut memo: HashMap<Vec<usize>, Option<usize>> = HashMap::new();

        let min = recurse_simplify(&m, m.joltage.clone(), &mut memo, 0);

        dbg!(min);

        if let Some(val) = min {
            answer += val;
        }
    }

    return Some(answer as u64);
}

fn recurse_simplify(
    m: &Machine,
    target: Vec<usize>,
    // bootstrap: Vec<Button>,
    // mut factor: usize,
    memo: &mut HashMap<Vec<usize>, Option<usize>>,
    depth: usize,
) -> Option<usize> {
    // We're done
    if target.iter().sum::<usize>() == 0 {
        return Some(0);
    }
    if let Some(res) = memo.get(&target) {
        return *res;
    }

    // All valid bootstraps
    let oddbits: BitVec<usize> = target.iter().map(|j| *j % 2 != 0).collect();
    let mut valid = all_valid_button_presses_sets(&m, &oddbits);
    valid.sort_by_key(|e| e.0.len());

    let mut min = None;
    // println!(
    //     "{:indent$}possible combinations for bootstrap from {target:?}:",
    //     "",
    //     indent = depth * 4
    // );
    // for v in valid.iter() {
    //     println!("{:indent$}{}", "", v, indent = depth * 4);
    // }
    for v in valid {
        if let Some(mut next_target) = bootstrap(&target, &v.0) {
            // if next_target.iter().sum::<usize>() == 0 {
            //     min = min.min(Some(v.0.len()));
            //     continue;
            // }

            // simplify by dividing by 2 since all target joltages are even.
            // We can find the pattern that solves for half the values and repeat it twice.
            next_target = next_target.iter().map(|e| e / 2).collect();

            // recurse
            let res = recurse_simplify(m, next_target.clone(), memo, depth + 1);
            memo.insert(next_target.clone(), res);

            let presses = v.0.len();
            if let Some(count) = res {
                min = min.or(Some(usize::MAX)).min(Some(presses + count * 2));
            }
            // println!(
            //     "{:indent$}After {} to {:?} min: {min:?}={presses}+{res:?}*2",
            //     "",
            //     v,
            //     &next_target,
            //     indent = depth * 4
            // );
        }
    }

    min
}

fn bootstrap(target: &[usize], bootstrap: &[Button]) -> Option<Vec<usize>> {
    let mut sub = vec![0; target.len()];
    let mut res = vec![0; target.len()];
    for button in bootstrap {
        for i in button.index.iter() {
            sub[*i] += 1;
        }
    }
    for (i, v) in target.iter().enumerate() {
        if *v < sub[i] {
            return None;
        }
        res[i] = v - sub[i];
    }
    return Some(res);
}

fn all_valid_button_presses_sets(m: &Machine, goal: &BitVec<usize>) -> Vec<Buttons> {
    m.buttons
        .clone()
        .into_iter()
        .powerset()
        .filter(|buttons| validate(goal, buttons))
        .map(|v| Buttons { 0: v })
        .collect()
}

fn validate(goal: &BitVec<usize>, buttons: &[Button]) -> bool {
    let mut state = bitvec![usize, Lsb0; 0; goal.len()];
    for b in buttons {
        state = state ^ &b.bv;
    }
    state == *goal
}

// fn fiddle_joltage(button: &[usize], mut state: Vec<usize>) -> Vec<usize> {
//     for j in button {
//         state[*j] += 1;
//     }
//     state
// }

// fn fiddle_joltage_twice(button: &[usize], mut state: Vec<usize>) -> Vec<usize> {
//     for j in button {
//         state[*j] += 2;
//     }
//     state
// }

fn parse_input(input: &str) -> Vec<Machine> {
    separated_list1(newline, machine).parse(input).unwrap().1
}

fn machine(input: &str) -> IResult<&str, Machine> {
    let (input, goal) = goal(input)?;
    let (input, _) = space1(input)?;
    let (input, buttons) = separated_list1(space1, button).parse(input)?;
    let (input, _) = space1(input)?;
    let (rest, joltage) = joltage(input)?;

    let size = goal.len();
    let buttons = buttons
        .iter()
        .map(|b| {
            let mut bits = bitvec![usize, Lsb0; 0; size];
            for flip in b {
                bits.set(*flip, true);
            }
            Button {
                index: b.clone(),
                bv: bits,
            }
        })
        .collect();

    Ok((
        rest,
        Machine {
            goal,
            buttons,
            joltage,
        },
    ))
}

fn goal(input: &str) -> IResult<&str, BitVec> {
    delimited(
        complete::char('['),
        fold_many1(
            alt((complete::char('#'), complete::char('.'))),
            BitVec::<usize>::new,
            |mut acc, c| {
                acc.push(match c {
                    '.' => false,
                    '#' => true,
                    _ => unreachable!(),
                });
                acc
            },
        ),
        complete::char(']'),
    )
    .parse(input)
}
fn button(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(
        complete::char('('),
        separated_list1(complete::char(','), complete::usize),
        complete::char(')'),
    )
    .parse(input)
}

fn joltage(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(
        complete::char('{'),
        separated_list1(complete::char(','), complete::usize),
        complete::char('}'),
    )
    .parse(input)
}

#[derive(Debug, Clone)]
struct Machine {
    goal: BitVec,
    buttons: Vec<Button>,
    joltage: Vec<usize>,
}

#[derive(Clone)]
struct Button {
    index: Vec<usize>,
    bv: BitVec<usize>,
}

impl std::fmt::Debug for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

struct Buttons(Vec<Button>);

impl Display for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", &self.index))
    }
}

impl Display for Buttons {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let _ = write!(f, "[ ");
        for b in self.0.iter() {
            let _ = write!(f, "{}", b);
        }
        write!(f, " ]")
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }

    #[test]
    fn failing_zero() {
        // min: None=4+Some(0)*2
        let mut min: Option<usize> = None;
        let presses = 4;
        let res = Some(0);
        if let Some(count) = res {
            min = min.or(Some(usize::MAX)).min(Some(presses + count * 2));
        }

        assert_eq!(min, Some(4));

        let s = "[.##...] (0,1,5) (0,2,3,5) (0,3,4) (1,2) (0,1,2,4) {38,30,29,22,13,25}";
        let result = part_two(s);
        assert_eq!(result, Some(52));
    }

    // state  :  0 1 0 1 1
    // button :  0 1 1 1 1
    // result :  0 0 1 0 0
    //
    #[test]
    fn test_xor() {
        let mut state = bitvec![usize, Lsb0; 0; 5];
        let mut button = bitvec![usize, Lsb0; 0; 5];

        button.set(1, true);
        button.set(2, true);
        button.set(3, true);
        button.set(4, true);

        state.set(1, true);
        state.set(3, true);
        state.set(4, true);

        let res = state.clone() ^ button.clone();
        // dbg!(&state, &button, &res);
        let res = res ^ button;

        assert_eq!(res, state);
    }

    // fn test_cannot_go_negative() {
    //     let valid = all_valid_button_presses_sets(m, goal)
    // }
}
