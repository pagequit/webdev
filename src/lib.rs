mod circle;
mod color;
use std::{cell::RefCell, rc::Rc, f64::consts::PI};
use crate::circle::Circle;
use crate::color::Color;
use wasm_bindgen::prelude::*;

fn window() -> web_sys::Window {
    return web_sys::window().unwrap();
}

fn document() -> web_sys::Document {
    return window().document().unwrap();
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}

#[wasm_bindgen]
pub fn run(name: &str) -> Result<(), JsValue> {
    web_sys::console::log_1(&name.into());

    let body = document().body().ok_or(JsError::new("Err"))?;
    let canvas = document().create_element("canvas")?;

    body.append_child(&canvas)?;

    let ctx = Rc::new(canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap()
        .get_context("2d")?.unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>()?);

    // TODO: use this ...
    let _circle = Circle::new(
        24,
        Color {
            red: 0,
            green: 144,
            blue: 255,
            alpha: 1.0,
        },
    );

    let animate = Rc::new(RefCell::new(None));
    let animate_clone = animate.clone();

    *animate_clone.borrow_mut() = Some(Closure::new(move || {
        request_animation_frame(animate.borrow().as_ref().unwrap());
        ctx.clear_rect(0.0, 0.0, 155.0, 155.0);

        ctx.set_fill_style(&JsValue::from_str("rgba(0, 144, 255, 1)"));
        ctx.begin_path();
        ctx.arc(75.0, 75.0, 24.0, PI * 2.0, 0.0).expect("Circle?"); // ... <- here
        ctx.close_path();
        ctx.fill();
    }));

    request_animation_frame(animate_clone.borrow().as_ref().unwrap());

    return Ok(());
}
