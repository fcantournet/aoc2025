use glam::IVec2;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let points = parse_input(input);
    let mut rects = Vec::new();
    for (i, p1) in points.iter().enumerate() {
        for p2 in points.iter().skip(i) {
            rects.push(makes_rect(p1, p2));
        }
    }
    dbg!(&rects, &points);
    Some(rects.into_iter().max().unwrap() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let points = parse_input(input);

    display(&points, 12);

    let mut perimeter: Vec<(IVec2, IVec2)> = Vec::new();
    for (p1, p2) in points.iter().zip(points.iter().cycle().skip(1)) {
        perimeter.push((*p1, p2-p1));
    }

    let mut rects = Vec::new();
    for (i, p1) in points.iter().enumerate() {
        for p2 in points.iter().skip(i+1) {
            if rect_inside(&perimeter, p1, p2) {
                rects.push(makes_rect(p1, p2));
            }
        }
    }
    Some(rects.into_iter().max().unwrap() as u64)
}

fn rect_inside(perimeter: &[(IVec2, IVec2)], p1: &IVec2, p2: &IVec2) -> bool {
    println!("does {} - {} fit ?", p1, p2);
    let p3 = IVec2{x: p1.x, y: p2.y};
    let p4 = IVec2{x: p2.x, y: p1.y};
    [p1, p2, &p3, &p4].iter().all(|p| in_perimeter(perimeter, p))
}

fn in_perimeter(perimeter: &[(IVec2, IVec2)], p1: &IVec2) -> bool {
    for (start, dir) in perimeter {
        if dir.perp_dot(p1-start) >= 0 {
            println!("  nope because: {} is not on the right side of {} -> {}", p1, start, dir);
            return false
        }
    }
    return true
}

fn parse_input(input: &str) -> Vec<IVec2> {
    input.lines().map(|s| {
        let (a, b) = s.split_once(",").unwrap();
        IVec2{
            x: a.parse().unwrap(),
            y: b.parse().unwrap(),
        }
    }).collect()
}

fn display(points: &[IVec2], size: usize) {
    let mut grid = vec![vec!['.'; size]; size];
    for point in points {
        grid[point.y as usize][point.x as usize] = '#';
    }
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

fn makes_rect(p1: &IVec2, p2: &IVec2) -> usize {
    (1 + (p1.x - p2.x).abs() as usize) * (1 + (p1.y - p2.y).abs() as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
