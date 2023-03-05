mod circle;
mod color;
mod drawable;
mod grid2d;
mod line2d;
mod node;
mod point2d;
mod shape;
mod vector2d;
use crate::drawable::Drawable;
use crate::grid2d::Grid2D;
use crate::node::Node;
use crate::point2d::Point2D;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::*;

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}

fn render_t(collection: &HtmlCollection, ctx: &CanvasRenderingContext2d, level: u32) {
    for idx in 0..collection.length() {
        let element = collection.item(idx).unwrap();
        let children = element.children();
        // log(element.node_name().as_str());

        // let shape = Circle {
        //     radius: 36,
        //     color: Color::from_rgb([0, 144, 255]),
        // };

        // let node = Node {
        //     shape: Box::new(shape),
        //     element,
        // };

        // node.draw(&ctx);

        // TODO

        let x_offset = (((collection.length() - 1) * 15) * (level - 1)) as f64;

        element.draw(
            &ctx,
            Point2D::from([
                150.0 - x_offset + (idx * 30) as f64,
                ((5 * (level - 1)) + (level * 30)) as f64,
            ]),
        );

        if children.length() > 0 {
            render_t(&children, &ctx, level + 1);
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
    let grid: Grid2D<Node> = Grid2D::new(canvas.width() as usize, canvas.height() as usize, 4, 8);

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

        grid.draw(&ctx);
        render_t(&xml_children, &ctx, 1);
    }));

    request_animation_frame(animate_clone.borrow().as_ref().unwrap());

    return Ok(());
}
