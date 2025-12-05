advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = parse_input(input);
    let mut total = 0u64;
    for range in ranges {
        for i in range.0..=range.1 {
            let s = i.to_string();
            if s.len() % 2 == 0 {
                let (first, second) = s.split_at(s.len() / 2);
                if first == second {
                    total += i as u64;
                }
            }
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = parse_input(input);
    let mut total = 0u64;
    for range in ranges {
        for i in range.0..=range.1 {
            let s = i.to_string();
            for size in 1..=s.len() / 2 {
                if valid(&s, size) {
                    //println!("range {}-{} : {i}", range.0, range.1);
                    total += i as u64;
                    break; // do not count a single ID multiple times if it has several rule matches.
                }
            }
        }
    }
    Some(total)
}

fn valid(s: &str, length: usize) -> bool {
    if s.len() % length != 0 {
        return false;
    }
    s[0..length]
        .chars()
        .cycle()
        .zip(s[length..].chars())
        .all(|(a, b)| a == b)

    // let mut chunks = s.chunks_exact(length);
    // let base = chunks.next().unwrap();
    // while let Some(next) = chunks.next() {
    //     if base != next {
    //         return false;
    //     }
    // }
    // true
}

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .trim_end()
        .split(",")
        .map(|range| {
            let (start, end) = range.split_once("-").unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));

        assert_eq!(result, Some(4174379265));
    }
}
