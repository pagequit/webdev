pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl From<[f64; 2]> for Position {
    fn from(value: [f64; 2]) -> Self {
        return Self {
            x: value[0],
            y: value[1],
        };
    }
}
