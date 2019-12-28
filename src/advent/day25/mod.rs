use std::collections::vec_deque::VecDeque;
use std::collections::HashMap;
use std::collections::HashSet;

use regex::Regex;

use crate::advent::geometry::Point;
use crate::advent::geometry::DOWN;
use crate::advent::geometry::LEFT;
use crate::advent::geometry::ORIGIN;
use crate::advent::geometry::RIGHT;
use crate::advent::geometry::UP;
use crate::advent::intcode::Program;

struct State {
    point: Point,
    prog: Program,
    path: Vec<String>,
}

pub fn execute(input: Vec<isize>) -> usize {
    let prog = Program::new(input.clone());
    let mut visited = HashSet::new();
    let mut items = HashMap::new();

    let mut queue = VecDeque::new();
    queue.push_back(State {
        point: ORIGIN,
        prog,
        path: vec![],
    });

    while let Some(mut current) = queue.pop_front() {
        visited.insert(current.point);
        let output = run_program(&mut current.prog);
        for dir in &[
            ("west", LEFT),
            ("south", DOWN),
            ("north", UP),
            ("east", RIGHT),
        ] {
            if output.contains(dir.0) {
                let next_point = current.point + dir.1;
                if !visited.contains(&next_point) {
                    let mut next = current.prog.clone();
                    next.input.extend(
                        dir.0
                            .chars()
                            .map(|x| (x as u8) as isize)
                            .collect::<Vec<isize>>(),
                    );
                    next.input.push_back(10);
                    let mut next_path = current.path.clone();
                    next_path.push(dir.0.to_string());
                    queue.push_back(State {
                        point: next_point,
                        prog: next,
                        path: next_path,
                    });
                }
            }
        }
        if output.contains("Items here:") {
            let item = Regex::new("Items here:\n- (.*)\n")
                .unwrap()
                .captures(&output)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str();
            if item != "infinite loop" {
                let mut next = current.prog.clone();
                next.input.extend(
                    "take "
                        .chars()
                        .map(|x| (x as u8) as isize)
                        .collect::<Vec<isize>>(),
                );
                next.input.extend(
                    item.chars()
                        .map(|x| (x as u8) as isize)
                        .collect::<Vec<isize>>(),
                );
                next.input.push_back(10);
                let output_item = run_program(&mut next);
                if output_item.contains("Command?") {
                    items.insert(item.to_string(), current.path);
                }
            }
        }
    }

    println!("{:?}", items);

    let instructions = vec![
        "east",
        "take food ration",
        "west",
        "south",
        "south",
        "take candy cane",
        "north",
        "north",
        //"north", "take wreath", "south",
        //"east", "south", "east", "south", "take hypercube", "north", "west", "north", "west",

        //"north", "east", "east", "east", "take weather machine", "west", "west", "west", "south",
        "east",
        "south",
        "east",
        "south",
        "east",
        "take space law space brochure",
        "west",
        "north",
        "west",
        "north",
        "west",
        //"south", "west", "take prime number", "east", "north",
        "south",
        "west",
        "west",
        "take astrolabe",
        "east",
        "east",
        "north",
        "east",
        "south",
        "east",
        "south",
        "east",
        "north",
    ];

    let mut prog = Program::new(input);
    for instruction in instructions {
        prog.input.extend(
            instruction
                .chars()
                .map(|x| (x as u8) as isize)
                .collect::<Vec<isize>>(),
        );
        prog.input.push_back(10);
        run_program(&mut prog);
    }
    prog.input.extend(
        "west\n"
            .chars()
            .map(|x| (x as u8) as isize)
            .collect::<Vec<isize>>(),
    );
    let output = run_program(&mut prog);

    println!("{}", &output);
    Regex::new("by typing (.*) on the keypad")
        .unwrap()
        .captures(&output)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap()
}

fn run_program(prog: &mut Program) -> String {
    prog.execute();
    let mut result = String::new();
    while let Some(output) = prog.output.pop_front() {
        result.push(output as u8 as char);
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::advent::intcode::parse_input;

    use super::*;

    #[test]
    fn check_step1() {
        assert_eq!(execute(parse_input("25")), 2415919488);
    }
}
