use std::ops::AddAssign;

pub const UP: Point = Point { x: 0, y: -1 };
pub const DOWN: Point = Point { x: 0, y: 1 };
pub const RIGHT: Point = Point { x: 1, y: 0 };
pub const LEFT: Point = Point { x: -1, y: 0 };

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn manhattan_distance_from(&self, other: &Point) -> usize {
        (self.x - other.x).abs() as usize + (self.y - other.y).abs() as usize
    }
    pub fn angle_with(&self, other: &Point) -> f64 {
        (self.x as f64 - other.x as f64).atan2(self.y as f64 - other.y as f64)
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
