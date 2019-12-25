use std::collections::HashSet;

use crate::advent::geometry::Point;
use crate::advent::geometry::DOWN;
use crate::advent::geometry::LEFT;
use crate::advent::geometry::ORIGIN;
use crate::advent::geometry::RIGHT;
use crate::advent::geometry::UP;
use crate::read_file;

type BugsMap = [[bool; 5]; 5];

pub fn step1() -> usize {
    let mut map = parse_input(read_file("src/advent/day24/input.txt"));
    let mut already_seen = HashSet::new();
    already_seen.insert(map.clone());

    loop {
        map = execute(&map);
        if already_seen.contains(&map) {
            return biodiversity_rating(map);
        }
        already_seen.insert(map.clone());
    }
}

fn execute(input: &BugsMap) -> BugsMap {
    let limit = Point::new(4, 4);
    let mut next = *input;

    for x in 0..5 {
        for y in 0..5 {
            let position = Point::new(x, y);
            let adjacents: Vec<Point> = [UP, DOWN, LEFT, RIGHT]
                .iter()
                .map(|&dir| position + dir)
                .filter(|adjacent| adjacent.is_in(ORIGIN, limit))
                .collect();
            let adjacent_bugs = adjacents
                .iter()
                .filter(|&adjacent| input[adjacent.x as usize][adjacent.y as usize])
                .count();
            if input[position.x as usize][position.y as usize] {
                // A bug dies (becoming an empty space) unless there is exactly one bug adjacent to it.
                next[position.x as usize][position.y as usize] = adjacent_bugs == 1;
            } else {
                // An empty space becomes infested with a bug if exactly one or two bugs are adjacent to it.
                next[position.x as usize][position.y as usize] =
                    adjacent_bugs == 1 || adjacent_bugs == 2;
            }
        }
    }

    next
}

fn parse_input(input: String) -> BugsMap {
    let mut result = [[false; 5]; 5];
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars()
            .enumerate()
            .for_each(|(x, value)| result[y][x] = value == '#')
    });

    result
}

fn biodiversity_rating(map: BugsMap) -> usize {
    let mut result = 0;
    for (x, line) in map.iter().enumerate() {
        for (y, bug) in line.iter().enumerate() {
            if *bug {
                result += 2usize.pow(y as u32 + x as u32 * 5);
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_one_execution() {
        assert_eq!(
            execute(&parse_input(
                r#"....#
#..#.
#..##
..#..
#...."#
                    .to_string()
            )),
            [
                [true, false, false, true, false],
                [true, true, true, true, false],
                [true, true, true, false, true],
                [true, true, false, true, true],
                [false, true, true, false, false]
            ]
        );
    }

    #[test]
    fn test_biodiversity_rating() {
        assert_eq!(
            biodiversity_rating([
                [false, false, false, false, false],
                [false, false, false, false, false],
                [false, false, false, false, false],
                [true, false, false, false, false],
                [false, true, false, false, false]
            ]),
            2129920
        );
    }

    #[test]
    fn check_step1() {
        assert_eq!(step1(), 18375063);
    }
}
