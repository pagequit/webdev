use web_sys::CanvasRenderingContext2d;

pub trait Shape {
    fn draw(&self, ctx: &CanvasRenderingContext2d);
}
