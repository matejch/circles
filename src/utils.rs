use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, console, HtmlCanvasElement};

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
pub fn get_canvas() -> HtmlCanvasElement {
    let document = document();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    canvas
}

pub fn get_context() -> CanvasRenderingContext2d {
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
    context
}


pub fn get_debug_context() -> CanvasRenderingContext2d {
    let win = window();

    //get DPI
    let dpi = win.device_pixel_ratio();
//get canvas
//     let canvas = document.getElementById('myCanvas');
// //get context
//     let ctx = canvas.getContext('2d');
//     function fix_dpi() {
// //get CSS height
// //the + prefix casts it to an integer
// //the slice method gets rid of "px"
//         let style_height = +getComputedStyle(canvas).getPropertyValue("height").slice(0, -2);
// //get CSS width
//         let style_width = +getComputedStyle(canvas).getPropertyValue("width").slice(0, -2);
// //scale the canvas
//         canvas.setAttribute('height', style_height * dpi);
//         canvas.setAttribute('width', style_width * dpi);
//     }



    let document = document();
    let canvas = document.get_element_by_id("canvas_debug").unwrap();
    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    //let stil = win.get_computed_style(&canvas);

    // let style_height = stil
    // get_property_value("height");
    //canvas.set_attribute("height", style_height * dpi);

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
    context
}


pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

pub fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}
