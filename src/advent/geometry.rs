use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::ops::AddAssign;

pub const UP: Point = Point { x: 0, y: -1 };
pub const DOWN: Point = Point { x: 0, y: 1 };
pub const RIGHT: Point = Point { x: 1, y: 0 };
pub const LEFT: Point = Point { x: -1, y: 0 };
pub const ORIGIN: Point = Point { x: 0, y: 0 };

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

#[derive(Default)]
pub struct Map<T> {
    pub values: HashMap<Point, T>,
}

impl<T> Map<T> {}

impl Display for Map<bool> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let points: Vec<Point> = self.values.keys().cloned().collect();
        let min_x = points.iter().min_by_key(|p| p.x).unwrap().x;
        let max_x = points.iter().max_by_key(|p| p.x).unwrap().x;
        let min_y = points.iter().min_by_key(|p| p.y).unwrap().y;
        let max_y = points.iter().max_by_key(|p| p.y).unwrap().y;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                write!(
                    f,
                    "{}",
                    if *self.values.get(&Point::new(x, y)).unwrap_or(&false) {
                        '#'
                    } else {
                        '.'
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }
    pub fn manhattan_distance_from(&self, other: &Point) -> usize {
        (self.x - other.x).abs() as usize + (self.y - other.y).abs() as usize
    }
    pub fn angle_with(&self, other: &Point) -> f64 {
        (self.x as f64 - other.x as f64).atan2(self.y as f64 - other.y as f64)
    }
    pub fn rotate(&self, clockwise: bool) -> Point {
        if clockwise {
            Point::new(-self.y, self.x)
        } else {
            Point::new(self.y, -self.x)
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        *self = Point {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.y == other.y {
            self.x.cmp(&other.x)
        } else {
            self.y.cmp(&other.y)
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
