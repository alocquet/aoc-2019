use crate::read_file;
use std::iter::once;

const BLACK: char = '0';
const WHITE: char = '1';
const TRANSPARENT: char = '2';

pub fn step1() -> usize {
    let input = parse_input();
    let layer = input
        .chunks(25 * 6)
        .min_by_key(|layer| count_nb_in_layer(layer, BLACK))
        .unwrap();

    count_nb_in_layer(layer, WHITE) * count_nb_in_layer(layer, TRANSPARENT)
}

pub fn step2() -> String {
    let input = parse_input();

    let mut result = vec![' '; 25 * 6];
    for layer in input.chunks(25 * 6).rev() {
        for (result_pixel, &layer_pixel) in result.iter_mut().zip(layer) {
            match layer_pixel {
                BLACK => {
                    *result_pixel = ' ';
                }
                WHITE => {
                    *result_pixel = '#';
                }
                _ => {}
            };
        }
    }
    result
        .chunks(25)
        .flat_map(|line| line.iter().chain(once(&'\n')))
        .collect()
}

fn parse_input() -> Vec<char> {
    read_file("src/advent/day08/input.txt").chars().collect()
}

fn count_nb_in_layer(layer: &[char], number: char) -> usize {
    layer.iter().filter(|&&n| number == n).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const STEP2: &str = "#### #### #    ###  #   #\n   # #    #    #  # #   #\n  #  ###  #    ###   # # \n #   #    #    #  #   #  \n#    #    #    #  #   #  \n#### #    #### ###    #  \n";

    #[test]
    fn check_step1() {
        assert_eq!(step1(), 2562);
    }

    #[test]
    fn check_step2() {
        assert_eq!(step2(), STEP2);
    }
}
