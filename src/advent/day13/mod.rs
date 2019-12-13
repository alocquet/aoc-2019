use crate::advent::geometry::Map;
use crate::advent::geometry::Point;
use crate::advent::geometry::ORIGIN;
use crate::advent::intcode::Program;
use crate::advent::intcode::ProgramState;
use std::fmt::Formatter;
use std::fmt::Error;
use termion::{color, style, cursor};

const SCORE_FLAG: Point = Point { x: -1, y: 0 };

pub fn execute(mut input: Vec<isize>, display: bool) -> (usize, isize) {
    input[0] = 2; // 2 quarters
    let mut program = Program::new(input);
    let mut map = Map::new(cell_formatter, newline_formatter);
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
    print!("{}", cursor::Show);
    (nb_blocks.unwrap(), score)
}

// Disable display coverage to avoid long tests
#[cfg_attr(tarpaulin, skip)]
fn display_game(display: bool, map: &Map<isize>, score: isize, first_display: bool) -> bool {
    if display {
        if first_display {
            print!("{}", cursor::Hide);
            let term_size = termion::terminal_size().ok();
            let term_height = term_size.map(|(_, h)| h - 2).unwrap();
            let term_width = term_size.map(|(w, _)| w - 2).unwrap();
            print!("{}", termion::clear::All);
            print!("{}{}", cursor::Goto((term_width - map.width() as u16 * 2) / 2, (term_height - map.height() as u16) / 2), cursor::Save);
        } else {
            print!("{}", cursor::Restore);
        }

        print!("{}{}  score: {}{}{}{}", style::Bold, color::Fg(color::Yellow), score, style::Reset, cursor::Restore, cursor::Down(1));
        println!("{}", &map);
    }
    false
}

// Disable display coverage to avoid long tests
#[cfg_attr(tarpaulin, skip)]
fn cell_formatter(f: &mut Formatter<'_>, value: Option<&isize>) -> Result<(), Error> {
    match value {
        Some(0) => write!(f, "  "),
        Some(1) => write!(f, "{}██{}", color::Fg(color::Black), color::Fg(color::Reset)),
        Some(2) => write!(f, "{}◀▶{}", color::Fg(color::LightRed), color::Fg(color::Reset)),
        Some(3) => write!(f, "{}▂▂{}", color::Fg(color::LightYellow), color::Fg(color::Reset)),
        Some(4) => write!(f, "⚾"),
        _ => write!(f, "  "),
    }
}

// Disable display coverage to avoid long tests
#[cfg_attr(tarpaulin, skip)]
fn newline_formatter(f: &mut Formatter<'_>, width: usize) -> Result<(), Error> {
    write!(f, "{}{}", cursor::Left((1 + width as u16) * 2), cursor::Down(1))
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
