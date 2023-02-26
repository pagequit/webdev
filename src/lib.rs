mod circle;
mod color;
mod node;
mod shape;
use crate::circle::Circle;
use crate::color::Color;
use crate::node::Node;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::*;

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}

fn render_t(collection: &HtmlCollection, ctx: &CanvasRenderingContext2d) {
    for idx in 0..collection.length() {
        let element = collection.item(idx).unwrap();
        // log(element.node_name().as_str());

        let shape = Circle {
            radius: 36,
            color: Color {
                red: 0,
                green: 144,
                blue: 255,
                alpha: 1.0,
            },
        };

        let node = Node {
            shape: Box::new(shape),
            element,
        };

        node.draw(&ctx);

        let children = collection.item(idx).unwrap().children();
        if children.length() > 0 {
            render_t(&children, &ctx);
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn render(xml_document: Document, canvas: HtmlCanvasElement) -> Result<(), JsValue> {
    let xml_children = xml_document.dyn_into::<XmlDocument>()?.children();

    let ctx = Rc::new(
        canvas
            .clone()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap()
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?,
    );

    let animate = Rc::new(RefCell::new(None));
    let animate_clone = animate.clone();

    *animate_clone.borrow_mut() = Some(Closure::new(move || {
        request_animation_frame(animate.borrow().as_ref().unwrap());
        ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

        render_t(&xml_children, &ctx);
    }));

    request_animation_frame(animate_clone.borrow().as_ref().unwrap());

    return Ok(());
}
