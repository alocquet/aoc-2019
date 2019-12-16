use std::ops::Range;

pub fn step1(input: &[u8], range: Range<usize>) -> usize {
    let mut result = input.to_owned();
    for _ in 0..100 {
        result = fft(&result);
    }
    vector_to_decimal(&result[range])
}

fn vector_to_decimal(input: &[u8]) -> usize {
    input
        .iter()
        .fold(0, |acc, digit| acc * 10 + *digit as usize)
}

#[cfg_attr(tarpaulin, skip)]
pub fn step2(input: &[u8]) -> usize {
    let real_signal: Vec<u8> = input
        .iter()
        .cycle()
        .take(input.len() * 10000)
        .copied()
        .collect();
    let offset = vector_to_decimal(&input[0..7]);
    step1(&real_signal, offset..offset + 8)
}

fn fft(input: &[u8]) -> Vec<u8> {
    let size = input.len();
    let mut result = Vec::with_capacity(size);
    let mut partial_sums: Vec<isize> = Vec::with_capacity(size);

    let mut sum: isize = 0;
    partial_sums.push(0);

    for &digit in input {
        sum += digit as isize;
        partial_sums.push(sum);
    }

    for step in 0..size {
        let mut value: isize = 0;

        for start in (step..size).step_by(4 * (step + 1)) {
            let end = size.min(start + step + 1);
            value += partial_sums[end] - partial_sums[start];
        }
        for start in (3 * (step + 1) - 1..size).step_by(4 * (step + 1)) {
            let end = size.min(start + step + 1);
            value -= partial_sums[end] - partial_sums[start];
        }

        result.push((value.abs() % 10) as u8);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file;

    fn parse_input() -> Vec<u8> {
        read_file("src/advent/day16/input.txt")
            .chars()
            .map(|c| c as u8 - 48)
            .collect()
    }

    #[test]
    fn fft_on_example_1() {
        assert_eq!(fft(&[1, 2, 3, 4, 5, 6, 7, 8]), vec!(4, 8, 2, 2, 6, 1, 5, 8));
    }

    #[test]
    fn step1_on_example() {
        assert_eq!(
            step1(
                &[
                    8, 0, 8, 7, 1, 2, 2, 4, 5, 8, 5, 9, 1, 4, 5, 4, 6, 6, 1, 9, 0, 8, 3, 2, 1, 8,
                    6, 4, 5, 5, 9, 5
                ],
                0..8
            ),
            24176176
        );
    }

    #[test]
    #[ignore]
    fn step2_on_example() {
        assert_eq!(
            step2(&[
                0, 3, 0, 3, 6, 7, 3, 2, 5, 7, 7, 2, 1, 2, 9, 4, 4, 0, 6, 3, 4, 9, 1, 5, 6, 5, 4, 7,
                4, 6, 6, 4
            ]),
            84462026
        );
    }

    #[test]
    fn check_step1() {
        assert_eq!(step1(&parse_input(), 0..8), 44098263);
    }

    #[test]
    #[ignore]
    fn check_step2() {
        assert_eq!(step2(&parse_input()), 12482168);
    }
}
