use crate::{color::Color, position::Position};
use std::f64::consts::PI;
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, Element};

pub trait Drawable {
    fn draw(&self, ctx: &CanvasRenderingContext2d, position: Position);
}

impl Drawable for Element {
    fn draw(&self, ctx: &CanvasRenderingContext2d, position: Position) {
        ctx.set_fill_style(&JsValue::from_str(
            format!("{}", Color::from((0, 144, 255, 0.5))).as_str(),
        ));
        ctx.begin_path();
        ctx.arc(position.x, position.y, 15.0, PI * 2.0, 0.0).unwrap();
        ctx.close_path();
        ctx.fill();
    }
}
