extern crate digits_iterator;

use digits_iterator::*;

pub fn step1() -> usize {
    let mut counter = 0;
    for number in 130254..678275 {
        if is_valid(number) {
            counter += 1;
        }
    }
    counter
}

fn is_valid(number: usize) -> bool{
    let mut previous = 0;
    let mut hasDouble = false;
    for digit in number.digits() {
        if digit < previous {
            return false;
        }
        if digit == previous {
            hasDouble = true;
        }
        previous = digit;
    }
    return hasDouble;
}

pub fn step2() -> usize {
    let mut counter = 0;
    for number in 130254..678275 {
        if is_valid_step2(number) {
            counter += 1;
        }
    }
    counter
}

fn is_valid_step2(number: usize) -> bool{
    let mut previous = 0;
    let mut isCurrentDouble = false;
    let mut digitAccount = 1;
    let mut hasDouble = false;
    for digit in number.digits() {
        if digit < previous {
            return false;
        }
        if digit == previous {
            digitAccount+=1;
        } else {
            if digitAccount == 2 {
                hasDouble = true
            }
            digitAccount = 1;
        }
        previous = digit;
    }
    return hasDouble || digitAccount == 2;
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
