use std::ops::Div;
use js_sys::Math::{max, min};
use web_sys::console;
use crate::constants;
use crate::constants::{TINY, SMALL, BIG, LARGE, FULL, SLOW, FAST, EXPLODES, GROWS, SHRINK, COLOSSAL};
use crate::logic::Point;
use crate::random::{random_range, random_sign, random_velocity};
use crate::spheres::BallState::{Shrinking, Vanish, Normal, Expanding};


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn to_string(self) -> String {
        format!("rgb({},{},{})", self.r, self.g, self.b)
    }
}

pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
pub const WHITE: Color = Color { r: 255, g: 255, b: 255 };
pub const GRAY: Color = Color { r: 128, g: 128, b: 128 };
pub const RED: Color = Color { r: 255, g: 0, b: 0 };
pub const BLUE: Color = Color { r: 0, g: 0, b: 255 };
pub const GREEN: Color = Color { r: 0, g: 255, b: 0 };
pub const YELLOW: Color = Color { r: 255, g: 255, b: 0 };
pub const CYAN: Color = Color { r: 0, g: 255, b: 255 };
pub const MAGENTA: Color = Color { r: 255, g: 0, b: 255 };
pub const TEAL: Color = Color { r: 0, g: 128, b: 128 };
pub const OLIVE: Color = Color { r: 128, g: 128, b: 0 };
pub const PURPLE: Color = Color { r: 128, g: 0, b: 128 };
pub const NAVY: Color = Color { r: 0, g: 0, b: 128 };
pub const MAROON: Color = Color { r: 128, g: 0, b: 0 };
pub const FORREST: Color = Color { r: 0, g: 128, b: 0 };
pub const SILVER: Color = Color { r: 192, g: 192, b: 192 };
pub const ORANGE: Color = Color { r: 255, g: 165, b: 0 };
pub const BROWN: Color = Color { r: 128, g: 69, b: 19 };


pub const DEBUG_RED: Color = Color { r: 50, g: 0, b: 0 };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BallState {
    Normal,
    Expanding,
    Shrinking,
    Vanish,
}

#[derive(Debug)]
pub struct BallType {
    color: Color,
    radius: f64,
    growth_speed: f64,
    growth_size: f64,
    velocity: f64,
}

macro_rules! impl_ball_types_constants {
    ($name: ident, $color: expr, $radius: expr, $velocity: expr, $growth_size: expr, $growth_speed: expr) => {
        pub const $name: BallType = BallType{
            color: $color,
            radius: $radius,
            velocity: $velocity,
            growth_speed: $growth_speed,
            growth_size: $growth_size,
         };
    }
}

impl_ball_types_constants!(BLACK_BALL, BLACK, LARGE, FAST, COLOSSAL, GROWS);
impl_ball_types_constants!(WHITE_BALL, WHITE, BIG, SLOW, LARGE, EXPLODES);

impl_ball_types_constants!(GRAY_BALL, GRAY, SMALL, FAST, FULL, GROWS);
impl_ball_types_constants!(RED_BALL, RED, SMALL, FAST, FULL, EXPLODES);
impl_ball_types_constants!(BLUE_BALL, BLUE, SMALL, FAST, FULL, GROWS);
impl_ball_types_constants!(GREEN_BALL, GREEN, SMALL, FAST, FULL, EXPLODES);
impl_ball_types_constants!(YELLOW_BALL, YELLOW, SMALL, SLOW, FULL, GROWS);
impl_ball_types_constants!(CYAN_BALL, CYAN, SMALL, SLOW, FULL, EXPLODES);
impl_ball_types_constants!(MAGENTA_BALL, MAGENTA, SMALL, SLOW, FULL, GROWS);
impl_ball_types_constants!(TEAL_BALL, TEAL, SMALL, SLOW, FULL, EXPLODES);
impl_ball_types_constants!(OLIVE_BALL, OLIVE, TINY, FAST, FULL, GROWS);
impl_ball_types_constants!(PURPLE_BALL, PURPLE, TINY, FAST, FULL, EXPLODES);
impl_ball_types_constants!(NAVY_BALL, NAVY, TINY, FAST, FULL, GROWS);
impl_ball_types_constants!(MAROON_BALL, MAROON, TINY, FAST, FULL, EXPLODES);
impl_ball_types_constants!(FORREST_BALL, FORREST, TINY, SLOW, FULL, GROWS);
impl_ball_types_constants!(SILVER_BALL, SILVER, TINY, SLOW, COLOSSAL, EXPLODES);
impl_ball_types_constants!(ORANGE_BALL, ORANGE, TINY, SLOW, FULL, GROWS);
impl_ball_types_constants!(BROWN_BALL, BROWN, TINY, SLOW, FULL, EXPLODES);

impl_ball_types_constants!(ACTIVE_BALL, WHITE, TINY, SLOW, FULL, GROWS);


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ball {
    pub id: usize,
    pub pos: Point,
    pub radius: f64,
    // velocity is a 2d vector
    pub velocity: Point,
    pub color: Color,
    pub ball_state: BallState,
    pub radius_growth: f64,
    pub max_radius: f64,
    //
    pub(crate) next_position: Point,
    pub(crate) next_radius: f64,
    pub next_velocity: Point,
    pub next_ball_state: BallState,

}

