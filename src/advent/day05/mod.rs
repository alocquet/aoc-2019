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

fn parse_input() -> Vec<isize> {
    crate::read_file("src/advent/day05/input.txt")
        .split(',')
        .map(|number| {
            number
                .parse::<isize>()
                .expect("input should contain only numbers")
        })
        .collect()
}

fn get_value(input: &[isize], idx: usize, immediate: bool) -> isize {
    if immediate {
        input[idx]
    } else {
        input[input[idx] as usize]
    }
}

pub fn execute_program(input_value: isize) -> Result<isize, isize> {
    let mut result = 0;
    let mut input = parse_input();
    let mut idx = 0;

    while idx < input.len() {
        let operation = Operation::from(input[idx]);
        match operation.code {
            OperationType::Add => {
                let output_idx = input[idx + 3] as usize;
                input[output_idx] = get_value(&input, idx + 1, operation.immediate.0)
                    + get_value(&input, idx + 2, operation.immediate.1);
                idx += 2;
            }
            OperationType::Mul => {
                let output_idx = input[idx + 3] as usize;
                input[output_idx] = get_value(&input, idx + 1, operation.immediate.0)
                    * get_value(&input, idx + 2, operation.immediate.1);
                idx += 2;
            }
            OperationType::Set => {
                let output_idx = input[idx + 1] as usize;
                input[output_idx] = input_value;
            }
            OperationType::Print => {
                if result != 0 {
                    return Err(result);
                }
                result = get_value(&input, idx + 1, operation.immediate.0);
            }
            OperationType::JumpIfTrue => {
                if get_value(&input, idx + 1, operation.immediate.0) != 0 {
                    idx = get_value(&input, idx + 2, operation.immediate.1) as usize - 2;
                } else {
                    idx += 1;
                }
            }
            OperationType::JumpIfFalse => {
                if get_value(&input, idx + 1, operation.immediate.0) == 0 {
                    idx = get_value(&input, idx + 2, operation.immediate.1) as usize - 2;
                } else {
                    idx += 1;
                }
            }
            OperationType::LessThan => {
                let output_idx = input[idx + 3] as usize;
                input[output_idx] = if get_value(&input, idx + 1, operation.immediate.0)
                    < get_value(&input, idx + 2, operation.immediate.1)
                {
                    1
                } else {
                    0
                };
                idx += 2;
            }
            OperationType::Equals => {
                let output_idx = input[idx + 3] as usize;
                input[output_idx] = if get_value(&input, idx + 1, operation.immediate.0)
                    == get_value(&input, idx + 2, operation.immediate.1)
                {
                    1
                } else {
                    0
                };
                idx += 2;
            }
            OperationType::Exit => return Ok(result),
        }
        idx += 2;
    }
    Err(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_step1() {
        assert_eq!(execute_program(1), Ok(6745903));
    }

    #[test]
    fn check_step2() {
        assert_eq!(execute_program(5), Ok(9168267));
    }
}
