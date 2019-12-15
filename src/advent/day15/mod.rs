use crate::advent::geometry::Map;
use crate::advent::geometry::Point;
use crate::advent::geometry::DOWN;
use crate::advent::geometry::LEFT;
use crate::advent::geometry::ORIGIN;
use crate::advent::geometry::RIGHT;
use crate::advent::geometry::UP;
use crate::advent::intcode::Program;
use std::collections::vec_deque::VecDeque;

#[derive(Clone)]
pub struct State {
    position: Point,
    step: usize,
    program: Program,
}

#[allow(clippy::map_entry)]
pub fn execute(input: Program) -> Result<(usize, Option<Program>), &'static str> {
    let mut visited = Map::new(
        |f, v| write!(f, "{}", v.unwrap_or(&'?')),
        |f, _| writeln!(f),
    );
    let mut queue = VecDeque::new();
    let mut max = 0;

    queue.push_back(State {
        position: ORIGIN,
        step: 0,
        program: input,
    });
    visited.values.insert(ORIGIN, '+');

    while let Some(state) = queue.pop_front() {
        max = max.max(state.step);
        for (direction, direction_value) in &[(UP, 1), (RIGHT, 4), (DOWN, 2), (LEFT, 3)] {
            let new_position = state.position + *direction;
            if !visited.values.contains_key(&new_position) {
                let mut new_state = State {
                    position: new_position,
                    step: state.step + 1,
                    program: state.program.clone(),
                };
                new_state.program.input.push_back(*direction_value);
                new_state.program.execute();
                match new_state.program.output.pop_front() {
                    Some(0) => {
                        // hit a wall
                        visited.values.insert(new_position, '#');
                    }
                    Some(1) => {
                        // move to direction
                        queue.push_back(new_state);
                        visited.values.insert(new_position, '.');
                    }
                    Some(2) => {
                        // found oxygen
                        visited.values.insert(new_position, 'O');
                        println!("{}", &visited);
                        return Ok((new_state.step, Some(new_state.program)));
                    }
                    _ => {
                        return Err("program should always return a valid output");
                    }
                }
            }
        }
    }
    println!("{}", &visited);
    Ok((max, None))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::advent::intcode::parse_input;

    #[test]
    fn check_step1() {
        assert_eq!(execute(Program::new(parse_input("15"))).unwrap().0, 214);
    }

    #[test]
    fn check_step2() {
        let program_step1 = execute(Program::new(parse_input("15"))).unwrap().1.unwrap();
        assert_eq!(execute(program_step1).unwrap().0, 344);
    }
}
