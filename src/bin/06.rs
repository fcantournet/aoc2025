use std::fmt;
use std::str::FromStr;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let mut problems = parse_input1(input);
    problems.compute();
    Some(problems.sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let nlines = input.lines().count();
    let ncols = input.lines().next().unwrap().chars().count();
    let operators: Vec<Operator> = input
        .lines()
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .rev()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut columns: Vec<u64> = Vec::new();
    let mut lines = input.lines();
    let mut line_its: Vec<_> = Vec::new();
    for _ in 0..nlines - 1 {
        line_its.push(lines.next().unwrap().chars().rev());
    }

    for _ in 0..ncols {
        let mut col = 0u64;
        for it in line_its.iter_mut() {
            let c = it.next().unwrap();
            match c {
                '1'..='9' => col = col * 10 + (c as u64 - '0' as u64),
                ' ' => continue,
                _ => unreachable!(),
            }
        }
        columns.push(col);
    }

    // dbg!(columns[columns.len() - 1]);
    // dbg!(columns[columns.len() - 2]);

    let mut problems = Problems {
        operator: operators.clone(),
        numbers: Vec::new(),
        results: Vec::new(),
    };

    // group colums
    let mut nums = Vec::new();
    for c in columns {
        if c == 0 {
            problems.numbers.push(nums);
            nums = Vec::new();
        } else {
            nums.push(c);
        }
    }
    problems.numbers.push(nums);

    problems.compute();
    Some(problems.sum())
}

// fn u64frombytes(chars: &[char]) -> u64 {
//     let mut res = 0;
//     for c in chars {
//         match c {
//             48..=57 => res = res * 10 + (b - 48) as u64,
//             32 => continue,
//             _ => unreachable!(),
//         };
//     }
//     return res;
// }

fn parse_input1(input: &str) -> Problems {
    let n = input.lines().count();
    let mut lines = input.lines();
    let mut problems = Problems {
        operator: Vec::new(),
        numbers: Vec::new(),
        results: Vec::new(),
    };

    let mut line_nums: Vec<Vec<u64>> = Vec::new();
    for _ in 1..n {
        let nums: Vec<u64> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        line_nums.push(nums);
    }
    problems.operator = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    problems.numbers.resize(problems.operator.len(), Vec::new());
    for (i, _) in problems.operator.iter().enumerate() {
        for l in line_nums.iter() {
            problems.numbers[i].push(l[i]);
        }
    }

    problems
}

struct Problems {
    operator: Vec<Operator>,
    numbers: Vec<Vec<u64>>,
    results: Vec<u64>,
}

impl Problems {
    fn sum(&self) -> u64 {
        self.results.iter().sum()
    }

    fn compute(&mut self) {
        assert_eq!(self.numbers.len(), self.operator.len());
        let n = self.operator.len();
        self.results.resize(n, 0u64);
        for (i, op) in self.operator.iter().enumerate() {
            let res: u64 = match op {
                Operator::Add => self.numbers[i].iter().sum(),
                Operator::Mul => self.numbers[i].iter().product(),
            };
            self.results[i] = res;
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Mul,
}

#[derive(Debug)]
struct ParseOperatorError {
    read: String,
}

impl fmt::Display for ParseOperatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid input {}", self.read)
    }
}

impl FromStr for Operator {
    type Err = ParseOperatorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "*" => Ok(Operator::Mul),
            "+" => Ok(Operator::Add),
            _ => Err(ParseOperatorError { read: s.into() }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
