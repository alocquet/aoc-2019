use crate::read_file;

pub fn step1() -> usize {
    let input = parse_input();
    0
}

fn parse_input() -> Vec<char> {
    read_file("src/advent/day09/input.txt").chars().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_step1() {
        assert_eq!(step1(), 0);
    }

}
