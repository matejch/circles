use std::f64::consts::PI;
use web_sys::CanvasRenderingContext2d;

pub fn draw_ball(ctx: &CanvasRenderingContext2d, x: f64, y: f64, radius: f64) {
    ctx.begin_path();
    ctx.set_fill_style(&"rgb(130,150,30)".into());
    ctx.arc(x, y, radius, 0.0, 2.0*PI);
    ctx.fill();
    ctx.stroke();
}

// fn draw_rect(ctx: &CanvasRenderingContext2d, x: f64, y: f64, width: f64, height: f64) {
//     ctx.begin_path();
//     ctx.set_fill_style(&"rgb(130,150,30)".into());
//     ctx.fill_rect(x, y, width, height);
//     ctx.stroke();
// }