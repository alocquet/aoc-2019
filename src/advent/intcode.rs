use std::collections::vec_deque::VecDeque;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug)]
pub enum ProgramState {
    Running,
    Waiting,
    Halted,
}

#[derive(Debug)]
pub struct Program {
    operations: HashMap<usize, isize>,
    idx: usize,
    relative_base: isize,
    pub input: VecDeque<isize>,
    pub output: VecDeque<isize>,
    pub state: ProgramState,
}

struct OperationMode { modes: (u8, u8, u8) }

struct Add { op_modes: OperationMode }

struct Mul { op_modes: OperationMode }

struct Set { op_modes: OperationMode }

struct Print { op_modes: OperationMode }

struct JumpIf { op_modes: OperationMode, test: bool }

struct LessThan { op_modes: OperationMode }

struct Equals { op_modes: OperationMode }

struct RelativeBaseOffset { op_modes: OperationMode }

struct Exit {}

trait Operation {
    fn execute(&self, prog: &mut Program);
}


fn parse_ope(input: isize) -> Box<dyn Operation> {
    let op_modes = OperationMode {
        modes: (
            (input / 100 % 10) as u8,
            (input / 1000 % 10) as u8,
            (input / 10000 % 10) as u8,
        )
    };
    match input % 100 {
        1 => Box::new(Add { op_modes }),
        2 => Box::new(Mul { op_modes }),
        3 => Box::new(Set { op_modes }),
        4 => Box::new(Print { op_modes }),
        5 => Box::new(JumpIf { op_modes, test: true }),
        6 => Box::new(JumpIf { op_modes, test: false }),
        7 => Box::new(LessThan { op_modes }),
        8 => Box::new(Equals { op_modes }),
        9 => Box::new(RelativeBaseOffset { op_modes }),
        99 => Box::new(Exit {}),
        _ => panic!("Bad operator")
    }
}

impl Operation for Add {
    fn execute(&self, prog: &mut Program) {
        let output_idx = prog.get_operation_idx(prog.idx + 3, self.op_modes.modes.2);
        prog.operations.insert(
            output_idx,
            prog.get_value(prog.idx + 1, self.op_modes.modes.0)
                + prog.get_value(prog.idx + 2, self.op_modes.modes.1),
        );
        prog.idx += 4;
    }
}

impl Operation for Mul {
    fn execute(&self, prog: &mut Program) {
        let output_idx = prog.get_operation_idx(prog.idx + 3, self.op_modes.modes.2);
        prog.operations.insert(
            output_idx,
            prog.get_value(prog.idx + 1, self.op_modes.modes.0)
                * prog.get_value(prog.idx + 2, self.op_modes.modes.1),
        );
        prog.idx += 4;
    }
}

impl Operation for Set {
    fn execute(&self, prog: &mut Program) {
        if let Some(input) = prog.input.pop_front() {
            let output_idx =
                prog.get_operation_idx(prog.idx + 1, self.op_modes.modes.0);
            prog.operations.insert(output_idx, input);
            prog.idx += 2;
        } else {
            prog.state = ProgramState::Waiting;
        }
    }
}

impl Operation for Print {
    fn execute(&self, prog: &mut Program) {
        prog.output
            .push_back(prog.get_value(prog.idx + 1, self.op_modes.modes.0));
        prog.idx += 2;
    }
}

impl Operation for JumpIf {
    fn execute(&self, prog: &mut Program) {
        let value = prog.get_value(prog.idx + 1, self.op_modes.modes.0);
        if self.test && value != 0 || !self.test && value == 0 {
            prog.idx = prog.get_value(prog.idx + 2, self.op_modes.modes.1) as usize;
        } else {
            prog.idx += 3;
        }
    }
}

impl Operation for LessThan {
    fn execute(&self, prog: &mut Program) {
        let output_idx = prog.get_operation_idx(prog.idx + 3, self.op_modes.modes.2);
        prog.operations.insert(
            output_idx,
            if prog.get_value(prog.idx + 1, self.op_modes.modes.0)
                < prog.get_value(prog.idx + 2, self.op_modes.modes.1)
                {
                    1
                } else {
                0
            },
        );
        prog.idx += 4;
    }
}

impl Operation for Equals {
    fn execute(&self, prog: &mut Program) {
        let output_idx = prog.get_operation_idx(prog.idx + 3, self.op_modes.modes.2);
        prog.operations.insert(
            output_idx,
            if prog.get_value(prog.idx + 1, self.op_modes.modes.0) == prog.get_value(prog.idx + 2, self.op_modes.modes.1) {
                1
            } else {
                0
            });
        prog.idx += 4;
    }
}

impl Operation for RelativeBaseOffset {
    fn execute(&self, prog: &mut Program) {
        prog.relative_base += prog.get_value(prog.idx + 1, self.op_modes.modes.0);
        prog.idx += 2;
    }
}

impl Operation for Exit {
    fn execute(&self, prog: &mut Program) {
        prog.state = ProgramState::Halted
    }
}

impl Program {
    pub fn new(opes: Vec<isize>) -> Self {
        let operations = opes
            .iter()
            .cloned()
            .enumerate()
            .collect::<HashMap<usize, isize>>();
        Program {
            idx: 0,
            relative_base: 0,
            operations,
            input: VecDeque::new(),
            output: VecDeque::new(),
            state: ProgramState::Running,
        }
    }

    pub fn execute(&mut self) {
        self.state = ProgramState::Running;
        while self.operations.contains_key(&self.idx) && self.state == ProgramState::Running {
            let ope = parse_ope(*self.operations.get(&self.idx).unwrap());
            ope.execute(self)
        }
    }

    fn get_value(&self, idx: usize, op_modes: u8) -> isize {
        *self
            .operations
            .get(&self.get_operation_idx(idx, op_modes))
            .unwrap_or(&0)
    }

    fn get_operation_idx(&self, idx: usize, op_modes: u8) -> usize {
        match op_modes {
            0 => (*self.operations.get(&idx).unwrap() as usize),
            1 => idx,
            2 => ((self.relative_base + self.operations.get(&idx).unwrap()) as usize),
            _ => panic!("bad op_modes value : {}", op_modes),
        }
    }
}

pub fn parse_input(day: &str) -> Vec<isize> {
    crate::read_file(&format!("src/advent/day{}/input.txt", day))
        .split(',')
        .map(|number| {
            number
                .parse::<isize>()
                .expect("input should contain only numbers")
        })
        .collect()
}
