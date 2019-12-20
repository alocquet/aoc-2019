use std::collections::vec_deque::VecDeque;
use std::collections::HashMap;

use crate::advent::geometry::Map;
use crate::advent::geometry::Point;
use crate::advent::geometry::DOWN;
use crate::advent::geometry::LEFT;
use crate::advent::geometry::RIGHT;
use crate::advent::geometry::UP;

#[allow(clippy::map_entry)]
pub fn step1(input: String) -> Option<usize> {
    let input = parse_input(input);
    let raw_map = parse_raw_map(input);
    let map = parse_map(&raw_map);

    let start = map.find("AA".to_string()).unwrap();
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();

    queue.push_back(start);
    visited.insert(start, 0usize);

    while let Some(current) = queue.pop_front() {
        for dir in &[UP, RIGHT, DOWN, LEFT] {
            let neightboor = current + *dir;
            if let Some(neightboor_value) = map.values.get(&neightboor) {
                if neightboor_value != "#" && !visited.contains_key(&neightboor) {
                    visited.insert(neightboor, visited.get(&current).unwrap_or(&0) + 1);
                    if neightboor_value == "." {
                        queue.push_back(neightboor);
                    } else if neightboor_value == "ZZ" {
                        return Some(*visited.get(&current).unwrap() - 1);
                    } else {
                        let other_gate_position = map
                            .values
                            .iter()
                            .find(|(pos, val)| *val == neightboor_value && neightboor != **pos)
                            .map(|(&pos, _)| pos)
                            .unwrap();
                        if !visited.contains_key(&other_gate_position) {
                            queue.push_back(other_gate_position);
                            visited
                                .insert(other_gate_position, *visited.get(&current).unwrap_or(&0));
                        }
                    }
                }
            }
        }
    }

    None
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct State {
    position: Point,
    level: isize,
}

#[cfg_attr(tarpaulin, skip)]
#[allow(clippy::map_entry)]
pub fn step2(input: String) -> Option<usize> {
    let input = parse_input(input);
    let raw_map = parse_raw_map(input);
    let map = parse_map(&raw_map);

    let start = State {
        position: map.find("AA".to_string()).unwrap(),
        level: 0,
    };
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();

    queue.push_back(start);
    visited.insert(start, 0usize);
    while let Some(current) = queue.pop_front() {
        for dir in &[UP, RIGHT, DOWN, LEFT] {
            let neightboor_pos = current.position + *dir;
            if let Some(neightboor_value) = map.values.get(&neightboor_pos) {
                if neightboor_value == "ZZ" && current.level == 0 {
                    // end loop
                    return Some(*visited.get(&current).unwrap() - 1);
                }
                if neightboor_value == "#" || neightboor_value == "AA" || neightboor_value == "ZZ" {
                    // Stop
                    continue;
                }
                // Add neightboor to bfs queue
                let neightboor_state = State {
                    position: neightboor_pos,
                    level: current.level,
                };
                if !visited.contains_key(&neightboor_state) {
                    visited.insert(neightboor_state, visited.get(&current).unwrap_or(&0) + 1);
                    queue.push_back(neightboor_state);
                }
                // try to pass a gate
                if neightboor_value != "." {
                    let other_gate = map
                        .find_with(&|pos, val| val == neightboor_value && neightboor_pos != *pos)
                        .unwrap();
                    let next_level = if neightboor_pos.x < 5
                        || neightboor_pos.y < 5
                        || neightboor_pos.x > map.width() as isize - 5
                        || neightboor_pos.y > map.height() as isize - 5
                    {
                        current.level - 1
                    } else {
                        current.level + 1
                    };
                    if next_level >= 0 {
                        let next_gate_state = State {
                            position: other_gate,
                            level: next_level,
                        };
                        if !visited.contains_key(&next_gate_state) {
                            visited.insert(next_gate_state, *visited.get(&current).unwrap_or(&0));
                            queue.push_back(next_gate_state);
                        }
                    }
                }
            }
        }
    }

    None
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect()
}

fn parse_map(raw_map: &Map<char>) -> Map<String> {
    let mut map = Map::with_default_formatters(" ".to_string());
    for (&position, value) in &raw_map.values {
        if *value == '.' || *value == '#' {
            map.values.insert(position, format!("{}", value));
        } else if value.is_alphabetic() {
            let up = raw_map.values.get(&(position + UP));
            let down = raw_map.values.get(&(position + DOWN));
            let left = raw_map.values.get(&(position + LEFT));
            let right = raw_map.values.get(&(position + RIGHT));

            if let Some(neightboor_value) = match (up, down, left, right) {
                (Some(c), Some('.'), _, _) if c.is_alphabetic() => Some(format!("{}{}", c, value)),
                (Some('.'), Some(c), _, _) if c.is_alphabetic() => Some(format!("{}{}", value, c)),
                (_, _, Some(c), Some('.')) if c.is_alphabetic() => Some(format!("{}{}", c, value)),
                (_, _, Some('.'), Some(c)) if c.is_alphabetic() => Some(format!("{}{}", value, c)),
                _ => None,
            } {
                map.values.insert(position, neightboor_value);
            }
        }
    }
    map
}

fn parse_raw_map(input: Vec<Vec<char>>) -> Map<char> {
    let mut raw_map = Map::with_default_formatters('?');
    for (y, line) in input.iter().enumerate() {
        for (x, value) in line.iter().enumerate() {
            let position = Point::new(x as isize, y as isize);
            raw_map.values.insert(position, *value);
        }
    }
    raw_map
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    use super::*;

    const EXAMPLE_1: &str = r#"         A
         A
  #######.#########
  #######.........#
  #######.#######.#
  #######.#######.#
  #######.#######.#
  #####  B    ###.#
BC...##  C    ###.#
  ##.##       ###.#
  ##...DE  F  ###.#
  #####    G  ###.#
  #########.#####.#
DE..#######...###.#
  #.#########.###.#
FG..#########.....#
  ###########.#####
             Z
             Z"#;

    const EXAMPLE_2: &str = r#"                   A
                   A
  #################.#############
  #.#...#...................#.#.#
  #.#.#.###.###.###.#########.#.#
  #.#.#.......#...#.....#.#.#...#
  #.#########.###.#####.#.#.###.#
  #.............#.#.....#.......#
  ###.###########.###.#####.#.#.#
  #.....#        A   C    #.#.#.#
  #######        S   P    #####.#
  #.#...#                 #......VT
  #.#.#.#                 #.#####
  #...#.#               YN....#.#
  #.###.#                 #####.#
DI....#.#                 #.....#
  #####.#                 #.###.#
ZZ......#               QG....#..AS
  ###.###                 #######
JO..#.#.#                 #.....#
  #.#.#.#                 ###.#.#
  #...#..DI             BU....#..LF
  #####.#                 #.#####
YN......#               VT..#....QG
  #.###.#                 #.###.#
  #.#...#                 #.....#
  ###.###    J L     J    #.#.###
  #.....#    O F     P    #.#...#
  #.###.#####.#.#####.#####.###.#
  #...#.#.#...#.....#.....#.#...#
  #.#####.###.###.#.#.#########.#
  #...#.#.....#...#.#.#.#.....#.#
  #.###.#####.###.###.#.#.#######
  #.#.........#...#.............#
  #########.###.###.#############
           B   J   C
           U   P   P
"#;

    const EXAMPLE_3: &str = r#"
             Z L X W       C
             Z P Q B       K
  ###########.#.#.#.#######.###############
  #...#.......#.#.......#.#.......#.#.#...#
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###
  #.#...#.#.#...#.#.#...#...#...#.#.......#
  #.###.#######.###.###.#.###.###.#.#######
  #...#.......#.#...#...#.............#...#
  #.#########.#######.#.#######.#######.###
  #...#.#    F       R I       Z    #.#.#.#
  #.###.#    D       E C       H    #.#.#.#
  #.#...#                           #...#.#
  #.###.#                           #.###.#
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#
CJ......#                           #.....#
  #######                           #######
  #.#....CK                         #......IC
  #.###.#                           #.###.#
  #.....#                           #...#.#
  ###.###                           #.#.#.#
XF....#.#                         RF..#.#.#
  #####.#                           #######
  #......CJ                       NM..#...#
  ###.#.#                           #.###.#
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#
  #.....#        F   Q       P      #.#.#.#
  ###.###########.###.#######.#########.###
  #.....#...#.....#.......#...#.....#.#...#
  #####.#.###.#######.#######.###.###.#.#.#
  #.......#.......#.#.#.#.#...#...#...#.#.#
  #####.###.#####.#.#.#.#.###.###.#.###.###
  #.......#.....#.#...#...............#...#
  #############.#.#.###.###################
               A O F   N
               A A D   M"#;

    #[test]
    fn check_example1() {
        assert_eq!(step1(EXAMPLE_1.to_string()), Some(26));
    }

    #[test]
    fn check_example2() {
        assert_eq!(step1(EXAMPLE_2.to_string()), Some(58));
    }

    #[test]
    fn check_example1_step2() {
        assert_eq!(step2(EXAMPLE_1.to_string()), Some(26));
    }

    #[test]
    fn check_example3() {
        assert_eq!(step2(EXAMPLE_3.to_string()), Some(396));
    }

    #[test]
    fn check_step1() {
        assert_eq!(step1(read_file("src/advent/day20/input.txt")), Some(448));
    }

    #[test]
    #[ignore]
    fn check_step2() {
        assert_eq!(step2(read_file("src/advent/day20/input.txt")), Some(5678));
    }
}
