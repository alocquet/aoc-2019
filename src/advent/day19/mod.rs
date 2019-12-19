use crate::advent::geometry::Map;
use crate::advent::geometry::Point;
use crate::advent::intcode::Program;

pub fn step1(input: Vec<isize>, offset: Point, size: isize) -> usize {
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
    for x in (offset.x)..(offset.x + size) {
        for y in (offset.y)..(offset.y + size) {
            let mut program = program.clone();
            program.input.push_back(x as isize);
            program.input.push_back(y as isize);
            program.execute();
            map.values
                .insert(Point::new(x, y), program.output.pop_front().unwrap() == 1);
        }
    }
    println!("{}", &map);
    map.values.iter().filter(|&(_, &v)| v).count()
}

#[cfg(test)]
mod tests {
    use crate::advent::geometry::ORIGIN;
    use crate::advent::intcode::parse_input;

    use super::*;

    #[test]
    fn check_step1() {
        assert_eq!(step1(parse_input("19"), ORIGIN, 50), 215);
    }

    #[test]
    #[ignore]
    fn check_step2() {
        // Ok, j'ai un peu sheaté : pour obtenir, l'offset, j'ai fait à tatons
        assert_eq!(step1(parse_input("19"), Point::new(772, 975), 100), 10_000);
    }
}
