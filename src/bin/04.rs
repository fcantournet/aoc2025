advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let map = parse_input(input);
    let mut removed = 0;
    for y in 0..map.y {
        for x in 0..map.x {
            if map.at(x as isize, y as isize) == Cell::Roll {
                if neighbourgs(&map, x as isize, y as isize) < 4 {
                    removed += 1;
                }
            }
        }
    }

    Some(removed as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = parse_input(input);
    let removed = remove(&map);

    Some(removed as u64)
}

fn remove(map: &PaddedGrid<Cell>) -> usize {
    let mut clone = map.clone();
    let mut removed = 0;
    for y in 0..map.y {
        for x in 0..map.x {
            if clone.at(x as isize, y as isize) == Cell::Roll {
                if neighbourgs(map, x as isize, y as isize) < 4 {
                    clone.insert_at(x as isize, y as isize, Cell::Empty);
                    removed += 1;
                }
            }
        }
    }
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

fn neighbourgs(map: &PaddedGrid<Cell>, x: isize, y: isize) -> usize {
    let mut n = 0;
    for (dx, dy) in NEIGHBOURS {
        if map.at(x + dx, y + dy) == Cell::Roll {
            n += 1;
        }
    }
    n
}

fn parse_input(input: &str) -> PaddedGrid<Cell> {
    let y = input.lines().count();
    let x = input.lines().next().unwrap().chars().count();
    dbg!(x, y);
    let mut map = PaddedGrid::<Cell>::new(x, y, 1);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '@' {
                map.insert_at(x as isize, y as isize, Cell::Roll);
            }
        }
    }
    map
}

#[derive(Default, PartialEq, Eq, Copy, Clone)]
enum Cell {
    #[default]
    Empty,
    Roll,
}

#[derive(Debug, Clone)]
struct PaddedGrid<T>
where
    T: Sized + Default + Copy,
{
    padding: isize,
    pub x: usize,
    pub y: usize,
    padded_width: usize,
    inner: Vec<T>,
}

impl<T> PaddedGrid<T>
where
    T: Sized + Default + Copy,
{
    fn new(x: usize, y: usize, padding: usize) -> Self {
        let outer_len = (x + 2 * padding) * (y + 2 * padding);
        let mut inner = Vec::<T>::with_capacity(outer_len);
        inner.resize_with(outer_len, T::default);
        Self {
            padding: padding as isize,
            x,
            y,
            padded_width: x + 2 * padding,
            inner,
        }
    }

    fn index(&self, x: isize, y: isize) -> usize {
        ((y + self.padding) as usize) * self.padded_width + (x + self.padding) as usize
    }

    fn insert_at(&mut self, x: isize, y: isize, val: T) {
        let index = self.index(x, y);
        self.inner[index] = val;
    }

    fn at(&self, x: isize, y: isize) -> T {
        let index = self.index(x, y);
        self.inner[index]
    }
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
