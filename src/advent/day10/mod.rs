use crate::advent::geometry::Point;
use std::collections::HashMap;
use std::collections::HashSet;

const ASTEROID: char = '#';

pub fn parse_map(input: String) -> Vec<Point> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == ASTEROID)
                .map(move |(x, _)| Point {
                    x: x as isize,
                    y: y as isize,
                })
        })
        .collect()
}

pub fn find_best_asteroid(asteroids: &[Point]) -> Option<(Point, usize)> {
    let mut result = None;
    let mut count = 0;
    for &a in asteroids {
        let mut angles = HashSet::new();
        for &b in asteroids {
            if a != b {
                // multiplication by 100_000 and conversion to isize because Eq is not implemented f64
                angles.insert((a.angle_with(&b) * 100_000.0) as isize);
            }
        }
        if angles.len() > count {
            count = angles.len();
            result = Some((a, count));
        }
    }
    result
}

pub fn find_nth_vaporized(asteroids: &[Point], nth: usize) -> Point {
    let (station, _) = find_best_asteroid(asteroids).unwrap();

    let mut angles: HashMap<isize, Point> = HashMap::new();
    for &b in asteroids {
        if station != b {
            let angle = -(station.angle_with(&b) * 100_000.0) as isize;
            angles
                .entry(angle)
                .and_modify(|other| {
                    if other.manhattan_distance_from(&station) > b.manhattan_distance_from(&station)
                    {
                        *other = b;
                    }
                })
                .or_insert(b);
        }
    }

    let mut keys = angles.keys().collect::<Vec<&isize>>();
    keys.sort();
    let angle = keys
        .iter()
        .cycle()
        .skip_while(|&&&a| a < 0)
        .nth(nth - 1)
        .unwrap();
    *angles.get(angle).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file;

    #[test]
    fn check_step1() {
        assert_eq!(
            find_best_asteroid(&parse_map(read_file("src/advent/day10/input.txt"))),
            Some((Point { x: 8, y: 16 }, 214))
        );
    }

    #[test]
    fn check_step2() {
        assert_eq!(
            find_nth_vaporized(&parse_map(read_file("src/advent/day10/input.txt")), 200),
            Point { x: 5, y: 2 }
        );
    }
}
