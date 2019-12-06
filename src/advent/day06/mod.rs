use std::collections::HashMap;

fn parse(input: String) -> HashMap<String, Vec<String>> {
    let mut result = HashMap::new();
    input
        .lines()
        .map(|orbit| orbit.split(')').collect::<Vec<&str>>())
        .for_each(|orbit| {
            result
                .entry(orbit[0].to_string())
                .or_insert_with(Vec::new)
                .push(orbit[1].to_string());
            result
                .entry(orbit[1].to_string())
                .or_insert_with(Vec::new)
                .push(orbit[0].to_string());
        });
    result
}

pub fn count_orbits(input: String, start: String, target: Option<&String>) -> usize {
    let orbits = parse(input);
    let mut weights = HashMap::new();

    let mut queue = vec![start.clone()];
    weights.insert(start, 0usize);
    while let Some(current) = queue.pop() {
        let current_weight = *weights
            .get(&current)
            .expect("node must have already visited");
        if target.unwrap_or(&"".to_string()) == &current {
            return current_weight - 2;
        }
        if let Some(satellites) = orbits.get(&current) {
            for satellite in satellites {
                if !weights.contains_key(satellite) {
                    weights.insert(satellite.clone(), current_weight + 1);
                    queue.push(satellite.clone());
                }
            }
        }
    }
    // if not target : return all possibles orbits
    weights.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r#"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L"#;

    const EXAMPLE_2: &str = r#"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN"#;

    #[test]
    fn check_parser() {
        let mut expected = HashMap::new();
        expected.insert("COM".to_string(), vec!["B".to_string()]);
        expected.insert("B".to_string(), vec!["COM".to_string(), "C".to_string()]);
        expected.insert("C".to_string(), vec!["B".to_string()]);
        assert_eq!(parse("COM)B\nB)C".to_string()), expected);
    }

    #[test]
    fn test_example_1() {
        assert_eq!(
            count_orbits(EXAMPLE_1.to_string(), "COM".to_string(), None),
            42
        );
    }

    #[test]
    fn check_step1() {
        assert_eq!(
            count_orbits(
                crate::read_file("src/advent/day06/input.txt"),
                "COM".to_string(),
                None
            ),
            247089
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            count_orbits(
                EXAMPLE_2.to_string(),
                "YOU".to_string(),
                Some(&"SAN".to_string())
            ),
            4
        );
    }

    #[test]
    fn check_step2() {
        assert_eq!(
            count_orbits(
                crate::read_file("src/advent/day06/input.txt"),
                "YOU".to_string(),
                Some(&"SAN".to_string())
            ),
            442
        );
    }
}
