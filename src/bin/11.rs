use std::{collections::HashMap, usize};

advent_of_code::solution!(11);

const ALPHA_BASE: usize = 26;
const UNIVERSE: usize = ALPHA_BASE * ALPHA_BASE * ALPHA_BASE;

pub fn part_one(input: &str) -> Option<u64> {
    let mut map = [const { Vec::new() }; UNIVERSE];
    parse_input(input, &mut map);
    let mut visited = [None; UNIVERSE];
    let you = name_to_id("you");
    let out = name_to_id("out");
    let inexistant = usize::MAX;
    let total = recursive_walk(&map, you, out, inexistant, &mut visited);

    Some(total as u64)
}

fn recursive_walk(
    devices: &[Vec<usize>; UNIVERSE],
    start: usize,
    end: usize,
    excluding: usize,
    visited: &mut [Option<usize>; UNIVERSE],
) -> usize {
    if start == end {
        return 1;
    }
    if let Some(counted) = visited[start] {
        return counted;
    }
    if let Some(outs) = devices.get(start) {
        let mut path_leading_to_out = 0usize;
        for out in outs {
            if *out == excluding {
                continue;
            }
            let new_paths = recursive_walk(devices, *out, end, excluding, visited);
            path_leading_to_out += new_paths;
        }
        visited[start] = Some(path_leading_to_out);
        return path_leading_to_out;
    }
    0
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map = [const { Vec::new() }; UNIVERSE];
    parse_input(input, &mut map);
    let path: Vec<_> = [
        ("svr", "dac", "fft"),
        ("svr", "fft", "dac"),
        ("fft", "out", "dac"),
        ("dac", "out", "fft"),
        ("fft", "dac", "inexistant"),
        ("dac", "fft", "inexistant"),
    ]
    .iter()
    .map(|(start, end, excluding)| {
        let mut visited = [None; UNIVERSE];
        let start = name_to_id(start);
        let end = name_to_id(end);
        let excluding = name_to_id(excluding);
        recursive_walk(&map, start, end, excluding, &mut visited)
    })
    .collect();

    dbg!(&path);
    Some((path[0] * path[5] * path[2] + path[1] * path[4] * path[3]) as u64)
}

fn parse_input(input: &str, map: &mut [Vec<usize>; UNIVERSE]) {
    for (dev, outs) in input.lines().map(|line| {
        let (dev, outs) = line.split_once(":").unwrap();
        let outs: Vec<usize> = outs
            .trim()
            .split_whitespace()
            .map(|s| name_to_id(s))
            .collect();
        (name_to_id(&dev), outs)
    }) {
        map[dev] = outs;
    }
}

fn name_to_id(name: &str) -> usize {
    // iterate over chars from most significant char and convert to base 26
    name.chars()
        .fold(0, |acc, c| acc * 26 + (c as u8 - 'a' as u8) as usize)
}

fn id_to_name(id: usize) -> String {
    let mut div = id;
    let mut res = ['0'; 3];
    for i in 0..3 {
        let rm = div % 26;
        res[i] = (rm as u8 + 'a' as u8) as char;
        dbg!(rm, div, &res);
        div = div / 26;
    }
    res.iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_encoding() {
        assert_eq!(12 / 14, 0);
        let name = "abc";
        let id = name_to_id(name);
        dbg!(id);
        assert_eq!(name, id_to_name(id));
    }
}
