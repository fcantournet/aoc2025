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

    let mut memo: HashMap<Vec<usize>, Option<usize>> = HashMap::new();
    for m in machines {
        memo.clear();
        let combinations = all_button_combinations_with_cost(&m);
        // dbg!(&m, &combinations);
        let min = recurse_simplify(&combinations, m.joltage.clone(), &mut memo, 0);

        dbg!(min);

        if let Some(val) = min {
            answer += val;
        }
    }

    return Some(answer as u64);
}

fn recurse_simplify(
    combinations: &[Combination],
    target: Vec<usize>,
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
    // println!("{:indent$} Solving for {target:?}", "", indent = depth * 4);

    // All valid bootstraps
    // let oddbits: BitVec<usize> = target.iter().map(|j| *j % 2 != 0).collect();
    //
    let oddbits = SimpleBitVec::new(&target);

    let mut min = None;
    for c in combinations {
        // Apply combination if valid and simplify by dividing by 2 since all target joltages will be even.
        // We can find the pattern that solves for half the values and repeat it twice.
        if let Some(next_target) = c.apply_if_valid(&target, &oddbits) {
            // recurse
            // println!(
            //     "{:indent$} got {next_target:?} using {c:?}",
            //     "",
            //     indent = depth * 4
            // );
            let res = recurse_simplify(combinations, next_target.clone(), memo, depth + 1);
            memo.insert(next_target.clone(), res);

            if let Some(count) = res {
                let candidate = c.buttons.len() + count * 2;
                min = min.or(Some(usize::MAX)).min(Some(candidate));
                // println!(
                //     "{:indent$} Solved {next_target:?} in {res:?}. {min:?} <= {candidate}={res:?}*2+{}",
                //     "",
                //     c.buttons.len(),
                //     indent = depth * 4
                // );
            }
        }
    }

    min
}

fn all_button_combinations_with_cost(m: &Machine) -> Vec<Combination> {
    m.buttons
        .clone()
        .into_iter()
        .powerset()
        .map(|v| Combination::new(v, m.joltage.len()))
        .collect()
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
            let mut values = vec![0; size];
            for flip in b {
                bits.set(*flip, true);
                values[*flip] = 1;
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

// The result of applying a combination of Buttons
#[derive(Debug, Clone)]
struct Combination {
    result: Vec<usize>,
    buttons: Vec<Button>,
    resulting_pattern: SimpleBitVec,
}

impl Combination {
    fn new(buttons: Vec<Button>, num_joltages: usize) -> Self {
        let mut result = vec![0; num_joltages];
        for b in buttons.iter() {
            for flip in b.index.iter() {
                result[*flip] += 1;
            }
        }
        let resulting_pattern = SimpleBitVec::new(&result);
        Self {
            result,
            buttons,
            resulting_pattern,
        }
    }

    fn apply_if_valid(&self, target: &[usize], oddbits: &SimpleBitVec) -> Option<Vec<usize>> {
        // this combination doesn't validate the target resulting parity pattern.
        if oddbits.0 != self.resulting_pattern.0 {
            return None;
        }
        let mut res = vec![0; target.len()];
        for (i, v) in target.iter().enumerate() {
            if *v < self.result[i] {
                return None;
            }
            res[i] = (v - self.result[i]) / 2;
        }
        return Some(res);
    }
}

#[derive(Debug, Clone, Copy)]
struct SimpleBitVec(usize);

impl SimpleBitVec {
    // takes a vec of values and return a bitvec if parity of those values.
    // This can be used to validate that applying a bunch of buttons gives the expected
    // bit parity result.
    fn new(values: &[usize]) -> Self {
        // rev because the values are in order of most significant bit
        SimpleBitVec(
            values
                .iter()
                .rev()
                .enumerate()
                .map(|(i, v)| if v % 2 != 0 { 1 << i } else { 0 })
                .sum(),
        )
    }
}

impl std::fmt::Debug for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
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
}
