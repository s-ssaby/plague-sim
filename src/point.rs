use serde::{Deserialize, Serialize};

/// Represents locations with a 2D Point

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Point2D {
    pub x: f64,
    pub y: f64
}

impl Point2D {
    pub fn new (x: f64, y: f64) -> Self {
        Self {x, y}
    }

    pub fn distance(&self, second: &Self) -> f64 {
        f64::sqrt((self.x - second.x)*(self.x - second.x) + (self.y - second.y)*(self.y - second.y))
    }
}