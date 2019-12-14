use crate::read_file;
use std::collections::vec_deque::VecDeque;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Element {
    name: String,
    nb: usize,
}

#[derive(Debug, Clone)]
pub struct Formula {
    result: Element,
    obtained_from: Vec<Element>,
}

pub type Formulas = HashMap<String, Formula>;

pub fn parse_input(input_file: &str) -> Formulas {
    read_file(&format!("src/advent/day14/{}", input_file))
        .lines()
        .map(|line| {
            let formula = line.split(" => ").collect::<Vec<&str>>();
            let result = parse_element(formula[1]);
            (
                result.name.clone(),
                Formula {
                    result,
                    obtained_from: formula[0].split(", ").map(parse_element).collect(),
                },
            )
        })
        .collect()
}

fn parse_element(input: &str) -> Element {
    let result = input.split(' ').collect::<Vec<&str>>();
    Element {
        name: result[1].to_string(),
        nb: result[0].parse::<usize>().expect("Bad element number"),
    }
}

pub fn step1(formulas: Formulas, fuel_quantity: usize) -> usize {
    let mut stock = HashMap::new();
    let mut needs = HashMap::new();
    let mut needs_queue = VecDeque::new();

    needs.insert("FUEL".to_string(), fuel_quantity);
    needs_queue.push_back("FUEL".to_string());

    while let Some(element_name) = needs_queue.pop_front() {
        let formula = formulas.get(&element_name).expect("missing formula");

        let element_stock = *stock.get(&element_name).unwrap_or(&0);
        let mut element_need = *needs.get(&element_name).unwrap();

        if element_need <= element_stock {
            stock.insert(element_name.clone(), element_stock - element_need);
        } else {
            // need production
            element_need -= element_stock;

            let nb_formula = if element_need % formula.result.nb == 0 {
                element_need / formula.result.nb
            } else {
                element_need / formula.result.nb + 1
            };
            stock.insert(
                element_name.clone(),
                nb_formula * formula.result.nb - element_need,
            );

            for need in &formula.obtained_from {
                *needs.entry(need.name.clone()).or_insert(0) += nb_formula * need.nb;
                if need.name != "ORE" && !needs_queue.contains(&need.name) {
                    needs_queue.push_back(need.name.clone());
                }
            }
        }
        needs.remove(&element_name);
    }

    *needs.get("ORE").unwrap()
}

pub fn step2(formulas: Formulas, ore_quantity: usize) -> usize {
    let mut fuel = 1;
    let mut step = 10_000_000;
    while step != 1 {
        step /= 10;
        fuel = brute_force_step1(formulas.clone(), ore_quantity, fuel, step);
    }
    fuel
}

fn brute_force_step1(formulas: Formulas, ore_quantity: usize, from: usize, step: usize) -> usize {
    let mut fuel = from;
    while step1(formulas.clone(), fuel) < ore_quantity {
        fuel += step;
    }
    fuel -= step;
    fuel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_example1_step1() {
        assert_eq!(step1(parse_input("example1.txt"), 1), 165);
    }

    #[test]
    fn check_example2_step1() {
        assert_eq!(step1(parse_input("example2.txt"), 1), 13312);
    }

    #[test]
    fn check_example3_step1() {
        assert_eq!(step1(parse_input("example3.txt"), 1), 180697);
    }

    #[test]
    fn check_example4_step1() {
        assert_eq!(step1(parse_input("example4.txt"), 1), 2210736);
    }

    #[test]
    fn check_step1() {
        assert_eq!(step1(parse_input("input.txt"), 1), 248794);
    }

    #[test]
    fn check_step2() {
        assert_eq!(step2(parse_input("input.txt"), 1_000_000_000_000), 4906796);
    }
}
