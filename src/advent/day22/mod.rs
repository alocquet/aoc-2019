use std::fmt::Debug;

use crate::read_file;

trait Operation: Debug {
    fn execute(&self, deck: &[usize]) -> Vec<usize>;
}

#[derive(Debug)]
struct DealNewStack {}

#[derive(Debug)]
struct Deal {
    increment: usize,
}

#[derive(Debug)]
struct Cut {
    position: isize,
}

impl Operation for DealNewStack {
    fn execute(&self, deck: &[usize]) -> Vec<usize> {
        let mut res = deck.to_vec();
        res.reverse();
        res
    }
}

impl Operation for Deal {
    fn execute(&self, deck: &[usize]) -> Vec<usize> {
        let mut result = deck.to_vec();
        for (idx, &card) in deck.iter().enumerate() {
            result[(idx * self.increment) % deck.len()] = card;
        }
        result
    }
}

impl Operation for Cut {
    fn execute(&self, deck: &[usize]) -> Vec<usize> {
        let position = self.position % deck.len() as isize;
        let position = if position < 0 {
            position + deck.len() as isize
        } else {
            position
        } as usize;
        deck.iter()
            .cycle()
            .skip(position)
            .take(deck.len())
            .cloned()
            .collect()
    }
}

fn parse_input(input: String) -> Vec<Box<dyn Operation>> {
    input
        .lines()
        .map(|line| {
            let result: Box<dyn Operation> = if line == "deal into new stack" {
                Box::new(DealNewStack {})
            } else if line.starts_with("cut") {
                Box::new(Cut {
                    position: line.split(' ').nth(1).unwrap().parse::<isize>().unwrap(),
                })
            } else {
                // if line.starts_with("deal with increment")
                Box::new(Deal {
                    increment: line.split(' ').nth(3).unwrap().parse::<usize>().unwrap(),
                })
            };
            result
        })
        .collect()
}

fn execute(deck_size: usize, operations: Vec<Box<dyn Operation>>) -> Vec<usize> {
    let mut deck: Vec<usize> = (0..deck_size).collect();
    for operation in &operations {
        deck = operation.execute(&deck);
    }
    deck
}

pub fn step1() -> usize {
    let deck = execute(10007, parse_input(read_file("src/advent/day22/input.txt")));
    deck.iter()
        .enumerate()
        .find(|&(_, &deck)| deck == 2019)
        .map(|(idx, _)| idx)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = r#"deal with increment 7
deal into new stack
deal into new stack"#;
        assert_eq!(
            execute(10, parse_input(input.to_string())),
            vec!(0, 3, 6, 9, 2, 5, 8, 1, 4, 7)
        );
    }

    #[test]
    fn test_example_2() {
        let input = r#"cut 6
deal with increment 7
deal into new stack"#;
        assert_eq!(
            execute(10, parse_input(input.to_string())),
            vec!(3, 0, 7, 4, 1, 8, 5, 2, 9, 6)
        );
    }

    #[test]
    fn test_example_3() {
        let input = r#"deal with increment 7
deal with increment 9
cut -2"#;
        assert_eq!(
            execute(10, parse_input(input.to_string())),
            vec!(6, 3, 0, 7, 4, 1, 8, 5, 2, 9)
        );
    }

    #[test]
    fn test_example_4() {
        let input = r#"deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1"#;
        assert_eq!(
            execute(10, parse_input(input.to_string())),
            vec!(9, 2, 5, 8, 1, 4, 7, 0, 3, 6)
        );
    }

    #[test]
    fn check_step1() {
        assert_eq!(step1(), 1867);
    }
}
