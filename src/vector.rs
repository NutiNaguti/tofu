use crate::direction::Direction;

#[derive(Debug)]
pub struct Vector {
    pub x: u8,
    pub y: u8,
}

impl Vector {
    pub fn new(x: u8, y: u8) -> Vector {
        Vector { x, y }
    }

    pub fn zero() -> Vector {
        Vector { x: 0, y: 0 }
    }
}
