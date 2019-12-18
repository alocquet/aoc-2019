use std::collections::vec_deque::VecDeque;
use std::collections::HashMap;
use std::hash::Hash;

use crate::advent::geometry::Map;
use crate::advent::geometry::Point;
use crate::advent::geometry::DOWN;
use crate::advent::geometry::LEFT;
use crate::advent::geometry::RIGHT;
use crate::advent::geometry::UP;
use crate::read_file;

#[derive(Eq, PartialEq, Hash, Clone)]
struct State {
    position: Point,
    keys: Vec<char>,
}

fn is_key(value: char) -> bool {
    value.is_alphabetic() && value.is_lowercase()
}

fn is_door(value: char) -> bool {
    value.is_alphabetic() && value.is_uppercase()
}

#[allow(clippy::map_entry)]
pub fn step1(map: Map<char>) -> Option<usize> {
    let start = State {
        position: map
            .values
            .iter()
            .find(|&(_, &v)| v == '@')
            .map(|(&p, _)| p)
            .expect("should have a start position"),
        keys: vec![],
    };
    let keys = map
        .values
        .iter()
        .filter(|&(_, &v)| is_key(v))
        .map(|(_, &v)| v)
        .collect::<Vec<char>>();
    let mut paths = HashMap::new();
    paths.insert(start.clone(), 0usize);

    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        for &dir in &[UP, LEFT, DOWN, RIGHT] {
            let next_position = current.position + dir;
            let next_value = *map.values.get(&next_position).unwrap_or(&'#');
            let mut next = None;

            if is_door(next_value) {
                if !keys.contains(&next_value.to_ascii_lowercase())
                    || current.keys.contains(&next_value.to_ascii_lowercase())
                {
                    // door open
                    next = Some(State {
                        position: next_position,
                        keys: current.keys.clone(),
                    });
                }
            } else if is_key(next_value) {
                let mut next_keys = current.keys.clone();
                if !current.keys.contains(&next_value) {
                    next_keys.push(next_value);
                    next_keys.sort();
                    if next_keys.len() == keys.len() {
                        return Some(paths.get(&current).unwrap() + 1);
                    }
                }
                next = Some(State {
                    position: next_position,
                    keys: next_keys,
                });
            } else if next_value != '#' {
                next = Some(State {
                    position: next_position,
                    keys: current.keys.clone(),
                });
            }

            if let Some(next_state) = next {
                if !paths.contains_key(&next_state) {
                    queue.push_back(next_state.clone());
                    paths.insert(next_state, paths.get(&current).unwrap() + 1);
                }
            }
        }
    }
    None
}

pub fn step2() -> usize {
    step1(parse_input(read_file("src/advent/day18/input-step2-a.txt"))).unwrap()
        + step1(parse_input(read_file("src/advent/day18/input-step2-b.txt"))).unwrap()
        + step1(parse_input(read_file("src/advent/day18/input-step2-c.txt"))).unwrap()
        + step1(parse_input(read_file("src/advent/day18/input-step2-d.txt"))).unwrap()
}

fn parse_input(input: String) -> Map<char> {
    let mut result = Map::new(
        |f, v| write!(f, "{}", v.unwrap_or(&'?')),
        |f, _| writeln!(f),
    );
    for (y, line) in input.lines().enumerate() {
        for (x, value) in line.chars().enumerate() {
            result
                .values
                .insert(Point::new(x as isize, y as isize), value);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_example1_step1() {
        assert_eq!(
            step1(parse_input(
                r#"#########
#b.A.@.a#
#########"#
                    .to_string()
            )),
            Some(8)
        );
    }

    #[test]
    fn check_example2_step1() {
        assert_eq!(
            step1(parse_input(
                r#"########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################"#
                    .to_string()
            )),
            Some(86)
        );
    }

    #[test]
    #[ignore]
    fn check_step1() {
        assert_eq!(
            step1(parse_input(read_file("src/advent/day18/input.txt"))),
            Some(3962)
        );
    }

    #[test]
    fn check_step2() {
        assert_eq!(step2(), 1844);
    }
}
