use crate::color::Color;
use crate::shape::Shape;
use std::f64::consts::PI;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct Circle {
    pub radius: u8,
    pub color: Color,
}

impl Shape for Circle {
    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.set_fill_style(&JsValue::from_str(format!("{}", self.color).as_str()));
        ctx.begin_path();
        ctx.arc(150.0, 75.0, 36.0, PI * 2.0, 0.0).unwrap();
        ctx.close_path();
        ctx.fill();
    }
}
