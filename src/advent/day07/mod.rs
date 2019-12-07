use crate::advent::intcode::Program;
use crate::advent::intcode::ProgramState;
use permutohedron::Heap;

pub fn find_best_value(input: Vec<isize>, feedback: bool) -> isize {
    let mut result = 0;
    let mut settings_value = if feedback {
        vec![5isize, 6, 7, 8, 9]
    } else {
        vec![0isize, 1, 2, 3, 4]
    };
    let settings = Heap::new(&mut settings_value);
    for setting in settings {
        result = result.max(amplifier_controller(input.clone(), &setting, feedback));
    }
    result
}

fn amplifier_controller(input: Vec<isize>, settings: &[isize], feedback: bool) -> isize {
    let mut amps = Vec::new();
    for &setting in settings {
        let mut amp = Program::new(input.clone());
        amp.input.push_back(setting);
        amps.push(amp);
    }
    amps[0].input.push_back(0);
    amps[0].execute();

    let mut exited = 0;
    let mut idx = 1;
    while if feedback {
        exited < settings.len()
    } else {
        idx != 0
    } {
        if amps[idx].state != ProgramState::Halted {
            while let Some(input) = amps[if idx == 0 {
                settings.len() - 1
            } else {
                idx - 1
            }]
            .output
            .pop_front()
            {
                amps[idx].input.push_back(input);
            }
            amps[idx].execute();
            if amps[idx].state == ProgramState::Halted {
                exited += 1;
            }
        }
        idx = (idx + 1) % settings.len();
    }
    *amps
        .iter()
        .map(|amp| &amp.output)
        .find(|output| !output.is_empty())
        .map(|output| output.get(0).unwrap())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::advent::intcode::parse_input;

    #[test]
    fn check_amplifier_1() {
        assert_eq!(
            amplifier_controller(
                vec!(3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0),
                &vec!(4, 3, 2, 1, 0),
                false,
            ),
            43210
        );
    }

    #[test]
    fn check_amplifier_2() {
        assert_eq!(
            amplifier_controller(
                vec!(
                    3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23,
                    23, 4, 23, 99, 0, 0
                ),
                &vec!(0, 1, 2, 3, 4),
                false,
            ),
            54321
        );
    }

    #[test]
    fn check_amplifier_3() {
        assert_eq!(
            amplifier_controller(
                vec!(
                    3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7,
                    33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
                ),
                &vec!(1, 0, 4, 3, 2),
                false,
            ),
            65210
        );
    }

    #[test]
    fn check_best_value_1() {
        assert_eq!(
            find_best_value(
                vec!(3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0),
                false,
            ),
            43210
        );
    }

    #[test]
    fn check_best_value_2() {
        assert_eq!(
            find_best_value(
                vec!(
                    3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23,
                    23, 4, 23, 99, 0, 0
                ),
                false,
            ),
            54321
        );
    }

    #[test]
    fn check_best_value_3() {
        assert_eq!(
            find_best_value(
                vec!(
                    3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7,
                    33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
                ),
                false,
            ),
            65210
        );
    }

    #[test]
    fn check_step1() {
        assert_eq!(find_best_value(parse_input("07"), false), 47064);
    }

    #[test]
    fn check_amplifier_with_feedback_1() {
        assert_eq!(
            amplifier_controller(
                vec!(
                    3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001,
                    28, -1, 28, 1005, 28, 6, 99, 0, 0, 5
                ),
                &vec!(9, 8, 7, 6, 5),
                true,
            ),
            139629729
        );
    }

    #[test]
    fn check_amplifier_with_feedback_2() {
        assert_eq!(
            amplifier_controller(
                vec!(
                    3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26,
                    1001, 54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55,
                    2, 53, 55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
                ),
                &vec!(9, 7, 8, 5, 6),
                true,
            ),
            18216
        );
    }

    #[test]
    fn check_best_value_with_feedback_1() {
        assert_eq!(
            find_best_value(
                vec!(
                    3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001,
                    28, -1, 28, 1005, 28, 6, 99, 0, 0, 5
                ),
                true,
            ),
            139629729
        );
    }

    #[test]
    fn check_best_value_with_feedback_2() {
        assert_eq!(
            find_best_value(
                vec!(
                    3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26,
                    1001, 54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55,
                    2, 53, 55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
                ),
                true,
            ),
            18216
        );
    }

    #[test]
    fn check_step2() {
        assert_eq!(find_best_value(parse_input("07"), true), 4248984);
    }
}
