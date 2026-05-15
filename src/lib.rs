use std::fmt;

pub struct BaseballState {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl fmt::Display for BaseballState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BaseballState(x: {}, y: {}, z: {})", self.x, self.y, self.z)
    }
}

