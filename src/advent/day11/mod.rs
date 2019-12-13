use crate::advent::geometry::Map;
use crate::advent::geometry::ORIGIN;
use crate::advent::geometry::UP;
use crate::advent::intcode::parse_input;
use crate::advent::intcode::Program;
use crate::advent::intcode::ProgramState;

pub fn step1() -> usize {
    let map = run_robot(false);
    map.values.len()
}

pub fn step2() -> String {
    let map = run_robot(true);
    format!("{}", &map)
}

fn run_robot(first_value: bool) -> Map<bool> {
    let mut map = Map::new(|value| match value.unwrap_or(&false) {
        true => '#',
        false => '.',
    });
    let mut prog = Program::new(parse_input("11"));
    let mut current_point = ORIGIN;
    let mut current_direction = UP;
    map.values.insert(current_point, first_value);

    while prog.state != ProgramState::Halted {
        let input = map.values.get(&current_point).unwrap_or(&false);
        prog.input.push_back(*input as isize);
        prog.execute();
        let output = prog.output.pop_front().expect("must have a color output");
        let turn = prog
            .output
            .pop_front()
            .expect("must have a direction output");
        current_direction = current_direction.rotate(turn == 1);
        map.values.insert(current_point, output != 0);
        current_point += current_direction;
    }
    map
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
