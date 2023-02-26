use crate::shape::Shape;
use web_sys::CanvasRenderingContext2d;
use web_sys::Element;

// #[derive(Shape)]
pub struct Node {
    pub shape: Box<dyn Shape>,
    pub element: Element,
}

impl Node {
    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        self.shape.draw(&ctx);
    }
}
