use std::fmt;

pub struct BaseballState {
    pub x: f64, // ft, x location on an axis from 3B -> 1B centered on plate-2B line
    pub y: f64, // ft, y location on plate-2B line starting from plate center (1B/3B line intersection)
    pub z: f64, // ft, z location (above field level)
}

impl fmt::Display for BaseballState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BaseballState(x: {}, y: {}, z: {})",
            self.x, self.y, self.z
        )
    }
}
