use std::collections::vec_deque::VecDeque;

#[derive(Eq, PartialEq)]
pub enum ProgramState {
    Running,
    Waiting,
    Halted,
}

pub struct Program {
    operations: Vec<isize>,
    idx: usize,
    pub input: VecDeque<isize>,
    pub output: VecDeque<isize>,
    pub state: ProgramState,
}

#[derive(FromPrimitive)]
enum OperationType {
    Add = 1,
    Mul,
    Set,
    Print,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Exit = 99,
}

struct Operation {
    code: OperationType,
    immediate: (bool, bool),
}

impl Operation {
    fn from(input: isize) -> Self {
        Operation {
            code: num::FromPrimitive::from_isize(input % 100).expect("bad operation code"),
            immediate: (input / 100 % 10 == 1, input / 1000 % 100 == 1),
        }
    }
}

fn get_value(input: &[isize], idx: usize, immediate: bool) -> isize {
    if immediate {
        input[idx]
    } else {
        input[input[idx] as usize]
    }
}

impl Program {
    pub fn new(operations: Vec<isize>) -> Self {
        Program {
            idx: 0,
            operations,
            input: VecDeque::new(),
            output: VecDeque::new(),
            state: ProgramState::Running,
        }
    }

    pub fn execute(&mut self) {
        self.state = ProgramState::Running;
        while self.idx < self.operations.len() && self.state == ProgramState::Running {
            let operation = Operation::from(self.operations[self.idx]);
            match operation.code {
                OperationType::Add => {
                    let output_idx = self.operations[self.idx + 3] as usize;
                    self.operations[output_idx] =
                        get_value(&self.operations, self.idx + 1, operation.immediate.0)
                            + get_value(&self.operations, self.idx + 2, operation.immediate.1);
                    self.idx += 2;
                }
                OperationType::Mul => {
                    let output_idx = self.operations[self.idx + 3] as usize;
                    self.operations[output_idx] =
                        get_value(&self.operations, self.idx + 1, operation.immediate.0)
                            * get_value(&self.operations, self.idx + 2, operation.immediate.1);
                    self.idx += 2;
                }
                OperationType::Set => {
                    if let Some(input) = self.input.pop_front() {
                        let output_idx = self.operations[self.idx + 1] as usize;
                        self.operations[output_idx] = input;
                    } else {
                        self.state = ProgramState::Waiting;
                        break;
                    }
                }
                OperationType::Print => {
                    self.output.push_back(get_value(
                        &self.operations,
                        self.idx + 1,
                        operation.immediate.0,
                    ));
                }
                OperationType::JumpIfTrue => {
                    if get_value(&self.operations, self.idx + 1, operation.immediate.0) != 0 {
                        self.idx = get_value(&self.operations, self.idx + 2, operation.immediate.1)
                            as usize
                            - 2;
                    } else {
                        self.idx += 1;
                    }
                }
                OperationType::JumpIfFalse => {
                    if get_value(&self.operations, self.idx + 1, operation.immediate.0) == 0 {
                        self.idx = get_value(&self.operations, self.idx + 2, operation.immediate.1)
                            as usize
                            - 2;
                    } else {
                        self.idx += 1;
                    }
                }
                OperationType::LessThan => {
                    let output_idx = self.operations[self.idx + 3] as usize;
                    self.operations[output_idx] =
                        if get_value(&self.operations, self.idx + 1, operation.immediate.0)
                            < get_value(&self.operations, self.idx + 2, operation.immediate.1)
                        {
                            1
                        } else {
                            0
                        };
                    self.idx += 2;
                }
                OperationType::Equals => {
                    let output_idx = self.operations[self.idx + 3] as usize;
                    self.operations[output_idx] =
                        if get_value(&self.operations, self.idx + 1, operation.immediate.0)
                            == get_value(&self.operations, self.idx + 2, operation.immediate.1)
                        {
                            1
                        } else {
                            0
                        };
                    self.idx += 2;
                }
                OperationType::Exit => self.state = ProgramState::Halted,
            }
            self.idx += 2;
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
