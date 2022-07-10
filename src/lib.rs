mod utils;
mod circles;
mod draw_utils;
mod logic;


use circles::draw_ball;

use std::{cell::RefCell, rc::Rc};
use std::f64::consts::PI;
//use js_sys::Function;
use wasm_bindgen::{prelude::*, JsCast, UnwrapThrowExt};
use web_sys::{HtmlDivElement, HtmlElement, KeyboardEvent, HtmlCanvasElement, CanvasRenderingContext2d, console};


fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    let precise_time = window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
    console::log_1(&format!("precise time: {}",precise_time as f64).into());
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn get_canvas() -> HtmlCanvasElement {
    let document = document();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    canvas
}

fn get_context() -> CanvasRenderingContext2d {
    let document = document();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    return context;
}


#[wasm_bindgen(start)]
pub fn paint() -> Result<(), JsValue> {
    let ctx = get_context();

    let canvas = get_canvas();


    let width = canvas.width() as f64;
    let height = canvas.height() as f64;

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let mut radius = 10.0;

    let mut xpos = 150.0;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        xpos += 3.0;
        if radius < 50.0 {
            radius += 1.0;
        }

        if xpos + radius < width {
            ctx.clear_rect(0.0, 0.0, width, height);

            console::log_1(&format!("{} {} ", xpos, width).into());

            draw_ball(&ctx, xpos, 350.0, radius);
            request_animation_frame(f.borrow().as_ref().unwrap());
        } else {
            let _ = f.borrow_mut().take();
            return;
        }
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}



