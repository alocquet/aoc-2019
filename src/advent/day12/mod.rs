use crate::read_file;
use crate::advent::geometry::D3Point;
use crate::advent::geometry::D3_ORIGIN;


pub fn step1(input: &[D3Point; 4], steps: usize) -> usize {
    let mut moons = input.clone();
    let mut velocities = [D3_ORIGIN, D3_ORIGIN, D3_ORIGIN, D3_ORIGIN];

    for step in 0..steps {
        // update velocities
        for i in 0..3 {
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
    fn check_step1() {
        assert_eq!(step1(&INPUT, 1000), 0);
    }
}
