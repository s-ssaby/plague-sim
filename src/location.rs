/// Responsible for dealing with locations


/// Represents a way to think of locations in the world
/// Requires distance to be implemented to represent distances between instances of location
pub trait Location : Clone {
    fn distance(first: Self, second: Self) -> f64;
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Point2D {
    pub x: f64,
    pub y: f64
}

impl Point2D {
    pub fn new (x: f64, y: f64) -> Self {
        Self {x, y}
    }
}

impl Location for Point2D {
    fn distance(first: Self, second: Self) -> f64 {
        f64::sqrt((first.x - second.x)*(first.x - second.x) + (first.y - second.y)*(first.y - second.y))
    }
}