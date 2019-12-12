use crate::advent::geometry::D3Point;
use crate::advent::geometry::D3_ORIGIN;
use num::Integer;

#[derive(Clone)]
struct Moon {
    position: D3Point,
    velocity: D3Point,
}

impl Moon {
    fn apply_velocity(&mut self) {
        self.position += self.velocity;
    }
    fn potential_energy(&self) -> usize {
        (self.position.x.abs() + self.position.y.abs() + self.position.z.abs()) as usize
    }

    fn kinetic_energy(&self) -> usize {
        (self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs()) as usize
    }
    fn has_common_axis(&self, other: &Self) -> [bool; 3] {
        [
            self.position.x == other.position.x && self.velocity.x == other.velocity.x,
            self.position.y == other.position.y && self.velocity.y == other.velocity.y,
            self.position.z == other.position.z && self.velocity.z == other.velocity.z,
        ]
    }
}

fn parse_moons(input: &[D3Point; 4]) -> Vec<Moon> {
    input
        .iter()
        .map(|point| Moon {
            position: *point,
            velocity: D3_ORIGIN,
        })
        .collect()
}

pub fn step1(input: &[D3Point; 4], steps: usize) -> usize {
    let mut moons = parse_moons(input);

    for _ in 1..=steps {
        // update velocities
        apply_gravity(&mut moons);
        // update positions
        moons.iter_mut().for_each(Moon::apply_velocity);
    }

    moons
        .iter()
        .map(|moon| moon.potential_energy() * moon.kinetic_energy())
        .sum()
}

pub fn step2(input: &[D3Point; 4]) -> usize {
    let mut moons = parse_moons(input);
    let start_state = moons.clone();
    let mut cycles_idx = [None; 3];

    for step in 1.. {
        // update velocities
        apply_gravity(&mut moons);
        // update positions
        moons.iter_mut().for_each(Moon::apply_velocity);

        // check match
        let mut matching = [true; 3];
        for i in 0..4 {
            let common_axis = moons[i].has_common_axis(&start_state[i]);
            for j in 0..3 {
                matching[j] &= cycles_idx[j].is_none() && common_axis[j];
            }
        }
        matching
            .iter()
            .enumerate()
            .filter(|(_, matched)| **matched)
            .for_each(|(idx, _)| cycles_idx[idx] = Some(step));

        if cycles_idx.iter().find(|cycle| cycle.is_none()).is_none() {
            break;
        }
    }

    cycles_idx
        .iter()
        .flatten()
        .fold(1, |result, cycle| result.lcm(cycle))
}

fn apply_gravity(moons: &mut Vec<Moon>) {
    for i in 0..4 {
        for j in i + 1..4 {
            let velocity_change = moons[i]
                .position
                .compute_velocity_change(&moons[j].position);
            moons[i].velocity += velocity_change;
            moons[j].velocity -= velocity_change;
        }
    }
}

impl D3Point {
    fn compute_velocity_change(&self, other: &D3Point) -> Self {
        D3Point {
            x: other.x.cmp(&self.x) as isize,
            y: other.y.cmp(&self.y) as isize,
            z: other.z.cmp(&self.z) as isize,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [D3Point; 4] = [
        D3Point {
            x: -9,
            y: -1,
            z: -1,
        },
        D3Point { x: 2, y: 9, z: 5 },
        D3Point {
            x: 10,
            y: 18,
            z: -12,
        },
        D3Point {
            x: -6,
            y: 15,
            z: -7,
        },
    ];

    const EXAMPLE: [D3Point; 4] = [
        D3Point { x: -1, y: 0, z: 2 },
        D3Point {
            x: 2,
            y: -10,
            z: -7,
        },
        D3Point { x: 4, y: -8, z: 8 },
        D3Point { x: 3, y: 5, z: -1 },
    ];

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
