use bitvec::{index, prelude::*};
use itertools::enumerate;
use nom::Parser;
use nom::branch::alt;
use nom::character::complete::{self, newline, space1};
use nom::multi::{fold_many1, separated_list1};
use nom::{IResult, multi::many1, sequence::delimited};
use std::ops::BitXor;

use std::collections::{HashMap, HashSet};

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
                for b in m.buttons_as_bv.iter() {
                    let ns = push_button(b.clone(), s.clone());
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
fn shortest_path(
    m: &Machine,
    state: BitVec<usize>,
    last: Option<&BitVec<usize>>,
    memo: &mut HashMap<BitVec<usize>, usize>,
) -> usize {
    if m.goal == state {
        return 0;
    }
    if let Some(val) = memo.get(&state) {
        return *val;
    }
    let next_states = m
        .buttons_as_bv
        .iter()
        .filter(|&b| Some(b) != last)
        .map(|b| (b, push_button(b.clone(), state.clone())));
    let min = next_states
        .map(|(b, s)| 1 + shortest_path(m, s, Some(b), memo))
        .min()
        .unwrap();
    memo.insert(state, min);
    min
}

fn push_button(button: BitVec<usize>, state: BitVec<usize>) -> BitVec<usize> {
    state ^ button
}

pub fn part_two(input: &str) -> Option<u64> {
    return None;
    let machines = parse_input(input);

    let mut shortests = Vec::new();
    for m in machines {
        let state = vec![0; m.joltage.len()];

        let mut states: HashSet<Vec<usize>> = HashSet::new();
        states.insert(state);
        let mut i = 0;
        let mut found = false;

        loop {
            // dbg!(states.len());
            if i > 10000 || states.len() == 0 {
                unreachable!("infinite loop ?")
            }
            i += 1;
            let mut next_states = HashSet::new();
            for s in states.iter() {
                for b in m.buttons.iter() {
                    let ns = fiddle_joltage(b, s.clone());
                    if ns == m.joltage {
                        found = true;
                    }
                    if ns.iter().enumerate().all(|(i, j)| m.joltage[i] >= *j) {
                        next_states.insert(ns);
                    }
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

fn fiddle_joltage(button: &[usize], mut state: Vec<usize>) -> Vec<usize> {
    for j in button {
        state[*j] += 1;
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
    let buttons_as_bv = buttons
        .iter()
        .map(|b| {
            let mut bits = bitvec![usize, Lsb0; 0; size];
            for flip in b {
                bits.set(*flip, true);
            }
            bits
        })
        .collect();

    Ok((
        rest,
        Machine {
            goal,
            buttons,
            buttons_as_bv,
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
    buttons: Vec<Vec<usize>>,
    buttons_as_bv: Vec<BitVec<usize>>,
    joltage: Vec<usize>,
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
}
