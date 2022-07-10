use std::f64::consts::PI;
use web_sys::{CanvasRenderingContext2d, console};
use crate::{GameState, get_context};
use crate::logic::Ball;

fn draw_ball_xy(ctx: &CanvasRenderingContext2d, x: f64, y: f64, radius: f64) {

    ctx.begin_path();
    ctx.set_fill_style(&"rgb(130,150,30)".into());
    ctx.arc(x, y, radius, 0.0, 2.0*PI);
    ctx.fill();
    ctx.stroke();
}


pub fn draw_ball(ctx: &CanvasRenderingContext2d, obj: &Ball) {
    draw_ball_xy(&ctx, obj.pos.x, obj.pos.y, obj.radius);
}

pub fn render_state(game: &GameState, ctx: &CanvasRenderingContext2d) {
    let width = game.width as f64;
    let height = game.height as f64;
    game.show_state();
    ctx.clear_rect(0.0, 0.0, width, height);
    for obj in &game.objects {
        &obj.show_state();
        draw_ball(&ctx, &obj);
    }

}

// fn draw_rect(ctx: &CanvasRenderingContext2d, x: f64, y: f64, width: f64, height: f64) {
//     ctx.begin_path();
//     ctx.set_fill_style(&"rgb(130,150,30)".into());
//     ctx.fill_rect(x, y, width, height);
//     ctx.stroke();
// }