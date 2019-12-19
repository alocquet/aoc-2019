use crate::advent::geometry::Map;
use crate::advent::geometry::Point;
use crate::advent::intcode::Program;

pub fn build_map(input: Vec<isize>) -> Map<bool> {
    let program = Program::new(input);
    let mut map = Map::new(
        |f, value| {
            write!(
                f,
                "{}",
                match value.unwrap_or(&false) {
                    true => "#",
                    false => ".",
                }
            )
        },
        |f, _| writeln!(f),
    );
    //for x in 400..400+100 {
    //for y in 450..450+100 {
    for x in 0..50 {
        for y in 0..50 {
            let mut program = program.clone();
            program.input.push_back(x as isize);
            program.input.push_back(y as isize);
            program.execute();
            map.values
                .insert(Point::new(x, y), program.output.pop_front().unwrap() == 1);
        }
    }
    map
}

pub fn step1(map: Map<bool>) -> usize {
    println!("{}", &map);
    map.values.iter().filter(|&(_, &v)| v).count()
}

#[cfg(test)]
mod tests {
    use crate::advent::intcode::parse_input;

    use super::*;

    #[test]
    fn check_step1() {
        assert_eq!(step1(build_map(parse_input("19"))), 215);
    }
}
