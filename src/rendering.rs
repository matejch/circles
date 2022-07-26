use std::cell::RefMut;
use std::f64::consts::PI;
use web_sys::{CanvasGradient, CanvasRenderingContext2d, console};
use crate::{GameState, get_context};
use crate::ball::{Ball, BLACK, BLUE, Color, DEBUG_RED, GOLD, GREEN, RED, WHITE};

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
          //  let bb = obj.bounding_rect_current();
            //draw_rect(&ctx,bb.x, bb.y, bb.w, bb.h, BLACK);
        }
    }

    pub fn render_debug_collision_info(&self, ctx: &CanvasRenderingContext2d) {
        self.clear_canvas(ctx);
        let mut i = 0.0;

        if self.tree.is_none() {
            return;
        }

        for line in self.tree.as_ref().unwrap().info_collisions().iter() {
            write_text(&ctx, 10.0, 20.0+15.0*i, format!("{:#?}",line).as_str());
            i += 1.0;
        }
    }

    pub fn render_debug_ball_quad_info(&self, ctx: &CanvasRenderingContext2d) {
        self.clear_canvas(ctx);
        let mut i = 0.0;

        if self.tree.is_none() {
            return;
        }

        for line in self.tree.as_ref().unwrap().info_balls().iter() {
            write_text(&ctx, 10.0, 20.0 + 15.0 * i, format!("{:#?}", line).as_str());
            i += 1.0;
        }
    }




    pub fn clear_canvas(&self, ctx: &CanvasRenderingContext2d) {
        ctx.clear_rect(0.0, 0.0, self.rect.w, self.rect.h);
    }

    pub fn render_quad_tree(&self, ctx: &CanvasRenderingContext2d, clear: bool) {
        if clear {
           self.clear_canvas(ctx);
        }

        let rects = self.get_rectangles();

        for r in rects {
            if r.many {
                draw_rect(ctx, r.rect.x, r.rect.y, r.rect.w, r.rect.h, RED);
            } else {
                draw_rect(ctx, r.rect.x, r.rect.y, r.rect.w, r.rect.h, BLACK);
            }
        }
    }

}

fn draw_ball_xy(ctx: &CanvasRenderingContext2d, x: f64, y: f64, radius: f64, color: Color) {
    ctx.begin_path();
    ctx.set_global_alpha(0.8);
   // let grd = ctx.create_radial_gradient(0.0,0.0,0.0, color.r as f64, color.g as f64, color.b as f64);
    let grd: CanvasGradient = ctx.create_linear_gradient(
        0,
        100,
        100
    );

    grd.add_color_stop(0.0, &WHITE.to_string());
    grd.add_color_stop(0.5, &GREEN.to_string());
    grd.add_color_stop(1.0, &BLUE.to_string());

    ctx.set_fill_style(&color.to_string().into());
    //ctx.set_fill_style(&color.to_string().into());
    ctx.arc(x, y, radius, 0.0, 1.0 * PI);
    ctx.close_path();
    ctx.fill();
    ctx.stroke();
    ctx.set_fill_style(&grd);
    ctx.begin_path();
    ctx.arc(x, y, radius, PI, 2.0 * PI);
    ctx.fill();
    ctx.stroke();
    ctx.close_path();
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