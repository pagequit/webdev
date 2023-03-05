use crate::point2d::Point2D;

#[derive(Copy, Clone)]
pub struct Line2D {
    pub a: Point2D,
    pub b: Point2D,
}

impl From<[Point2D; 2]> for Line2D {
    fn from(value: [Point2D; 2]) -> Self {
        return Self {
            a: value[0],
            b: value[1],
        };
    }
}
