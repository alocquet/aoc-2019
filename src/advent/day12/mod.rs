use crate::read_file;
use crate::advent::geometry::D3Point;
use crate::advent::geometry::D3_ORIGIN;
use std::collections::HashSet;
use std::collections::HashMap;
use num::Integer;

pub fn step1(input: &[D3Point; 4], steps: usize) -> usize {
    let mut moons = input.clone();
    let mut velocities = [D3_ORIGIN, D3_ORIGIN, D3_ORIGIN, D3_ORIGIN];

    for step in 1..=steps {
        // update velocities
        for i in 0..4 {
            for j in i + 1..4 {
                let mut moon = moons[i];
                let other = moons[j];
                let velocity_change = D3Point { x: compute_velocity_change(moon.x, other.x), y: compute_velocity_change(moon.y, other.y), z: compute_velocity_change(moon.z, other.z) };
                velocities[i] += velocity_change;
                velocities[j] -= velocity_change;
            }
        }

        // update positions
        for i in 0..4 {
            moons[i] += velocities[i];
        }
    }

    moons.iter().zip(&velocities).map(|(moon, velocity)| (moon.x.abs() + moon.y.abs() + moon.z.abs()) * (velocity.x.abs() + velocity.y.abs() + velocity.z.abs())).sum::<isize>() as usize
}

pub fn step2(input: &[D3Point; 4]) -> usize {
    let mut moons = input.clone();
    let mut velocities = [D3_ORIGIN, D3_ORIGIN, D3_ORIGIN, D3_ORIGIN];
    let mut found = [None, None, None];
    let mut step = 1;
    while found[0].is_none() || found[1].is_none() || found[2].is_none() {
        // update velocities
        for i in 0..4 {
            for j in i + 1..4 {
                let mut moon = moons[i];
                let other = moons[j];
                let velocity_change = D3Point { x: compute_velocity_change(moon.x, other.x), y: compute_velocity_change(moon.y, other.y), z: compute_velocity_change(moon.z, other.z) };
                velocities[i] += velocity_change;
                velocities[j] -= velocity_change;
            }
        }

        // update positions
        // check step2
        let mut is_matching = [true, true, true];
        for i in 0..4 {
            moons[i] += velocities[i];
            is_matching[0] = found[0].is_none() && is_matching[0] && moons[i].x == input[i].x && velocities[i].x == 0;
            is_matching[1] = found[1].is_none() && is_matching[1] && moons[i].y == input[i].y && velocities[i].y == 0;
            is_matching[2] = found[2].is_none() && is_matching[2] && moons[i].z == input[i].z && velocities[i].z == 0;
        }
        for i in 0..3 {
            if is_matching[i] { found[i] = Some(step); }
        }
        step += 1;
    }

    found[0].unwrap().lcm(&found[1].unwrap()).lcm(&found[2].unwrap())
}

fn compute_velocity_change(moon: isize, other: isize) -> isize {
    if moon > other {
        -1
    } else if moon < other {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [D3Point; 4] = [D3Point { x: -9, y: -1, z: -1 },
        D3Point { x: 2, y: 9, z: 5 },
        D3Point { x: 10, y: 18, z: -12 },
        D3Point { x: -6, y: 15, z: -7 }];

    const EXAMPLE: [D3Point; 4] = [D3Point { x: -1, y: 0, z: 2 },
        D3Point { x: 2, y: -10, z: -7 },
        D3Point { x: 4, y: -8, z: 8 },
        D3Point { x: 3, y: 5, z: -1 }];

    #[test]
    fn example() {
        assert_eq!(step1(&EXAMPLE, 10), 179);
    }

    #[test]
    fn example_step2() {
        assert_eq!(step2(&EXAMPLE), 2772);
    }

    #[test]
    fn check_step1() {
        assert_eq!(step1(&INPUT, 1000), 12644);
    }

    #[test]
    fn check_step2() {
        assert_eq!(step2(&INPUT), 290_314_621_566_528);
    }
}
