use std::collections::vec_deque::VecDeque;

use crate::advent::intcode::Program;

#[derive(Clone, Debug)]
struct Instructions {
    delegate: Vec<String>,
    used_registers: Vec<char>,
}

impl Instructions {
    pub fn add(&self, instruction: String, reg: char) -> Self {
        let mut next = self.clone();
        next.delegate.push(instruction);
        next.used_registers.push(reg);
        next
    }
}

pub fn find_prog(input: Vec<isize>, ro_registers: Vec<char>, mode: &str) -> Option<isize> {
    let mut queue = VecDeque::new();
    for reg in &ro_registers {
        queue.push_back(Instructions {
            delegate: vec![format!("NOT {} J\n", reg)],
            used_registers: vec![*reg],
        });
    }

    while let Some(instructions) = queue.pop_front() {
        let result = execute(
            input.clone(),
            instructions
                .add(format!("NOT A T\nOR T J\nAND D J\n{}", mode), '.')
                .delegate,
        );

        if result != Some(10) {
            println!("{:?}", instructions);
            return result;
        }

        // search other instructions
        for reg in ro_registers
            .iter()
            .filter(|reg| !instructions.used_registers.contains(reg))
        {
            // AND reg J
            queue.push_back(instructions.add(format!("AND {} J\n", reg), *reg));
            // NOT reg T
            // AND T J
            queue.push_back(instructions.add(format!("NOT {} T\nAND T J\n", reg), *reg));
            // OR reg J
            queue.push_back(instructions.add(format!("OR {} J\n", reg), *reg));
            // NOT reg T
            // OR T J
            queue.push_back(instructions.add(format!("NOT {} T\nOR T J\n", reg), *reg));
        }
    }
    None
}

pub fn execute(input: Vec<isize>, instructions: Vec<String>) -> Option<isize> {
    let mut program = Program::new(input);
    program.execute();

    instructions
        .iter()
        .for_each(|instruction| fill_input(&mut program, &instruction));

    program.execute();

    program.output.pop_back()
}

fn fill_input(program: &mut Program, instruction: &str) {
    program
        .input
        .extend(instruction.chars().map(|c| c as u8 as isize));
}

#[cfg(test)]
mod tests {
    use crate::advent::intcode::parse_input;

    use super::*;

    #[test]
    fn check_step1() {
        assert_eq!(
            find_prog(parse_input("21"), vec!('A', 'C'), "WALK\n"),
            Some(19350258)
        );
    }

    #[test]
    #[ignore]
    fn check_step2() {
        assert_eq!(
            find_prog(
                parse_input("21"),
                vec!('B', 'C', 'E', 'F', 'G', 'H', 'I'),
                "RUN\n"
            ),
            Some(1142627861)
        );
    }
}
