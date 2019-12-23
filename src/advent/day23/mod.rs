use crate::advent::intcode::Program;

pub fn execute(input: Vec<isize>, stop_on_first_nat_value: bool) -> Option<isize> {
    let mut computers: Vec<Program> = (0..50)
        .map(|id| {
            let mut program = Program::new(input.clone());
            program.input.push_back(id);
            program
        })
        .collect();

    let mut nat = None;
    let mut previous_nat = None;

    loop {
        let mut idle = true;
        for id in 0..50 {
            if computers[id].input.is_empty() {
                computers[id].input.push_back(-1);
            } else {
                idle = false;
            }
            computers[id].execute();
            while computers[id].output.len() >= 3 {
                idle = false;
                let dest = computers[id].output.pop_front().unwrap();
                let x = computers[id].output.pop_front().unwrap();
                let y = computers[id].output.pop_front().unwrap();
                if dest == 255 {
                    if stop_on_first_nat_value {
                        return Some(y);
                    }
                    nat = Some((x, y));
                } else {
                    computers[dest as usize].input.push_back(x);
                    computers[dest as usize].input.push_back(y);
                }
            }
        }
        if idle {
            if previous_nat.is_some() && previous_nat == nat {
                return Some(nat.unwrap().1);
            }
            computers[0].input.push_back(nat.unwrap().0);
            computers[0].input.push_back(nat.unwrap().1);
            previous_nat = nat;
            nat = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::advent::intcode::parse_input;

    use super::*;

    #[test]
    fn check_step1() {
        assert_eq!(execute(parse_input("23"), true), Some(20225));
    }

    #[test]
    fn check_step2() {
        assert_eq!(execute(parse_input("23"), false), Some(14348));
    }
}
