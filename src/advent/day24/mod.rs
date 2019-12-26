use std::collections::BTreeSet;

use crate::advent::geometry::Point;
use crate::advent::geometry::DOWN;
use crate::advent::geometry::LEFT;
use crate::advent::geometry::ORIGIN;
use crate::advent::geometry::RIGHT;
use crate::advent::geometry::UP;
use crate::read_file;

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
struct LPoint {
    p: Point,
    l: isize,
}

pub fn step1() -> usize {
    let mut map = parse_input(read_file("src/advent/day24/input.txt"));
    let mut already_seen = BTreeSet::new();
    already_seen.insert(map.clone());

    loop {
        map = execute(&map, &get_adjacents_step1);
        if already_seen.contains(&map) {
            return biodiversity_rating(map);
        }
        already_seen.insert(map.clone());
    }
}

pub fn step2() -> usize {
    let mut map = parse_input(read_file("src/advent/day24/input.txt"));

    for _ in 0..200 {
        map = execute(&map, &get_adjacents_step2);
    }
    map.len()
}

fn get_adjacents_step1(position: LPoint) -> Vec<LPoint> {
    [UP, DOWN, LEFT, RIGHT]
        .iter()
        .map(|&dir| LPoint {
            p: position.p + dir,
            l: position.l,
        })
        .filter(|adjacent| adjacent.p.is_in(ORIGIN, LIMIT))
        .collect()
}

fn get_adjacents_step2(position: LPoint) -> Vec<LPoint> {
    let mut result = vec![];
    for &dir in &[UP, DOWN, LEFT, RIGHT] {
        let adjacent = LPoint {
            p: position.p + dir,
            l: position.l,
        };
        if adjacent.p.x == 2 && adjacent.p.y == 2 {
            for i in 0..5 {
                match dir {
                    UP => result.push(LPoint {
                        p: Point::new(i, 4),
                        l: position.l - 1,
                    }),
                    DOWN => result.push(LPoint {
                        p: Point::new(i, 0),
                        l: position.l - 1,
                    }),
                    LEFT => result.push(LPoint {
                        p: Point::new(4, i),
                        l: position.l - 1,
                    }),
                    _ => result.push(LPoint {
                        p: Point::new(0, i),
                        l: position.l - 1,
                    }),
                };
            }
        } else if adjacent.p.is_in(ORIGIN, LIMIT) {
            result.push(adjacent);
        } else {
            result.push(LPoint {
                p: Point::new(2, 2) + dir,
                l: position.l + 1,
            });
        }
    }
    result
}

fn execute(
    input: &BTreeSet<LPoint>,
    get_adjacents: &dyn Fn(LPoint) -> Vec<LPoint>,
) -> BTreeSet<LPoint> {
    let mut next = BTreeSet::new();

    for bug in input.iter() {
        let adjacents = get_adjacents(*bug);
        let adjacent_bugs = adjacents
            .iter()
            .filter(|&adjacent| input.contains(adjacent))
            .count();
        if adjacent_bugs == 1 {
            next.insert(*bug);
        }
        for adjacent in adjacents {
            if !input.contains(&adjacent) {
                let adj_adjacents = get_adjacents(adjacent);
                let adj_adjacent_bugs = adj_adjacents
                    .iter()
                    .filter(|&adj_adjacent| input.contains(adj_adjacent))
                    .count();
                if adj_adjacent_bugs == 1 || adj_adjacent_bugs == 2 {
                    next.insert(adjacent);
                }
            }
        }
    }

    next
}

const LIMIT: Point = Point { x: 4, y: 4 };

fn parse_input(input: String) -> BTreeSet<LPoint> {
    let mut result = BTreeSet::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, value)| {
            if value == '#' {
                result.insert(LPoint {
                    p: Point::new(x as isize, y as isize),
                    l: 0,
                });
            }
        })
    });

    result
}

fn biodiversity_rating(map: BTreeSet<LPoint>) -> usize {
    map.iter().fold(0, |sum, bug| {
        sum + 2usize.pow(bug.p.x as u32 + bug.p.y as u32 * 5)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_one_execution() {
        assert_eq!(
            biodiversity_rating(execute(
                &parse_input(
                    r#"....#
#..#.
#..##
..#..
#...."#
                        .to_string()
                ),
                &get_adjacents_step1
            )),
            7200233
        );
    }

    #[test]
    fn test_biodiversity_rating() {
        let mut map = BTreeSet::new();
        map.insert(LPoint {
            p: Point::new(0, 3),
            l: 0,
        });
        map.insert(LPoint {
            p: Point::new(1, 4),
            l: 0,
        });
        assert_eq!(biodiversity_rating(map), 2129920);
    }

    #[test]
    fn check_step1() {
        assert_eq!(step1(), 18375063);
    }

    #[test]
    fn check_step2() {
        assert_eq!(step2(), 1959);
    }
}
