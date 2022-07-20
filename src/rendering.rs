use std::cell::RefMut;
use std::f64::consts::PI;
use web_sys::{CanvasRenderingContext2d, console};
use crate::{GameState, get_context};
use crate::ball::{Ball, BLACK, Color, DEBUG_RED, RED};

impl GameState {

    pub fn render_state(&self, ctx: &CanvasRenderingContext2d, clear: bool) {
        if clear {
            self.clear_canvas(ctx);
        }
        if self.objects.len() == 0 {
            return;
        }
        for (_, obj) in &self.objects {
            draw_ball(&ctx, &obj);
        }
    }

    pub fn render_debug_collision_info(&self, ctx: &CanvasRenderingContext2d) {
        self.clear_canvas(ctx);
        let mut i = 0.0;

        if self.quad.is_none() {
            return;
        }

        for line in self.quad.as_ref().unwrap().info_collisions().iter() {
            write_text(&ctx, 10.0, 20.0+15.0*i, format!("{:#?}",line).as_str());
            i += 1.0;
        }
    }

    pub fn render_debug_ball_quad_info(&self, ctx: &CanvasRenderingContext2d) {
        self.clear_canvas(ctx);
        let mut i = 0.0;

        if self.quad.is_none() {
            return;
        }

        for line in self.quad.as_ref().unwrap().info_ball_quads().iter() {
            write_text(&ctx, 10.0, 20.0 + 15.0 * i, format!("{:#?}", line).as_str());
            i += 1.0;
        }
    }




    pub fn clear_canvas(&self, ctx: &CanvasRenderingContext2d) {
        let width = self.width as f64;
        let height = self.height as f64;

        ctx.clear_rect(0.0, 0.0, width, height);
    }

    pub fn render_quad_tree(&self, ctx: &CanvasRenderingContext2d, clear: bool) {
        let width = self.width as f64;
        let height = self.height as f64;
        if clear {
           self.clear_canvas(ctx);
        }

        let rects = self.get_rectangles();

        for r in rects {
            if r.many {
                draw_rect(ctx, r.x, r.y, r.w, r.h, RED);
            } else {
                draw_rect(ctx, r.x, r.y, r.w, r.h, BLACK);
            }
        }
    }

}

fn draw_ball_xy(ctx: &CanvasRenderingContext2d, x: f64, y: f64, radius: f64, color: Color) {
    ctx.begin_path();
    ctx.set_global_alpha(0.8);
    ctx.set_fill_style(&color.to_string().into());
    ctx.arc(x, y, radius, 0.0, 2.0 * PI);
    ctx.fill();
    ctx.stroke();
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

pub fn draw_ball(ctx: &CanvasRenderingContext2d, obj: &Ball) {
    draw_ball_xy(&ctx, obj.pos.x, obj.pos.y, obj.radius, obj.color);
}

pub fn write_text(ctx: &CanvasRenderingContext2d, x:f64, y:f64, txt: &str) {
    ctx.set_font("14px monospace");
    ctx.fill_text(txt, x, y);
}