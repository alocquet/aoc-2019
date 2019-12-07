#[cfg(test)]
mod tests {
    use crate::advent::intcode::*;

    #[test]
    fn check_step1() {
        let mut program = Program::new(parse_input("05"));
        program.input.push_back(1);
        program.execute();
        assert_eq!(program.output.pop_back(), Some(6745903));
    }

    #[test]
    fn check_step2() {
        let mut program = Program::new(parse_input("05"));
        program.input.push_back(5);
        program.execute();
        assert_eq!(program.output.pop_back(), Some(9168267));
    }
}
