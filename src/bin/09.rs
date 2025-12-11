use glam::I64Vec2;
use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let points = parse_input(input);
    let mut rects = Vec::new();
    for (i, p1) in points.iter().enumerate() {
        for p2 in points.iter().skip(i) {
            rects.push(makes_rect(p1, p2));
        }
    }
    // dbg!(&rects, &points);
    Some(rects.into_iter().max().unwrap() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let points = parse_input(input);
    let mut perimeter: Vec<(I64Vec2, I64Vec2)> = Vec::new();
    for (p1, p2) in points.iter().zip(points.iter().cycle().skip(1)) {
        perimeter.push((*p1, *p2));
    }

    let verts: Vec<_> = perimeter
        .iter()
        .copied()
        .filter(|(p1, p2)| I64Vec2::X.dot(p2 - p1) == 0)
        .collect();

    let horizontals: Vec<_> = perimeter
        .iter()
        .copied()
        .filter(|(p1, p2)| I64Vec2::Y.dot(p2 - p1) == 0)
        .collect();

    let mut rects = Vec::new();
    for (i, p1) in points.iter().enumerate() {
        for p2 in points.iter().skip(i + 1) {
            rects.push(((p1, p2), makes_rect(p1, p2)));
        }
    }
    rects.sort_by_key(|(_, size)| *size);

    let largest = rects
        .iter()
        .rev()
        .find(|((p1, p2), _size)| rect_inside(&perimeter, &verts, &horizontals, p1, p2));

    dbg!(largest);
    Some(largest.unwrap().1 as u64)
}

fn rect_inside(
    perimeter: &[(I64Vec2, I64Vec2)],
    verts: &[(I64Vec2, I64Vec2)],
    horizontals: &[(I64Vec2, I64Vec2)],
    p1: &I64Vec2,
    p2: &I64Vec2,
) -> bool {
    // println!("does {} - {} fit ?", p1, p2);
    let p3 = I64Vec2 { x: p1.x, y: p2.y };
    let p4 = I64Vec2 { x: p2.x, y: p1.y };
    if rect_intersects_perimeter(verts, horizontals, &[*p1, p3, *p2, p4]) {
        return false;
    } else {
        [&p3, &p4].iter().all(|p| {
            // fast check if we are ON the perimeter we are inside.
            if on_perimeter(perimeter, p) {
                true
            } else {
                inside_by_ray_casting(verts, horizontals, p)
            }
        })
    }
}

// Check if points are ON the perimeter
// Turns out the 2 other corners of the rectangle can be inside the perimeter not on it.
fn on_perimeter(perimeter: &[(I64Vec2, I64Vec2)], p1: &I64Vec2) -> bool {
    for (start, end) in perimeter {
        let maxx = start.x.max(end.x);
        let minx = start.x.min(end.x);
        let maxy = start.y.max(end.y);
        let miny = start.y.min(end.y);
        if p1.x >= minx && p1.x <= maxx && p1.y >= miny && p1.y <= maxy {
            return true;
        }
    }
    return false;
}

// I think this is technically ray casting not tracing ?
// Anyway from a point p we cast 4 rays, 1 in each cardinal direction
// and we check if we intersect with the perimeter, if we do in all directions we're inside.
// We can greatly reduce the search space and have a simple boundary condition to end the ray casting
// by only trying to intersect with segments of the perimeter which are orthogonal to us and are in the direction
// we are casting.
fn inside_by_ray_casting(
    verts: &[(I64Vec2, I64Vec2)],
    horizontals: &[(I64Vec2, I64Vec2)],
    p: &I64Vec2,
) -> bool {
    // vertical checks
    let vert_intersect = [I64Vec2::X, I64Vec2::NEG_X].iter().all(|dir| {
        for (p1, p2) in verts {
            // p1.x == p2.x
            if (p.x - p1.x) * dir.x < 0 {
                // if p * dir intersects [p1,p2] then p.y is between p1 and p2
                // so one diff is positive (or 0) and the other negative (or 0), so we can multiply them and check <= 0.
                if (p.y - p1.y) * (p.y - p2.y) <= 0 {
                    return true;
                }
            }
        }
        false
    });
    if !vert_intersect {
        return false;
    }

    // horizontal checks
    let horizontal_intersect = [I64Vec2::Y, I64Vec2::NEG_Y].iter().all(|dir| {
        for (p1, p2) in horizontals {
            // p1.y == p2.y
            if (p.y - p1.y) * dir.y < 0 {
                // if p * dir intersects [p1,p2] then p.y is between p1 and p2
                // so one diff is positive (or 0) and the other negative (or 0), so we can multiply them and check <= 0.
                if (p.x - p1.x) * (p.x - p2.x) <= 0 {
                    return true;
                }
            }
        }
        false
    });

    vert_intersect && horizontal_intersect
}

// follow each rect side and check if it intersects (and crosses) any perpendicular segment of the perimeter.
fn rect_intersects_perimeter(
    verts: &[(I64Vec2, I64Vec2)],
    horizontals: &[(I64Vec2, I64Vec2)],
    rect: &[I64Vec2; 4],
) -> bool {
    for (p1, p2) in rect.iter().circular_tuple_windows() {
        let dir = p2 - p1;
        match (I64Vec2::X.dot(dir), I64Vec2::Y.dot(dir)) {
            (0, _) => {
                // rect side is vertical : search horizontal perimeter segments
                for (start, end) in horizontals {
                    assert_eq!(p1.x, p2.x);
                    // If one diff is positive and the other negative, p1x is between start.x and end.x.
                    if (p1.x - start.x) * (p1.x - end.x) < 0 {
                        // strictly because we do not want to check the edge case here.
                        if (start.y - p1.y) * (start.y - p2.y) < 0 {
                            // intersection with crossing
                            return true;
                        }
                    }
                }
            }
            (_, 0) => {
                // rect side is horizontal : search vertical perimeter segments
                for (start, end) in verts {
                    assert_eq!(p1.y, p2.y);
                    // If one diff is positive and the other negative, p1x is between start.x and end.x.
                    if (p1.y - start.y) * (p1.y - end.y) < 0 {
                        // strictly because we do not want to check the edge case here.
                        if (start.x - p1.x) * (start.x - p2.x) < 0 {
                            // intersection with crossing
                            return true;
                        }
                    }
                }
            }
            _ => {
                dbg!(p1, p2, p2 - p1);
                unreachable!("only vertical and horizontal segments !")
            }
        }
    }
    false
}

fn parse_input(input: &str) -> Vec<I64Vec2> {
    input
        .lines()
        .map(|s| {
            let (a, b) = s.split_once(",").unwrap();
            I64Vec2 {
                x: a.parse().unwrap(),
                y: b.parse().unwrap(),
            }
        })
        .collect()
}

fn makes_rect(p1: &I64Vec2, p2: &I64Vec2) -> usize {
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
        assert_eq!(result, Some(24));
    }
}
