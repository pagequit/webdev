use crate::color::Color;

pub struct Circle {
    radius: u8,
    color: Color,
}

impl Circle {
    pub fn new(radius: u8, color: Color) -> Self {
        Self { radius, color }
    }
}
