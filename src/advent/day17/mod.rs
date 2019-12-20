use std::collections::HashSet;

use regex::Regex;

use crate::advent::geometry::Map;
use crate::advent::geometry::Point;
use crate::advent::geometry::DOWN;
use crate::advent::geometry::LEFT;
use crate::advent::geometry::RIGHT;
use crate::advent::geometry::UP;
use crate::advent::intcode::Program;

pub fn execute(input: Vec<isize>) -> Result<(isize, isize), &'static str> {
    let mut map = build_map(input.clone());

    let step1 = map
        .values
        .iter()
        .filter(|(_, &v)| v == '#')
        .filter(|(&point, _)| {
            [UP, RIGHT, DOWN, LEFT]
                .iter()
                .all(|&dir| *map.values.get(&(point + dir)).unwrap_or(&'?') == '#')
        })
        .map(|(&p, _)| p.x * p.y)
        .sum();

    // Execute step 2
    let mut input = input;
    input[0] = 2;
    let mut program = Program::new(input);

    let instructions = compute_instructions(&mut map);
    let (main_routine, functions) = extract_patterns(&instructions.join(""));

    // Fill parameters
    program.input.extend(string_to_ascii(main_routine));
    program.input.push_back('\n' as isize);
    functions
        .iter()
        .for_each(|f| program.input.extend(format_function(f)));

    let debug = "n\n".chars().map(|c| c as isize).collect::<Vec<isize>>();

    program.input.extend(debug);
    program.execute();

    Ok((step1, program.output.pop_back().unwrap()))
}

fn build_map(input: Vec<isize>) -> Map<char> {
    let mut program = Program::new(input);
    program.execute();
    let mut map = Map::new(
        |f, v| write!(f, "{}", v.unwrap_or(&'?')),
        |f, _| writeln!(f),
    );
    let mut y = 0;
    let mut x = 0;
    while let Some(item) = program.output.pop_front() {
        if item == 10 {
            y += 1;
            x = 0;
        } else {
            map.values.insert(Point::new(x, y), item as u8 as char);
            x += 1;
        }
    }
    map
}

fn compute_instructions(map: &mut Map<char>) -> Vec<String> {
    let mut instructions = Vec::new();

    let mut robot = *map
        .values
        .iter()
        .find(|(_, &value)| value == '^')
        .unwrap()
        .0;
    let mut direction = UP;

    let mut visited = HashSet::new();
    let mut current_instruction = 0;
    let nb_part = map.values.values().filter(|&v| *v == '#').count();
    while nb_part > visited.len() {
        if *map.values.get(&(robot + direction)).unwrap_or(&'?') != '#' {
            if current_instruction != 0 {
                instructions.push(current_instruction.to_string());
            }
            current_instruction = 0;
            if *map
                .values
                .get(&(robot + direction.rotate(true)))
                .unwrap_or(&'?')
                == '#'
            {
                direction = direction.rotate(true);
                instructions.push("R".to_string());
            } else if *map
                .values
                .get(&(robot + direction.rotate(false)))
                .unwrap_or(&'?')
                == '#'
            {
                direction = direction.rotate(false);
                instructions.push("L".to_string());
            } else {
                panic!("No path")
            }
        }
        current_instruction += 1;
        robot += direction;
        visited.insert(robot);
    }
    instructions.push(current_instruction.to_string());
    instructions
}

fn string_to_ascii(input: String) -> Vec<isize> {
    input.chars().map(|c| c as isize).collect::<Vec<isize>>()
}

fn format_function(input: &str) -> Vec<isize> {
    let mut formated = Regex::new("(L|R)").unwrap().replace_all(&input, ",$1,")[1..].to_string();
    formated.push('\n');
    string_to_ascii(formated)
}

fn extract_patterns(input: &str) -> (String, Vec<String>) {
    let mut main_routine = input.to_string();
    let mut functions = Vec::new();

    let mut input_rest = input.to_string();
    let mut fn_idx = 'A';

    while !input_rest.is_empty() {
        let mut size = 1;
        loop {
            let pattern = &input_rest[..size];
            if Regex::new(pattern).unwrap().find_iter(&input_rest).count() < 2 {
                break;
            }
            size += 1;
        }
        let pattern = input_rest[0..size - 1].to_owned();
        let re = Regex::new(&pattern).unwrap();
        main_routine = re
            .replace_all(&main_routine, format!("{},", fn_idx).as_str())
            .to_string();
        input_rest = re.replace_all(&input_rest, "").to_string();
        functions.push(pattern);
        fn_idx = (fn_idx as u8 + 1) as char;
    }
    main_routine.remove(main_routine.len() - 1);
    (main_routine, functions)
}

#[cfg(test)]
mod tests {
    use crate::advent::intcode::parse_input;

    use super::*;

    #[test]
    fn check_step1() {
        assert_eq!(execute(parse_input("17")), Ok((5620, 768_115)));
    }
}
