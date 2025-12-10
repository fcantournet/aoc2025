use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let boxes = parse_input(input);

    let mut distances: Vec<((Point, Point), u64)> = Vec::with_capacity(boxes.len()*boxes.len());
    for (i, &b1) in boxes.iter().enumerate() {
        for (j, &b2) in boxes.iter().enumerate().skip(i) {
            if i == j {
                continue;
            }
            distances.push(((b1, b2), b1.distance(&b2)));
        }
    }

    distances.sort_by_key(|a| a.1);

    let mut circuits: Vec<HashSet<Point>> = Vec::new();
    let mut box_to_circuits: HashMap<Point, usize> = HashMap::new();
    for ((p1, p2), _) in distances[..1000].iter() {
        let i1 = box_to_circuits.get(p1).copied();
        let i2 = box_to_circuits.get(p2).copied();
        dbg!(p1, p2, i1, i2, &circuits.len());
        match (i1, i2) {
            (None, None) => {
                let i = circuits.len();
                let c = HashSet::from([*p1,*p2]);
                circuits.push(c);
                box_to_circuits.insert(*p1, i);
                box_to_circuits.insert(*p2, i);
            },
            (None, Some(i2)) => {
                circuits[i2].insert(*p1);
                box_to_circuits.insert(*p1, i2);
            },
            (Some(i1), None) => {
                circuits[i1].insert(*p2);
                box_to_circuits.insert(*p2, i1);
            },
            (Some(i1), Some(i2)) => {
                if i1 == i2 {
                    continue;
                }
                let c2 = circuits.get(i2).unwrap().clone();
                let c1 = circuits.get_mut(i1).unwrap();
                for p in c2 {
                    c1.insert(p);
                    box_to_circuits.insert(p,i1);
                };
                circuits.get_mut(i2).unwrap().clear();
            },
        }
    }
    circuits.sort_by_key(|c| c.len());
    Some(circuits.iter().rev().take(3).fold(1, |acc, c| acc * c.len() as u64))
}

pub fn part_two(input: &str) -> Option<u64> {
    let boxes = parse_input(input);
    let num_boxes = boxes.len();

    let mut distances: Vec<((Point, Point), u64)> = Vec::with_capacity(boxes.len()*boxes.len());
    for (i, &b1) in boxes.iter().enumerate() {
        for (j, &b2) in boxes.iter().enumerate().skip(i) {
            if i == j {
                continue;
            }
            distances.push(((b1, b2), b1.distance(&b2)));
        }
    }
    distances.sort_by_key(|a| a.1);

    let (p1, p2) = connect_until_all_connected(distances, num_boxes).unwrap();
    Some(p1.x as u64 * p2.x as u64)
}

fn connect_until_all_connected(distances: Vec<((Point, Point), u64)>, num_boxes: usize) -> Option<(Point, Point)> {
    let mut circuits: Vec<HashSet<Point>> = Vec::new();
    let mut box_to_circuits: HashMap<Point, usize> = HashMap::new();
    for ((p1, p2), _) in distances.iter() {
        let i1 = box_to_circuits.get(p1).copied();
        let i2 = box_to_circuits.get(p2).copied();
        //dbg!(p1, p2, i1, i2, &circuits.len());
        match (i1, i2) {
            (None, None) => {
                let i = circuits.len();
                let c = HashSet::from([*p1,*p2]);
                circuits.push(c);
                box_to_circuits.insert(*p1, i);
                box_to_circuits.insert(*p2, i);
            },
            (None, Some(i2)) => {
                let c2 = circuits.get_mut(i2).unwrap();
                c2.insert(*p1);
                box_to_circuits.insert(*p1, i2);
                if c2.len() == num_boxes {
                    return Some((*p1, *p2));
                }

            },
            (Some(i1), None) => {
                let c1 = circuits.get_mut(i1).unwrap();
                c1.insert(*p2);
                box_to_circuits.insert(*p2, i1);
                if c1.len() == num_boxes {
                    return Some((*p1, *p2));
                }
            },
            (Some(i1), Some(i2)) => {
                if i1 == i2 {
                    continue;
                }
                let c2 = circuits.get(i2).unwrap().clone();
                let c1 = circuits.get_mut(i1).unwrap();
                for p in c2 {
                    c1.insert(p);
                    box_to_circuits.insert(p,i1);
                };
                if c1.len() == num_boxes {
                    return Some((*p1, *p2));
                }
                circuits.get_mut(i2).unwrap().clear();
            },
        }
    }

    circuits.sort_by_key(|c| c.len());

    println!("longest circuits {:?}", circuits.iter().rev().take(1).next().unwrap().len());
    None
}


fn parse_input(input: &str) -> Vec<Point> {
    input.lines().map(|line| {
        let mut it = line.splitn(3, ',');
        Point {
            x: it.next().unwrap().parse().unwrap(),
            y: it.next().unwrap().parse().unwrap(),
            z: it.next().unwrap().parse().unwrap(),
        }
    }).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn distance(&self, other: &Point) -> u64 {
        ((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)).checked_isqrt().unwrap() as u64
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
