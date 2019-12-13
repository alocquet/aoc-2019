use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::ops::AddAssign;
use std::ops::SubAssign;

pub const UP: Point = Point { x: 0, y: -1 };
pub const DOWN: Point = Point { x: 0, y: 1 };
pub const RIGHT: Point = Point { x: 1, y: 0 };
pub const LEFT: Point = Point { x: -1, y: 0 };
pub const ORIGIN: Point = Point { x: 0, y: 0 };

pub const D3_ORIGIN: D3Point = D3Point { x: 0, y: 0, z: 0 };

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct D3Point {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

type ValueFormatter<T> = fn(Option<&T>) -> char;

pub struct Map<T> {
    pub values: HashMap<Point, T>,
    pub formatter: ValueFormatter<T>,
}

impl<T> Map<T> {
    pub fn new(formatter: ValueFormatter<T>) -> Self {
        Map {
            values: HashMap::new(),
            formatter,
        }
    }
    pub fn height(&self) -> isize {
        let points: Vec<Point> = self.values.keys().cloned().collect();
        points.iter().max_by_key(|p| p.y).unwrap_or(&ORIGIN).y
            - points.iter().min_by_key(|p| p.y).unwrap_or(&ORIGIN).y
    }
}

impl<T> Display for Map<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let points: Vec<Point> = self.values.keys().cloned().collect();
        let min_x = points.iter().min_by_key(|p| p.x).unwrap_or(&ORIGIN).x;
        let max_x = points.iter().max_by_key(|p| p.x).unwrap_or(&ORIGIN).x;
        let min_y = points.iter().min_by_key(|p| p.y).unwrap_or(&ORIGIN).y;
        let max_y = points.iter().max_by_key(|p| p.y).unwrap_or(&ORIGIN).y;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                write!(
                    f,
                    "{}",
                    (self.formatter)(self.values.get(&Point::new(x, y)))
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

impl D3Point {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        D3Point { x, y, z }
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

impl AddAssign for D3Point {
    fn add_assign(&mut self, other: D3Point) {
        *self = D3Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl SubAssign for D3Point {
    fn sub_assign(&mut self, other: D3Point) {
        *self = D3Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}
