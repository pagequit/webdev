use std::fmt;

pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: f32,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(
            f,
            "rgba({}, {}, {}, {})",
            self.red, self.green, self.blue, self.alpha
        );
    }
}

impl From<(u8, u8, u8, f32)> for Color {
    fn from(value: (u8, u8, u8, f32)) -> Self {
        return Self {
            red: value.0,
            green: value.1,
            blue: value.2,
            alpha: value.3,
        };
    }
}

impl Color {
    pub fn from_rgb(value: [u8; 3]) -> Self {
        return Self {
            red: value[0],
            green: value[1],
            blue: value[2],
            alpha: 1.0,
        };
    }
}
