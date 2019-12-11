use crate::read_file;
use std::collections::HashMap;
use crate::advent::intcode::Program;
use crate::advent::intcode::parse_input;
use crate::advent::geometry::Point;
use crate::advent::geometry::ORIGIN;
use crate::advent::intcode::ProgramState;
use crate::advent::geometry::UP;
use std::fmt::Display;

pub fn step1() -> usize {
    let mut impress = run_robot(0);
    impress.len()
}


pub fn step2() -> String{
    let mut result = String::new();

    let mut impress = run_robot(1);
    let mut points: Vec<Point> = impress.keys().cloned().collect();
    let min_x = points.iter().min_by_key(|p| p.x).unwrap().x;
    let max_x = points.iter().max_by_key(|p| p.x).unwrap().x;
    let min_y = points.iter().min_by_key(|p| p.y).unwrap().y;
    let max_y = points.iter().max_by_key(|p| p.y).unwrap().y;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            result.push(if *impress.get(&Point::new(x, y)).unwrap_or(&0) == 0 { '.' } else { '#' });
        }
        result.push_str("\n");
    }
    result
}

fn run_robot(first_value: isize) -> HashMap<Point, isize> {
    let mut impress = HashMap::new();
    let mut prog = Program::new(parse_input("11"));
    let mut current_point = ORIGIN;
    let mut current_direction = UP;
    impress.insert(current_point, first_value);

    while prog.state != ProgramState::Halted {
        let input = impress.get(&current_point).unwrap_or(&0);
        prog.input.push_back(*input);
        prog.execute();
        let output = prog.output.pop_front().expect("must have a color output");
        let turn = prog.output.pop_front().expect("must have a direction output");
        current_direction = current_direction.rotate(turn == 1);
        impress.insert(current_point, output);
        current_point += current_direction;
    }
    impress
}

#[cfg(test)]
mod tests {
    use super::*;

    const STEP2: &str = r#".#..#...##.#..#...##.#..#..##..###..#..#...
.#..#....#.#.#.....#.#.#..#..#.#..#.#..#...
.####....#.##......#.##...#....#..#.####...
.#..#....#.#.#.....#.#.#..#.##.###..#..#...
.#..#.#..#.#.#..#..#.#.#..#..#.#....#..#...
.#..#..##..#..#..##..#..#..###.#....#..#...
"#;

    #[test]
    fn check_step1() {
        assert_eq!(step1(), 2511);
    }

    #[test]
    fn check_step2() {
        assert_eq!(step2(), STEP2.to_string());
    }
}
