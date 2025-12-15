use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let devices = parse_input(input);
    let mut visited: HashMap<String, usize> = HashMap::new();

    let total = recursive_walk(&devices, "you", "out", "inexistant", &mut visited);

    Some(total as u64)
}

fn recursive_walk(
    devices: &HashMap<String, Vec<String>>,
    start: &str,
    end: &str,
    excluding: &str,
    visited: &mut HashMap<String, usize>,
) -> usize {
    if start == end {
        return 1;
    }
    if let Some(counted) = visited.get(start) {
        return *counted;
    }
    // println!("visiting {} for the first time", start);
    if let Some(outs) = devices.get(start) {
        let mut path_leading_to_out = 0usize;
        for out in outs {
            if out == excluding {
                continue;
            }
            let new_paths = recursive_walk(devices, out, end, excluding, visited);
            path_leading_to_out += new_paths;
        }
        println!("{path_leading_to_out} paths from {start} to {end}");
        visited.insert(start.to_string(), path_leading_to_out);
        return path_leading_to_out;
    }
    0
}

pub fn part_two(input: &str) -> Option<u64> {
    let devices = parse_input(input);
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
        let mut visited: HashMap<String, usize> = HashMap::new();
        recursive_walk(&devices, start, end, excluding, &mut visited)
    })
    .collect();

    dbg!(&path);
    Some((path[0] * path[5] * path[2] + path[1] * path[4] * path[3]) as u64)
}

fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    input
        .lines()
        .map(|line| {
            let (dev, outs) = line.split_once(":").unwrap();
            let outs = outs
                .trim()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            (dev.into(), outs)
        })
        .collect()
}

struct Device {
    out: Vec<String>,
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
}
