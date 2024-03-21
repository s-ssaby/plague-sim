/// Responsible for dealing with locations


/// Represents a way to think of locations in the world
/// Requires distance to be implemented to represent distances between instances of location
pub trait Location {
    fn distance(first: Self, second: Self) -> f64;
}

pub struct Point2D {
    pub x: f64,
    pub y: f64
}

impl Location for Point2D {
    fn distance(first: Self, second: Self) -> f64 {
        f64::sqrt((first.x - second.x)*(first.x - second.x) + (first.y - second.y)*(first.y - second.y))
    }
}