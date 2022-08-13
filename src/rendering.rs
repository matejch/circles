use crate::ball::{Ball, Color, BLACK, RED};
use crate::GameState;
use std::f64::consts::PI;
use web_sys::{console, CanvasGradient, CanvasRenderingContext2d};

pub struct Renderer<'a> {
    pub gamestate: &'a GameState,
}

impl Renderer<'static> {
    pub fn render_state(&self, ctx: &mut CanvasRenderingContext2d, clear: bool) {
        let state = self.gamestate;
        if clear {
            self.clear_canvas(ctx);
        }

        if state.objects.is_empty() {
            return;
        }
        // for (_, obj) in &self.objects {
        // draw_ball(ctx, &obj);
        //  let bb = obj.bounding_rect_current();
        //draw_rect(&ctx,bb.x, bb.y, bb.w, bb.h, BLACK);
        // }

        state
            .objects
            .values()
            .into_iter()
            .for_each(|obj| draw_ball(ctx, obj));
    }

    pub fn render_won(&self, ctx: &mut CanvasRenderingContext2d) {
        self.clear_canvas(ctx);

        write_text(ctx, 30.0, 50.0, "Congratz! You won.");
        write_text(
            ctx,
            30.0,
            70.0,
            "Click canvas or press 'n' to go to the next level",
        );
    }

    pub fn render_lost(&self, ctx: &mut CanvasRenderingContext2d) {
        self.clear_canvas(ctx);
        write_text(ctx, 10.0, 20.0, "you lost");
    }

    pub fn render_debug_collision_info(&self, ctx: &CanvasRenderingContext2d) {
        let tree = self.gamestate.tree;
        self.clear_canvas(ctx);
        let mut i = 0.0;

        if tree.is_none() {
            return;
        }

        for line in tree.as_ref().unwrap().info_collisions().iter() {
            write_text(ctx, 10.0, 20.0 + 15.0 * i, format!("{:#?}", line).as_str());
            i += 1.0;
        }
    }

    pub fn render_debug_ball_quad_info(&self, ctx: &CanvasRenderingContext2d) {
        self.clear_canvas(ctx);
        let tree = self.gamestate.tree;
        let mut i = 0.0;

        if tree.is_none() {
            return;
        }

        tree.as_ref().unwrap().info_balls().iter().for_each(|line| {
            write_text(ctx, 10.0, 20.0 + 15.0 * i, format!("{:#?}", line).as_str());
            i += 1.0;
        });
    }

    pub fn clear_canvas(&self, ctx: &CanvasRenderingContext2d) {
        ctx.clear_rect(0.0, 0.0, self.gamestate.rect.w, self.gamestate.rect.h);
    }

    pub fn render_quad_tree(&self, ctx: &CanvasRenderingContext2d, clear: bool) {
        if clear {
            self.clear_canvas(ctx);
        }

        let rects = self.gamestate.get_rectangles();

        for r in rects {
            if r.many {
                draw_rect(ctx, r.rect.x, r.rect.y, r.rect.w, r.rect.h, RED);
            } else {
                draw_rect(ctx, r.rect.x, r.rect.y, r.rect.w, r.rect.h, BLACK);
            }
        }
    }
}

fn draw_ball_xy(ctx: &mut CanvasRenderingContext2d, x: f64, y: f64, radius: f64, color: Color) {
    ctx.begin_path();
    ctx.set_global_alpha(0.8);
    // let grd = ctx.create_radial_gradient(0.0,0.0,0.0, color.r as f64, color.g as f64, color.b as f64);
    //  let grd: CanvasGradient = ctx.create_linear_gradient(
    //      x-radius,
    //      y-radius,
    //      x+radius,
    //      y+radius
    //  );

    //    let grd: CanvasGradient = ctx
    //      .create_radial_gradient(x, y, 3.3 * radius / 4.0, x, y, radius)
    //    .unwrap();

    // grd.add_color_stop(0.0, &color.to_string());
    // grd.add_color_stop(1.0, &WHITE.to_string());
    //grd.add_color_stop(0.5, &WHITE.to_string());

    ctx.begin_path();
    ctx.set_fill_style(&color.to_string().into());
    // //ctx.set_fill_style(&color.to_string().into());
    // ctx.arc(x, y, radius, 0.0, 1.0 * PI);
    // ctx.close_path();
    // ctx.fill();
    // ctx.stroke();
    //ctx.set_fill_style(&grd);
    ctx.arc(x, y, radius, 0.0 * PI, 2.0 * PI);
    ctx.fill();
    ctx.stroke();
    ctx.close_path();
}

fn draw_rect(
    ctx: &CanvasRenderingContext2d,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    color: Color,
) {
    ctx.begin_path();
    ctx.set_fill_style(&color.to_string().into());
    ctx.set_stroke_style(&color.to_string().into());
    ctx.stroke_rect(x, y, width, height);
    ctx.stroke();
}

fn draw_active_rect(
    ctx: &CanvasRenderingContext2d,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    color: Color,
) {
    ctx.begin_path();
    ctx.set_fill_style(&color.to_string().into());
    ctx.set_stroke_style(&color.to_string().into());
    ctx.fill_rect(x, y, width, height);
    ctx.stroke();
}

pub fn draw_ball(ctx: &mut CanvasRenderingContext2d, obj: &Ball) {
    draw_ball_xy(ctx, obj.pos.x, obj.pos.y, obj.radius, obj.color);
}

pub fn write_text(ctx: &CanvasRenderingContext2d, x: f64, y: f64, txt: &str) {
    ctx.set_font("14px Verdana");
    ctx.set_fill_style(&BLACK.to_string().into());
    let result = ctx.fill_text(txt, x, y);
    match result {
        Ok(_) => {}
        Err(_) => {
            console::log_1(&format!("error filling text: {}", txt).into());
        }
    }
}
