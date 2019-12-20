use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::ops::Add;
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

type ValueFormatter<T> = fn(&mut Formatter<'_>, Option<&T>) -> Result<(), Error>;
type NewLineFormatter = fn(&mut Formatter<'_>, usize) -> Result<(), Error>;

pub fn default_value_formatter<T>() -> ValueFormatter<T>
where
    T: Display,
{
    |f, v| write!(f, "{}", v.unwrap())
}

pub fn default_nl_formatter() -> NewLineFormatter {
    |f, _| writeln!(f)
}

pub struct Map<T>
where
    T: Display,
{
    pub values: HashMap<Point, T>,
    pub formatter: ValueFormatter<T>,
    pub nl_formatter: NewLineFormatter,
    default_value: T,
}

impl<T> Map<T>
where
    T: Display + Default,
{
    pub fn new(formatter: ValueFormatter<T>, nl_formatter: NewLineFormatter) -> Self {
        Map {
            values: HashMap::new(),
            formatter,
            nl_formatter,
            default_value: T::default(),
        }
    }
    pub fn with_default_formatters(default_value: T) -> Self
    where
        T: Clone,
    {
        Map {
            values: HashMap::new(),
            formatter: default_value_formatter::<T>(),
            nl_formatter: default_nl_formatter(),
            default_value,
        }
    }
    pub fn height(&self) -> usize {
        let points: Vec<Point> = self.values.keys().cloned().collect();
        (points.iter().max_by_key(|p| p.y).unwrap_or(&ORIGIN).y
            - points.iter().min_by_key(|p| p.y).unwrap_or(&ORIGIN).y) as usize
    }
    pub fn width(&self) -> usize {
        let points: Vec<Point> = self.values.keys().cloned().collect();
        (points.iter().max_by_key(|p| p.x).unwrap_or(&ORIGIN).x
            - points.iter().min_by_key(|p| p.x).unwrap_or(&ORIGIN).x) as usize
    }
}

impl<T> Display for Map<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let points: Vec<Point> = self.values.keys().cloned().collect();
        let min_x = points.iter().min_by_key(|p| p.x).unwrap_or(&ORIGIN).x;
        let max_x = points.iter().max_by_key(|p| p.x).unwrap_or(&ORIGIN).x;
        let min_y = points.iter().min_by_key(|p| p.y).unwrap_or(&ORIGIN).y;
        let max_y = points.iter().max_by_key(|p| p.y).unwrap_or(&ORIGIN).y;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                (self.formatter)(
                    f,
                    self.values
                        .get(&Point::new(x, y))
                        .or(Some(&self.default_value)),
                )?;
            }
            (self.nl_formatter)(f, (max_x - min_x) as usize)?;
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
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_map_size() {
        let mut map = Map::new(|_, _| Ok(()), |_, _| Ok(()));
        map.values.insert(Point::new(0, 1), 1);
        map.values.insert(Point::new(-1, -1), 1);
        assert_eq!(map.height(), 2);
        assert_eq!(map.width(), 1);
    }
}
