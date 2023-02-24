mod circle;
mod color;
use crate::circle::Circle;
use crate::color::Color;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::Document;

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
pub fn run(xml_document: Document) -> Result<(), JsValue> {
    let xml_children = xml_document.dyn_into::<web_sys::XmlDocument>()?.children();
    for index in 0..xml_children.length() {
        web_sys::console::log_1(&JsValue::from_str(xml_children.item(index).unwrap().node_name().as_str()));
    }

    let body = document().body().ok_or(JsError::new("Err"))?;
    let canvas = document().create_element("canvas")?;

    body.append_child(&canvas)?;

    let ctx = Rc::new(
        canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap()
            .get_context("2d")?
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?,
    );

    let circle = Circle {
        radius: 36,
        color: Color {
            red: 0,
            green: 144,
            blue: 255,
            alpha: 1.0,
        },
    };

    let animate = Rc::new(RefCell::new(None));
    let animate_clone = animate.clone();

    *animate_clone.borrow_mut() = Some(Closure::new(move || {
        request_animation_frame(animate.borrow().as_ref().unwrap());
        ctx.clear_rect(0.0, 0.0, 300.0, 150.0);

        circle.draw(&ctx);
    }));

    request_animation_frame(animate_clone.borrow().as_ref().unwrap());

    return Ok(());
}