impl Ball {
    pub fn new(id: usize, pos: Point, velocity: Point, ball_type: BallType, ball_state: BallState) -> Self {
        Self {
            id,
            pos,
            velocity,
            ball_state,
            radius: ball_type.radius,
            color: ball_type.color,
            radius_growth: ball_type.growth_speed,
            max_radius: ball_type.growth_size,

            next_position: pos,
            next_velocity: velocity,
            next_radius: ball_type.radius,
            next_ball_state: ball_state,
        }
    }

    pub fn random_ball(id: usize, width: usize, height: usize, ball_type: BallType) -> Self {
        let pos = Point::random_point(width, height);
        let vel = Point::random_velocity(ball_type.velocity * 0.8, ball_type.velocity * 1.2);

        Self {
            id,
            pos,
            velocity: vel,
            radius: ball_type.radius,
            color: ball_type.color,
            radius_growth: ball_type.growth_speed,
            max_radius: ball_type.growth_size,
            ball_state: Normal,

            next_position: pos,
            next_velocity: vel,
            next_radius: ball_type.radius,
            next_ball_state: Normal,
        }
    }

    fn check_wall_collisions(&mut self) {
        let next_x = self.pos.x + self.velocity.x;
        let next_y = self.pos.y + self.velocity.y;

        if next_x + self.radius > constants::WIDTH as f64 {
            self.next_velocity.x = -self.velocity.x;
        }
        if next_x - self.radius < 0.0 {
            self.next_velocity.x = -self.velocity.x;
        }
        if next_y + self.radius > constants::HEIGHT as f64 {
            self.next_velocity.y = -self.velocity.y;
        }

        if next_y - self.radius < 0.0 {
            self.next_velocity.y = -self.velocity.y;
        }

        self.next_position = Point {
            x: self.pos.x + self.velocity.x,
            y: self.pos.y + self.velocity.y,
        };
    }

    fn handle_ball_states(&mut self) {
        match self.ball_state {
            Expanding => {
                self.next_velocity = Point { x: 0.0, y: 0.0 };
                self.next_radius = min(self.radius + self.radius_growth, self.max_radius);
                if self.next_radius == self.max_radius {
                    self.next_ball_state = Shrinking;
                }
            }
            Shrinking => {
                self.next_velocity = Point { x: 0.0, y: 0.0 };
                self.next_radius += SHRINK / self.radius;
                if self.next_radius < 5.0 {
                    self.next_ball_state = Vanish;
                }
            }

            Normal => {}
            Vanish => {}
        }
    }

    pub fn apply_tick_changes(&mut self) {
        self.pos = self.next_position;
        self.velocity = self.next_velocity;
        self.radius = self.next_radius;
        self.ball_state = self.next_ball_state;
    }

    pub fn tick(&mut self) {
        // walls
        self.check_wall_collisions();
        self.handle_ball_states();
    }

    pub fn change_ball_state(&mut self, state: BallState) {
        self.ball_state = state;
    }

    pub fn show_state(&self) {
        console::log_1(&format!("bala na poziciji: {} {}", self.pos.x, self.pos.y).into());
    }
}

