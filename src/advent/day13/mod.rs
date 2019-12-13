use crate::advent::geometry::Map;
use crate::advent::geometry::Point;
use crate::advent::geometry::ORIGIN;
use crate::advent::intcode::Program;
use crate::advent::intcode::ProgramState;

const SCORE_FLAG: Point = Point { x: -1, y: 0 };

pub fn execute(mut input: Vec<isize>, display: bool) -> (usize, isize) {
    input[0] = 2; // 2 quarters
    let mut program = Program::new(input);
    let mut map = Map::new(cell_formatter);
    let mut ball = ORIGIN;
    let mut paddle = ORIGIN;
    let mut score = 0;
    let mut nb_blocks = None;

    let mut first_display = true;

    while program.state != ProgramState::Halted {
        program.execute();
        while program.output.len() > 2 {
            let point = Point::new(
                program.output.pop_front().unwrap(),
                program.output.pop_front().unwrap(),
            );
            let value = program.output.pop_front().unwrap();
            if point == SCORE_FLAG {
                score = value;
            } else {
                map.values.insert(point, value);
                if value == 4 {
                    ball = point;
                } else if value == 3 {
                    paddle = point;
                }
            }
        }
        if nb_blocks.is_none() {
            nb_blocks = Some(map.values.values().filter(|&&cell| cell == 2).count());
        }
        program.input.push_back(ball.x.cmp(&paddle.x) as isize);
        first_display = display_game(display, &map, score, first_display);
    }
    (nb_blocks.unwrap(), score)
}

// Disable display coverage to avoid long tests
#[cfg_attr(tarpaulin, skip)]
fn display_game(display: bool, map: &Map<isize>, score: isize, first_display: bool) -> bool {
    if display {
        if !first_display {
            let mut t = term::stdout().expect("should have a term");
            for _ in 0..map.height() + 3 {
                t.cursor_up().expect("should be ok");
            }
        }
        println!("{}", &map);
        println!("score: {}", score);
    }
    false
}

// Disable display coverage to avoid long tests
#[cfg_attr(tarpaulin, skip)]
fn cell_formatter(value: Option<&isize>) -> char {
    match value {
        Some(0) => ' ',
        Some(1) => '█',
        Some(2) => '#',
        Some(3) => '▂',
        Some(4) => '*',
        _ => ' ',
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::advent::intcode::parse_input;

    #[test]
    fn test_with_input() {
        assert_eq!(execute(parse_input("13"), false), (253, 12263));
    }

    #[test]
    #[ignore]
    fn test_with_input_debug() {
        assert_eq!(execute(parse_input("13"), true), (253, 12263));
    }
}
