pub fn step1() -> usize {
    let mut input = parse_input();
    input[1] = 12;
    input[2] = 2;
    execute_program(input)[0]
}

pub fn step2(expected_result: usize) -> Option<usize> {
    let mut input = parse_input();

    for noun in 0..=99 {
        for verb in 0..=99 {
            input[1] = noun;
            input[2] = verb;
            let result = execute_program(input.clone())[0];
            if result == expected_result {
                return Some(100 * noun + verb);
            }
        }
    }
    None
}

fn parse_input() -> Vec<usize> {
    crate::read_file("src/advent/day02/input.txt")
        .split(',')
        .map(|number| number
            .parse::<usize>()
            .expect("input should contain only numbers"))
        .collect()
}

fn execute_program(mut input: Vec<usize>) -> Vec<usize> {
    for idx in 0..input.len() / 4 {
        let output_idx = input[4 * idx + 3];
        let noun = input[input[4 * idx + 1]];
        let verb = input[input[4 * idx + 2]];
        match input[4 * idx] {
            1 => input[output_idx] = noun + verb,
            2 => input[output_idx] = noun * verb,
            99 => return input,
            _ => panic!("operator value not expected")
        }
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_addition() {
        assert_eq!(execute_program(vec!(1, 0, 0, 0, 99)), &[2, 0, 0, 0, 99]);
    }

    #[test]
    fn simple_multiplication() {
        assert_eq!(execute_program(vec!(2, 3, 0, 3, 99)), &[2, 3, 0, 6, 99]);
    }

    #[test]
    fn program_ends_when_there_is_99() {
        assert_eq!(execute_program(vec!(2, 4, 4, 5, 99, 0)), &[2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn if_99_is_erased_program_doesnt_end() {
        assert_eq!(execute_program(vec!(1, 1, 1, 4, 99, 5, 6, 0, 99)), &[30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn check_step1() {
        assert_eq!(step1(), 5534943);
    }

    #[test]
    fn check_step2() {
        assert_eq!(step2(19690720), Some(7603));
    }

    #[test]
    fn impossible_expected_result_as_input() {
        assert_eq!(step2(0), None);
    }
}