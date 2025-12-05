advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let swings = parse_input(input);
    let mut pos = 50isize;
    let mut count = 0;
    for s in swings {
        pos = (pos + s).rem_euclid(100);
        if pos == 0 {
            count += 1;
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let swings = parse_input(input);

    // println!(
    // "min: {}, max: {}",
    // swings.iter().min().unwrap(),
    // swings.iter().max().unwrap(),
    // );
    let mut pos = 50isize;
    let mut count = 0;
    for s in swings {
        let rounds = s.abs().div_euclid(100);
        count += rounds as u64;
        let next = (pos + s).rem_euclid(100);
        // println!("{next} = ({pos} + {s}) % 100");
        // println!("  {s} * {next} - {pos} = {} < 0", s * (next - pos));

        // If we end up on 0, OR we didn't start on 0, and we end up not on the same "side" as the swing
        if (next == 0) || (pos != 0 && s * (next - pos) < 0) {
            count += 1;
        }
        pos = next
    }
    Some(count)
}

fn parse_input(input: &str) -> Vec<isize> {
    input
        .lines()
        .map(|s| {
            let (sign, number) = s.split_at(1);
            let sign = match sign.chars().next().unwrap() {
                'L' => -1,
                'R' => 1,
                _ => unreachable!(),
            };
            let value: isize = number.parse().unwrap();
            value * sign
        })
        .collect()
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
        assert_eq!(result, Some(6));
    }
}
