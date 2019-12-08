#[cfg(test)]
mod tests {
    use crate::advent::intcode::*;

    #[test]
    fn example_1() {
        let mut program = Program::new(vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ]);
        program.execute();
        assert_eq!(program.output.pop_back(), Some(99));
    }

    #[test]
    fn example_2() {
        let mut program = Program::new(vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0]);
        program.execute();
        assert_eq!(program.output.pop_back(), Some(1219070632396864));
    }

    #[test]
    fn example_3() {
        let mut program = Program::new(vec![104, 1125899906842624, 99]);
        program.execute();
        assert_eq!(program.output.pop_back(), Some(1125899906842624));
    }

    #[test]
    fn check_step1() {
        let mut program = Program::new(parse_input("09"));
        program.input.push_back(1);
        program.execute();
        assert_eq!(program.output.pop_back(), Some(4261108180));
    }

    #[test]
    fn check_step2() {
        let mut program = Program::new(parse_input("09"));
        program.input.push_back(2);
        program.execute();
        assert_eq!(program.output.pop_back(), Some(77944));
    }
}
