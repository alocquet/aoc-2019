use digits_iterator::*;

pub fn step1() -> usize {
    count_valid(false)
}

pub fn step2() -> usize {
    count_valid(true)
}

pub fn count_valid(strict_double: bool) -> usize {
    (130_254..678_275)
        .filter(|&number| is_valid(number, strict_double))
        .count()
}

fn match_double_pattern(nb_same_digit: usize, strict_double: bool) -> bool {
    nb_same_digit == 2 || !strict_double && nb_same_digit > 2
}

fn is_valid(number: usize, strict_double: bool) -> bool {
    let mut previous = 0;
    let mut nb_same_digit = 1;
    let mut has_double = false;
    for digit in number.digits() {
        if digit < previous {
            return false;
        }
        if digit == previous {
            nb_same_digit += 1;
        } else {
            has_double = has_double || match_double_pattern(nb_same_digit, strict_double);
            nb_same_digit = 1;
        }
        previous = digit;
    }
    has_double || match_double_pattern(nb_same_digit, strict_double)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_step1() {
        assert_eq!(step1(), 2090);
    }

    #[test]
    fn check_step2() {
        assert_eq!(step2(), 1419);
    }
}
