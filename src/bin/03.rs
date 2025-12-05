advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let banks = parse_input(input);
    let power = banks.iter().map(|bank| largest_recurse(&bank, 1)).sum();

    Some(power)
}

pub fn part_two(input: &str) -> Option<u64> {
    let banks = parse_input(input);
    let powers = banks.iter().map(|bank| largest_recurse(&bank, 11)).sum();

    Some(powers)
}

fn largest_recurse(slice: &[u64], remaining_n_to_pick_after: usize) -> u64 {
    let candidates = &slice[..slice.len() - remaining_n_to_pick_after];

    let (index, value) = candidates
        .iter()
        .enumerate()
        .max_by(|a, b| match a.1.cmp(b.1) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => b.0.cmp(&a.0),
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        })
        .unwrap();
    // println!("{candidates:?}, {remaining_n_to_pick_after} : {value}");
    if remaining_n_to_pick_after == 0 {
        return *value;
    } else {
        return *value * 10u64.pow(remaining_n_to_pick_after as u32)
            + largest_recurse(&slice[index + 1..], remaining_n_to_pick_after - 1); // starting after the value selected
    }
}

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| (c as u8 - 48) as u64).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
