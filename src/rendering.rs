use std::f64::consts::PI;
use web_sys::{CanvasRenderingContext2d, console};
use crate::{GameState, get_context};
use crate::spheres::{Ball, BLACK, Color, DEBUG_RED, RED};


fn draw_ball_xy(ctx: &CanvasRenderingContext2d, x: f64, y: f64, radius: f64, color: Color) {
    ctx.begin_path();
    ctx.set_fill_style(&color.to_string().into());
    ctx.arc(x, y, radius, 0.0, 2.0 * PI);
    ctx.fill();
    ctx.stroke();
}


pub fn draw_ball(ctx: &CanvasRenderingContext2d, obj: &Ball) {
    draw_ball_xy(&ctx, obj.pos.x, obj.pos.y, obj.radius, obj.color);
}

pub fn render_state(game: &GameState, ctx: &CanvasRenderingContext2d) {
    // game.show_state();
    clear_canvas(game, ctx);
    for obj in &game.objects {
        // &obj.show_state();
        draw_ball(&ctx, &obj);
    }
}

pub fn clear_canvas(game: &GameState, ctx: &CanvasRenderingContext2d) {
    let width = game.width as f64;
    let height = game.height as f64;
    ctx.clear_rect(0.0, 0.0, width, height);
}

pub fn render_quad_tree(game: &GameState, ctx: &CanvasRenderingContext2d) {


    let width = game.width as f64;
    let height = game.height as f64;

    ctx.clear_rect(0.0, 0.0, width, height);

    let rects = game.quad.as_ref().unwrap().get_rectangles();



    for r in rects {
        if r.many {
            draw_rect(ctx, r.x, r.y, r.w, r.h, RED);
        } else {
            draw_rect(ctx, r.x, r.y, r.w, r.h, BLACK);
        }

    }
}

fn draw_rect(ctx: &CanvasRenderingContext2d, x: f64, y: f64, width: f64, height: f64, color: Color) {
    ctx.begin_path();
    ctx.set_fill_style(&color.to_string().into());
    ctx.set_stroke_style(&color.to_string().into());
    ctx.stroke_rect(x, y, width, height);
    ctx.stroke();
}

fn draw_active_rect(ctx: &CanvasRenderingContext2d, x: f64, y: f64, width: f64, height: f64, color: Color) {
    ctx.begin_path();
    ctx.set_fill_style(&color.to_string().into());
    ctx.set_stroke_style(&color.to_string().into());
    ctx.fill_rect(x, y, width, height);
    ctx.stroke();
}