impl Point {
    pub fn random_point(width: usize, height: usize) -> Self {
        Self {
            x: random_range(width / 10, 9 * width / 10) as f64,
            y: random_range(width / 10, 9 * height / 10) as f64,

        }
    }
    pub fn random_velocity(min: f64, max: f64) -> Self {
        Self {
            x: random_sign() * random_velocity(min, max),
            y: 0.0,//random_sign() * random_velocity(min, max),
        }
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BallPair {
    pub(crate) first: Ball,
    pub(crate) second: Ball,
}

impl BallPair {
    pub fn is_collision(&self) -> bool {
        return balls_distance_squared(self.first, self.second) <= f64::powi(self.first.next_radius, 2) + f64::powi(self.second.next_radius, 2);
    }
}

fn circles_distance_squared(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    return (x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1);
}

fn balls_distance_squared(ball1: Ball, ball2: Ball) -> f64 {
    return circles_distance_squared(ball1.next_position.x, ball1.next_position.y, ball2.next_position.x, ball2.next_position.y);
}


// pub fn is_point_in_circle(x: f64, y: f64: f64, point_x: f64, point_y: f64, radius:f64) -> bool {
//     if point_x > x
//         && point_x < x + w
//         && point_y > y
//         && point_y < y + h {
//         return true;
//     }
//     false
// }

pub fn is_point_in_rect(x: f64, y: f64, w: f64, h: f64, point_x: f64, point_y: f64) -> bool {
    if point_x > x
        && point_x < x + w
        && point_y > y
        && point_y < y + h {
        return true;
    }
    false
}


pub fn is_ball_in_cell(x: f64, y: f64, w: f64, h: f64, ball: &Ball) -> bool {
    let left_point = is_point_in_rect(x, y, w, h, ball.next_position.x - ball.next_radius, ball.next_position.y);
    let right_point = is_point_in_rect(x, y, w, h, ball.next_position.x + ball.next_radius, ball.next_position.y);
    let top_point = is_point_in_rect(x, y, w, h, ball.next_position.x, ball.next_position.y + ball.next_radius);
    let bottom_point = is_point_in_rect(x, y, w, h, ball.next_position.x, ball.next_position.y - ball.next_radius);

    if left_point || right_point || top_point || bottom_point {
        return true;
    }

    false
}


pub fn is_ball_in_cell_diag(x: f64, y: f64, w: f64, h: f64, ball: &Ball) -> bool {
    let top_left = is_point_in_rect(x, y, w, h, ball.next_position.x - ball.next_radius, ball.next_position.y + ball.next_radius);
    let top_right = is_point_in_rect(x, y, w, h, ball.next_position.x + ball.next_radius, ball.next_position.y + ball.next_radius);
    let bottom_left = is_point_in_rect(x, y, w, h, ball.next_position.x - ball.next_radius, ball.next_position.y - ball.next_radius);
    let bottom_right = is_point_in_rect(x, y, w, h, ball.next_position.x + ball.next_radius, ball.next_position.y - ball.next_radius);

    if top_left || top_right || bottom_left || bottom_right {
        return true;
    }

    false
}


pub fn resolve_collision_active(ball1: &mut Ball, ball2: &mut Ball) {
    if (ball1.ball_state == Expanding || ball1.ball_state == Shrinking) && ball2.ball_state == Normal {
        ball2.change_ball_state(Expanding);
    }

    if (ball2.ball_state == Expanding || ball2.ball_state == Shrinking) && ball2.ball_state == Normal {
        ball1.change_ball_state(Expanding);
    }
}

pub fn resolve_collision(mut pair: BallPair) -> BallPair {
    let x1b = pair.first.pos.x;
    let y1b = pair.first.pos.y;

    let x2b = pair.second.pos.x;
    let y2b = pair.second.pos.y;

    let vx1b = pair.first.velocity.x;
    let vy1b = pair.first.velocity.y;

    let vx2b = pair.second.velocity.x;
    let vy2b = pair.second.velocity.y;

    let m1 = f64::powi(pair.first.radius, 2);
    let m2 = f64::powi(pair.second.radius, 2);

    let energy_before = 0.5 * (m1 * (f64::powi(vx1b, 2) + f64::powi(vy1b, 2)) + m2 * (f64::powi(vx2b, 2) + f64::powi(vy2b, 2)));


    return pair;
}


pub fn calc_moment_of_collision(ball1: &Ball, ball2: &Ball) -> Option<f64> {
    let first_x = ball1.pos.x;
    let first_next_x = ball1.next_position.x;
    let first_y = ball1.pos.y;
    let first_next_y = ball1.next_position.y;
    let first_r = ball1.radius;

    let second_x = ball2.pos.x;
    let second_next_x = ball2.next_position.x;
    let second_y = ball2.pos.y;
    let second_next_y = ball2.next_position.y;
    let second_r = ball2.radius;

    console::log_1(&format!("calculatating collision {} {} ", ball1.id, ball2.id).into());

    return calc_moment_of_collision_helper(
        first_x,
        first_next_x,
        first_y,
        first_next_y,
        first_r,
        second_x,
        second_next_x,
        second_y,
        second_next_y,
        second_r);
}


pub fn calc_moment_of_collision_helper(first_x: f64,
                                       first_next_x: f64,
                                       first_y: f64,
                                       first_next_y: f64,
                                       first_r: f64,
                                       second_x: f64,
                                       second_next_x: f64,
                                       second_y: f64,
                                       second_next_y: f64,
                                       second_r: f64) -> Option<f64> {
    let a_dif = first_x - second_x;
    let b_dif = first_next_x - second_next_x;
    let c_dif = first_y - second_y;
    let d_dif = first_next_y - second_next_y;

    let e = f64::powi(a_dif, 2) + f64::powi(c_dif, 2);
    let f = f64::powi(b_dif, 2) + f64::powi(d_dif, 2);
    let g = a_dif * b_dif + c_dif * d_dif;

    let r_sq = f64::powi(first_r, 2) + f64::powi(second_r, 2);

    let a = e + f + g;
    let b = g - 2.0 * e;
    let c = e - r_sq;
    let d = f64::powi(b_dif, 2) - 4.0 * a * c;
    console::log_1(&format!("calculated discriminant {} ", d).into());
    if d < 0.0 {
        //something went horribly wrong
        return None;
    }

    let rez1 = (-b + f64::sqrt(d)).div(a);
    let rez2 = (-b - f64::sqrt(d)).div(a);
    console::log_1(&format!("-----------").into());
    console::log_1(&format!("calculated rez1  {} ", rez1).into());
    console::log_1(&format!("calculated rez2  {} ", rez2).into());
    console::log_1(&format!("-----------").into());

    if rez1 > 0.0 && rez1 < 1.0 {
        return Some(rez1);
    }

    if rez2 > 0.0 && rez2 < 1.0 {
        return Some(rez2);
    }

    return None;
}
