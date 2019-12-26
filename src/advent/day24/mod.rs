use std::collections::BTreeSet;

use crate::advent::geometry::DOWN;
use crate::advent::geometry::LEFT;
use crate::advent::geometry::ORIGIN;
use crate::advent::geometry::Point;
use crate::advent::geometry::RIGHT;
use crate::advent::geometry::UP;
use crate::read_file;

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

fn get_adjacents_step1(position: Point) -> Vec<Point> {
    [UP, DOWN, LEFT, RIGHT]
        .iter()
        .map(|&dir| position + dir)
        .filter(|adjacent| adjacent.is_in(ORIGIN, LIMIT))
        .collect()
}

fn execute(input: &BTreeSet<Point>, get_adjacents: &dyn Fn(Point) -> Vec<Point>) -> BTreeSet<Point> {
    let mut next = BTreeSet::new();


    for bug in input.iter() {
        let adjacents = get_adjacents(*bug);
        let adjacent_bugs = adjacents.iter().filter(|&adjacent| input.contains(adjacent)).count();
        if adjacent_bugs == 1 {
            next.insert(*bug);
        }
        for adjacent in adjacents {
            if !input.contains(&adjacent) {
                let adj_adjacents = get_adjacents(adjacent);
                let adj_adjacent_bugs = adj_adjacents.iter().filter(|&adj_adjacent| input.contains(adj_adjacent)).count();
                if adj_adjacent_bugs == 1 || adj_adjacent_bugs == 2 {
                    next.insert(adjacent);
                }
            }
        }
    }

    next
}

const LIMIT: Point = Point { x: 4, y: 4 };

fn parse_input(input: String) -> BTreeSet<Point> {
    let mut result = BTreeSet::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars()
            .enumerate()
            .for_each(|(x, value)| { if value == '#' { result.insert(Point::new(x as isize, y as isize)); } })
    });

    result
}

fn biodiversity_rating(map: BTreeSet<Point>) -> usize {
    map.iter().fold(0, |sum, bug| {
        return sum + 2usize.pow(bug.x as u32 + bug.y as u32 * 5);
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_one_execution() {
        assert_eq!(biodiversity_rating(
            execute(&parse_input(
                r#"....#
#..#.
#..##
..#..
#...."#
                    .to_string()
            ), &get_adjacents_step1)), 7200233
        );
    }

    #[test]
    fn test_biodiversity_rating() {
        let mut map = BTreeSet::new();
        map.insert(Point::new(0, 3));
        map.insert(Point::new(1, 4));
        assert_eq!(biodiversity_rating(map), 2129920);
    }

    #[test]
    fn check_step1() {
        assert_eq!(step1(), 18375063);
    }
}
