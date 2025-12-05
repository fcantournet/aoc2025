advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let db = parse_input(input);

    Some(db.freshes().len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut db = parse_input(input);

    let merged = db.merge_ranges();
    dbg!(&merged);
    Some(merged.iter().map(|range| range.end - range.start + 1).sum())
}

fn parse_input(input: &str) -> DB {
    let (first, second) = input.split_once("\n\n").unwrap();
    let ranges: Vec<_> = first
        .lines()
        .map(|l| {
            let (start, end) = l.split_once("-").unwrap();
            Range {
                start: start.parse().unwrap(),
                end: end.parse().unwrap(),
            }
        })
        .collect();
    let ids: Vec<u64> = second.lines().map(|l| l.parse().unwrap()).collect();
    DB {
        ranges: ranges,
        ids: ids,
    }
}

struct DB {
    ranges: Vec<Range>,
    ids: Vec<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    // assumes self <= other
    fn overlaps(&self, other: &Range) -> bool {
        self.end >= other.start
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

impl DB {
    fn freshes(self) -> Vec<u64> {
        let mut fresh = Vec::new();
        for id in self.ids {
            for range in self.ranges.iter() {
                if range.start <= id && range.end >= id {
                    fresh.push(id);
                    break;
                }
            }
        }
        fresh
    }

    fn merge_ranges(&mut self) -> Vec<Range> {
        dbg!(&self.ranges);
        self.ranges.sort();
        dbg!(&self.ranges);
        let mut current = self.ranges[0];
        let mut merged = Vec::new();
        for next in self.ranges[1..].iter().copied() {
            if current.overlaps(&next) {
                // let's merge them
                current.end = current.end.max(next.end);
                dbg!(current, next);
            } else {
                merged.push(current);
                dbg!(&merged);
                current = next;
            }
        }
        merged.push(current);
        merged
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
