pub fn step1() -> usize {
    run(false)
}

pub fn step2() -> usize {
    run(true)
}

pub fn run(fuel_has_mass: bool) -> usize {
    crate::read_file("src/advent/day01/input.txt")
        .split('\n')
        .map(|mass_str| {
            mass_str
                .parse::<usize>()
                .expect("input should contain only numbers")
        })
        .map(|mass| compute_fuel_for_mass(mass, fuel_has_mass))
        .sum()
}

fn compute_fuel(mass: usize) -> usize {
    if mass < 6 {
        0
    } else {
        mass / 3 - 2
    }
}

fn compute_fuel_for_mass(mass: usize, fuel_has_mass: bool) -> usize {
    let mut fuel_quantity = compute_fuel(mass);
    let mut fuel_mass_that_need_fuel = fuel_quantity;
    while fuel_has_mass && fuel_mass_that_need_fuel > 0 {
        fuel_mass_that_need_fuel = compute_fuel(fuel_mass_that_need_fuel);
        fuel_quantity += fuel_mass_that_need_fuel;
    }
    fuel_quantity
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mass_of_12_is_2() {
        assert_eq!(2, compute_fuel(12));
    }

    #[test]
    fn mass_of_14_is_2() {
        assert_eq!(2, compute_fuel(14));
    }

    #[test]
    fn mass_of_1969_is_654() {
        assert_eq!(654, compute_fuel(1969));
    }

    #[test]
    fn mass_of_100756_is_33583() {
        assert_eq!(33583, compute_fuel(100756));
    }

    #[test]
    fn check_step1() {
        assert_eq!(3464735, step1());
    }

    #[test]
    fn mass_of_12_is_still_2_when_full_as_a_mass() {
        assert_eq!(2, compute_fuel_for_mass(12, true));
    }

    #[test]
    fn mass_of_1969_is_now_966_when_full_as_a_mass() {
        assert_eq!(966, compute_fuel_for_mass(1969, true));
    }

    #[test]
    fn mass_of_100756_is_now_50346_when_full_as_a_mass() {
        assert_eq!(50346, compute_fuel_for_mass(100756, true));
    }

    #[test]
    fn check_step2() {
        assert_eq!(5194211, step2());
    }
}
