use std::{collections::{HashMap, HashSet}, ops::Add};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);

    let splitters = grid.iter().filter_map(|(k, v)| {
        if v == &Tile::Splitter {
            Some(*k)
        } else {
            None
        }
    }).collect::<HashSet<_>>();

    let start = grid.iter().find(|(_, v)| *v == &Tile::Start).unwrap().0;
    let end = grid.iter().max_by_key(|(k, _)| k.y).unwrap().0;

    let mut splits = 0;
    let mut tachyons = HashSet::new();
    tachyons.insert(*start);

    let DOWN = Point{x: 0, y: 1};
    let LEFT = Point{x: -1, y: 0};
    let RIGHT = Point{x: 1, y: 0};


    dbg!(&splitters);
    for _ in 0..end.y {
        let mut next_tachyons = HashSet::new();
        dbg!(&tachyons);
        for t in tachyons {
            let tn = t + DOWN;
            if splitters.contains(&tn) {
                splits += 1;
                next_tachyons.insert(tn + LEFT);
                next_tachyons.insert(tn + RIGHT);
            } else {
                next_tachyons.insert(tn);
            }
        }
        tachyons = next_tachyons;
    }

    Some(splits as u64)
}


// There is an actual linear solution to part2 which is super fast and simpler than memo+recursive graph approach.
pub fn part_two(input: &str) -> Option<u64> {
    let mut beams: Vec<u64> = vec![0u64; 150];
    let mut next_beams = beams.clone();

    // find the start.
    let s_index = input.lines().next().unwrap().find(|c| c == 'S').unwrap();
    beams[s_index] = 1;

    // we're holding how many beams go through each column and iterate down the lines, splitting when we see '^'.
    for line in input.lines(){
        next_beams.fill(0);
        let bytes = line.as_bytes();

        for (i, b) in beams.iter().enumerate().filter(|&(_, b)| *b != 0) {
            if bytes[i] == b'^' {
                next_beams[i-1] += *b;
                next_beams[i+1] += *b;
            } else {
                next_beams[i] += *b
            }
        }
        (next_beams, beams) = (beams, next_beams);
    }

    // at the end beams holds the number of beam paths that end in each column of the last line
    // so we just sum them.
    Some(beams.iter().sum())
}

fn parse_input(input: &str) -> HashMap<Point, Tile> {
    let mut grid = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate(){
            let (k, v) = match c {
                '.' => (Point { x: x as isize, y: y as isize }, Tile::Empty),
                '^' => (Point { x: x as isize, y: y as isize }, Tile::Splitter),
                'S' => (Point { x: x as isize, y: y as isize }, Tile::Start),
                _ => panic!("Invalid input"),
            };
            grid.insert(k, v);
        }
    }
    grid
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Tile {
    #[default]
    Empty,
    Splitter,
    Start,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
