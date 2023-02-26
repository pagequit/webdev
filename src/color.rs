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

impl Color {
    pub fn from_rgb(color: [u8; 3]) -> Self {
        return Self {
            red: color[0],
            green: color[1],
            blue: color[2],
            alpha: 1.0,
        };
    }
}
