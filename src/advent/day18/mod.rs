use std::collections::HashMap;
use std::collections::vec_deque::VecDeque;
use std::ops::Add;
use std::ops::AddAssign;
use indexmap::IndexMap;

use crate::read_file;
use std::hash::Hash;
use std::hash::Hasher;

pub const UP: I8Point = I8Point { x: 0, y: -1 };
pub const DOWN: I8Point = I8Point { x: 0, y: 1 };
pub const RIGHT: I8Point = I8Point { x: 1, y: 0 };
pub const LEFT: I8Point = I8Point { x: -1, y: 0 };

#[derive(Eq, PartialEq, Hash, Clone)]
struct State {
    position: I8Point,
    keys: Vec<char>,
}

fn is_key(value: char) -> bool {
    value.is_alphabetic() && value.is_lowercase()
}

fn is_door(value: char) -> bool {
    value.is_alphabetic() && value.is_uppercase()
}

fn step1(map: HashMap<I8Point, char>) -> Option<usize> {
    let start = State { position: map.iter().find(|&(_, &v)| v == '@').map(|(&p, _)| p).expect("should have a start position"), keys: vec!() };
    let nb_keys = map.iter().filter(|&(_, &v)| is_key(v)).count();
    let mut paths = HashMap::new();
    paths.insert(start.clone(), 0usize);

    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        for &dir in &[UP, LEFT, DOWN, RIGHT] {
            let next_position = current.position + dir;
            let next_value = *map.get(&next_position).unwrap_or(&'#');
            let mut next = None;

            if is_door(next_value) {
                if current.keys.contains(&next_value.to_ascii_lowercase()) {
                    // door open
                    next = Some(State { position: next_position, keys: current.keys.clone() });
                }
            } else if is_key(next_value) {
                let mut next_keys = current.keys.clone();
                if !current.keys.contains(&next_value) {
                    next_keys.push(next_value);
                    next_keys.sort();
                    if next_keys.len() == nb_keys {
                        return Some(paths.get(&current).unwrap() + 1);
                    }
                }
                next = Some(State { position: next_position, keys: next_keys });
            } else if next_value != '#' {
                next = Some(State { position: next_position, keys: current.keys.clone() });
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct I8Point {
    pub x: i8,
    pub y: i8,
}

impl I8Point {
    fn new(x: i8, y: i8) -> Self {
        I8Point { x, y }
    }
}

impl AddAssign for I8Point {
    fn add_assign(&mut self, other: I8Point) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }
}

impl Add for I8Point {
    type Output = I8Point;

    fn add(self, other: I8Point) -> I8Point {
        I8Point { x: self.x + other.x, y: self.y + other.y }
    }
}

#[derive(Clone)]
struct States {
    positions: [I8Point; 4],
    keys: Vec<char>,
    bot: Option<u8>,
}
impl PartialEq for States {
    fn eq(&self, other: &States) -> bool {
        self.positions.eq(&other.positions) && self.keys.eq(&other.keys)
    }
}
impl Eq for States {
}
impl Hash for States {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.positions.hash(state);
        self.keys.hash(state);
    }
}

fn step2(map: HashMap<I8Point, char>) -> Option<usize> {
    let mut start_positions = [I8Point::new(0, 0); 4];
    start_positions.copy_from_slice(map.iter().filter(|&(_, &v)| v == '@').map(|(&p, _)| p).collect::<Vec<I8Point>>().as_slice());
    let start = States { positions: start_positions, keys: vec!(), bot: None };
    let nb_keys = map.iter().filter(|&(_, &v)| is_key(v)).count();

    let mut paths = IndexMap::new();
    paths.insert(start.clone(), 0usize);

    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        for &dir in &[UP, LEFT, DOWN, RIGHT] {
            let active_bots = if let Some(bot) = current.bot { bot..=bot } else { 0..=3 };
            for i in active_bots {
                let next_position = current.positions[i as usize] + dir;
                if let Some(next_value) = map.get(&next_position) {
                    if *next_value == '#' {
                        continue;
                    }
                    if is_door(*next_value) && !current.keys.contains(&next_value.to_ascii_lowercase()) {
                        continue;
                    }

                    let mut next_positions = current.positions.clone();
                    next_positions[i as usize] += dir;

                    let next = if is_key(*next_value) && !current.keys.contains(next_value) {
                        let mut next_keys = current.keys.clone();
                        next_keys.push(*next_value);
                        next_keys.sort();
                        if next_keys.len() == nb_keys {
                            return Some(paths.get(&current).unwrap() + 1);
                        }
                        States { positions: next_positions, keys: next_keys, bot: None }
                    } else {
                        States { positions: next_positions, keys: current.keys.clone(), bot: current.bot }
                    };

                    if !paths.contains_key(&next) {
                        queue.push_back(next.clone());
                        paths.insert(next, paths.get(&current).unwrap() + 1);
                    }
                }
            }
        }
    }
    None
}

fn parse_input(input: String) -> HashMap<I8Point, char> {
    let mut result = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, value) in line.chars().enumerate() {
            result.insert(I8Point::new(x as i8, y as i8), value);
        }
    };
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_example1_step1() {
        assert_eq!(step1(parse_input(r#"#########
#b.A.@.a#
#########"#.to_string())), Some(8));
    }

    #[test]
    fn check_example2_step1() {
        assert_eq!(step1(parse_input(r#"########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################"#.to_string())), Some(86));
    }

    #[test]
    fn check_step1() {
        assert_eq!(step1(parse_input(read_file("src/advent/day18/input.txt"))), Some(3962));
    }

    #[test]
    fn check_example1_step2() {
        assert_eq!(step2(parse_input(r#"#######
#a.#Cd#
##@#@##
#######
##@#@##
#cB#.b#
#######"#.to_string())), Some(8));
    }

    #[test]
    fn check_example3_step2() {
        assert_eq!(step2(parse_input(r#"#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba@#@BcIJ#
#############
#nK.L@#@G...#
#M###N#H###.#
#o#m..#i#jk.#
#############"#.to_string())), Some(72));
    }

    #[test]
    fn check_step2() {
        assert_eq!(step2(parse_input(read_file("src/advent/day18/input-step2.txt"))), Some(3962));
    }
}
