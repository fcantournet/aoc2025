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
        let min = recurse_simplify(&m, m.joltage.clone());

        dbg!(min);

        answer += min;
    }

    return Some(answer as u64);

    let bootstraped: Vec<(Machine, Vec<Vec<Button>>)> = machines
        .into_iter()
        .map(|m| {
            // odd numbers in joltage ~ true bits in Machine.goal.
            let new_goal: BitVec<usize> = m.joltage.iter().map(|j| *j % 2 != 0).collect();
            let mut valid = all_valid_button_presses_sets(&m, &new_goal);
            // dbg!(&valid);
            valid.sort_by_key(|v| v.len());
            (m, valid)
        })
        .collect();

    let mut shortests = Vec::new();
    for (m, bootstraps) in bootstraped {
        let mut min = usize::MAX;
        for bootstrap in bootstraps {
            let mut state = vec![0; m.joltage.len()];
            // apply bootstrap
            for b in bootstrap.iter() {
                state = fiddle_joltage(&b.index, state);
            }
            // println!(
            //     "Applied {:#?} resulting in {:?} for target {:?}",
            //     &bootstrap, &state, &m.joltage
            // );

            // Apply the bootstrap as a substraction on the target so we can then simplify further by dived until odd.
            let mut new_target: Vec<_> = m
                .joltage
                .iter()
                .zip(state)
                .map(|(joltage, state)| joltage - state)
                .collect();

            // simplify by dividing while all target elements are even (we can reach state 2*N by pressing the buttons for state N twice as many times)
            let mut factor = 1usize;
            while new_target.iter().all(|e| e % 2 == 0) {
                new_target = new_target.iter().map(|e| e / 2).collect();
                factor *= 2;
            }

            let mut states: HashSet<Vec<usize>> = HashSet::new();
            states.insert(vec![0; m.joltage.len()]);
            let mut i = 0;
            let mut found = false;
            dbg!(&factor, &new_target);
            loop {
                dbg!(states.len());
                if i > 1000 || states.len() == 0 {
                    break;
                }
                i += 1;
                let mut next_states = HashSet::new();
                for s in states.iter() {
                    for b in m.buttons.iter() {
                        let ns = fiddle_joltage(&b.index, s.clone());
                        if ns == new_target {
                            found = true;
                        }
                        if ns.iter().enumerate().all(|(i, j)| new_target[i] >= *j) {
                            next_states.insert(ns);
                        }
                    }
                }
                if found {
                    break;
                }
                states = next_states;
            }
            if found {
                let count = i * factor + bootstrap.len();
                min = min.min(count);
            }
        }
        shortests.push(min);
    }

    dbg!(&shortests);

    Some(shortests.iter().sum::<usize>() as u64)
}

fn recurse_simplify(
    m: &Machine,
    target: Vec<usize>,
    // bootstrap: Vec<Button>,
    // mut factor: usize,
) -> usize {
    // We're done
    if target.iter().sum::<usize>() == 0 {
        return 0;
    }

    // All valid bootstraps
    let oddbits: BitVec<usize> = target.iter().map(|j| *j % 2 != 0).collect();
    let mut valid = all_valid_button_presses_sets(&m, &oddbits);
    valid.sort_by_key(|e| e.len());

    let mut min = usize::MAX;
    for v in valid {
        println!("{:?}", &target);
        if let Some(mut next_target) = bootstrap(&target, &v) {
            if next_target.iter().sum::<usize>() == 0 {
                min = v.len();
                continue;
            }
            // simplify by dividing while all target elements are even (we can reach state 2*N by pressing the buttons for state N twice as many times)
            let mut next_factor = 1usize;
            while next_target.iter().all(|e| e % 2 == 0) {
                next_target = next_target.iter().map(|e| e / 2).collect();
                next_factor *= 2;
            }

            // recurse
            let count = recurse_simplify(m, next_target);
            min = min.min(v.len() + count * next_factor);
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

fn all_valid_button_presses_sets(m: &Machine, goal: &BitVec<usize>) -> Vec<Vec<Button>> {
    m.buttons
        .clone()
        .into_iter()
        .powerset()
        .filter(|buttons| validate(goal, buttons))
        .collect()
}

fn validate(goal: &BitVec<usize>, buttons: &[Button]) -> bool {
    let mut state = bitvec![usize, Lsb0; 0; goal.len()];
    for b in buttons {
        state = state ^ &b.bv;
    }
    state == *goal
}

fn fiddle_joltage(button: &[usize], mut state: Vec<usize>) -> Vec<usize> {
    for j in button {
        state[*j] += 1;
    }
    state
}

fn fiddle_joltage_twice(button: &[usize], mut state: Vec<usize>) -> Vec<usize> {
    for j in button {
        state[*j] += 2;
    }
    state
}

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

#[derive(Debug, Clone)]
struct Button {
    index: Vec<usize>,
    bv: BitVec<usize>,
}

impl Display for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", &self.index))
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
