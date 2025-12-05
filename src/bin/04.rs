use std::collections::HashSet;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let map = parse_input(input);
    let total = map
        .iter()
        .map(|n| neighbourgs(&map, n))
        .filter(|&n| n < 4)
        .count();
    Some(total as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = parse_input(input);
    let removed = remove(&map);

    Some(removed as u64)
}

fn remove(map: &HashSet<(isize, isize)>) -> usize {
    let mut clone = map.clone();
    clone.retain(|roll| neighbourgs(map, roll) > 3);
    let removed = map.len() - clone.len();
    if removed == 0 {
        return removed;
    }
    return removed + remove(&clone);
}

const NEIGHBOURS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn neighbourgs(map: &HashSet<(isize, isize)>, start: &(isize, isize)) -> usize {
    let mut n = 0;
    for (x, y) in NEIGHBOURS {
        if map.contains(&(start.0 + x, start.1 + y)) {
            n += 1;
        }
    }
    n
}

fn parse_input(input: &str) -> HashSet<(isize, isize)> {
    let mut map = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '@' {
                map.insert((x as isize, y as isize));
            }
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
