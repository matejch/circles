mod logic;
mod circles;


use logic::{Ball, GameState};
use circles::draw_ball;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, console, HtmlCanvasElement};
use crate::circles::render_state;

thread_local! {
    static GAME: Rc<RefCell<GameState>> = Rc::new(RefCell::new(GameState::new(800, 600)));
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
    console::log_1(&format!("v get_context").into());
    let document = document();
    let canvas = document.get_element_by_id("canvas").unwrap();
    console::log_1(&format!("v get_context {:#?}", canvas).into());
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
    context
}

fn window() -> web_sys::Window {
    console::log_1(&format!("v window").into());
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn document() -> web_sys::Document {
    console::log_1(&format!("v document").into());

    window()
        .document()
        .expect("should have a document on window")
}

fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let ctx = get_context();
    let mut i = 0;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if i > 300 {
            let _ = f.borrow_mut().take();
            return;
        }

        i += 1;

        GAME.with(|game|
            {
                let mut game = game.borrow_mut();
                game.tick();
                render_state(&game, &ctx);
            }
        );

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}


// mod utils;
// mod circles;
// mod draw_utils;
// mod logic;
// mod raf;
//
//
// use circles::draw_ball;
// use logic::{GameState};
//
// use std::{cell::RefCell, rc::Rc};
// use std::borrow::BorrowMut;
//
// use js_sys::Function;
// use wasm_bindgen::{prelude::*, JsCast, UnwrapThrowExt};
// use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, console};
// use crate::raf::Raf;
//
//
// fn window() -> web_sys::Window {
//     web_sys::window().expect("no global `window` exists")
// }
//
// fn request_animation_frame(f: &Closure<dyn FnMut()>) {
//     let precise_time = window()
//         .request_animation_frame(f.as_ref().unchecked_ref())
//         .expect("should register `requestAnimationFrame` OK");
//     console::log_1(&format!("precise time: {}", precise_time as f64).into());
// }
//
// fn request_animation_frame_func(f: &Function) {
//     let precise_time = window()
//         .request_animation_frame(f)
//         .expect("should register `requestAnimationFrame` OK");
//     console::log_1(&format!("precise time func: {}", precise_time as f64).into());
// }
//
//
// fn document() -> web_sys::Document {
//     window()
//         .document()
//         .expect("should have a document on window")
// }
//
// fn get_canvas() -> HtmlCanvasElement {
//     let document = document();
//     let canvas = document.get_element_by_id("canvas").unwrap();
//     let canvas: HtmlCanvasElement = canvas
//         .dyn_into::<HtmlCanvasElement>()
//         .map_err(|_| ())
//         .unwrap();
//     canvas
// }
//
// fn get_context() -> CanvasRenderingContext2d {
//     let document = document();
//     let canvas = document.get_element_by_id("canvas").unwrap();
//     let canvas: HtmlCanvasElement = canvas
//         .dyn_into::<HtmlCanvasElement>()
//         .map_err(|_| ())
//         .unwrap();
//     let context = canvas
//         .get_context("2d")
//         .unwrap()
//         .unwrap()
//         .dyn_into::<CanvasRenderingContext2d>()
//         .unwrap();
//     context
// }
//
//
// pub fn main() {
//
//
//
//     let clos = Closure::new(|mut num: f64| {
//         console::log_1(&format!("heheh").into());
//         num * 2.0
//     });
//
//     let raf = Raf::new(clos);
//
//
// }
//
// //
// // pub fn myfun() -> impl FnMut(f64) -> f64 + 'static  {
// //     let clos = |mut num:f64| -> f64 {
// //         console::log_1(&format!("heheh").into());
// //         num * 2.0
// //     };
// //     clos
// // }

