use web_sys::console;
use crate::constants;
use crate::constants::{TINY, SMALL, BIG, LARGE,SLOW, FAST, EXPLODES, GROWS };
use crate::logic::Point;
use crate::random::{random_range, random_sign, random_velocity};

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
    ($name: ident, $color: expr, $radius: expr, $velocity: expr, $growth_speed: expr, $growth_size: expr) => {
        pub const $name: BallType = BallType{
            color: $color,
            radius: $radius,
            velocity: $velocity,
            growth_speed: $growth_speed,
            growth_size: $growth_size,
         };
    }
}

impl_ball_types_constants!(BLACK_BALL, BLACK, LARGE, FAST, LARGE, GROWS);
impl_ball_types_constants!(WHITE_BALL, WHITE, BIG, SLOW, LARGE, EXPLODES);

impl_ball_types_constants!(GRAY_BALL, GRAY, SMALL, FAST, LARGE, GROWS);
impl_ball_types_constants!(RED_BALL, RED, SMALL, FAST, LARGE, EXPLODES);
impl_ball_types_constants!(BLUE_BALL, BLUE, SMALL, FAST, BIG, GROWS);
impl_ball_types_constants!(GREEN_BALL, GREEN, SMALL, FAST, BIG, EXPLODES);
impl_ball_types_constants!(YELLOW_BALL, YELLOW, SMALL, SLOW, LARGE, GROWS);
impl_ball_types_constants!(CYAN_BALL, CYAN, SMALL, SLOW, LARGE, EXPLODES);
impl_ball_types_constants!(MAGENTA_BALL, MAGENTA, SMALL, SLOW, BIG, GROWS);
impl_ball_types_constants!(TEAL_BALL, TEAL, SMALL, SLOW, BIG, EXPLODES);
impl_ball_types_constants!(OLIVE_BALL, OLIVE, TINY, FAST, LARGE, GROWS);
impl_ball_types_constants!(PURPLE_BALL, PURPLE, TINY, FAST, LARGE, EXPLODES);
impl_ball_types_constants!(NAVY_BALL, NAVY, TINY, FAST, BIG, GROWS);
impl_ball_types_constants!(MAROON_BALL, MAROON, TINY, FAST, BIG, EXPLODES);
impl_ball_types_constants!(FORREST_BALL, FORREST, TINY, SLOW, LARGE, GROWS);
impl_ball_types_constants!(SILVER_BALL, SILVER, TINY, SLOW, LARGE, EXPLODES);
impl_ball_types_constants!(ORANGE_BALL, ORANGE, TINY, SLOW, BIG, GROWS);
impl_ball_types_constants!(BROWN_BALL, BROWN, TINY, SLOW, BIG, EXPLODES);


#[derive(Debug, Clone, Copy)]
pub struct Ball {
    pub pos: Point,
    pub radius: f64,
    // velocity is a 2d vector
    pub velocity: Point,
    pub color: Color,
    pub active: bool,
}

impl Ball {
    pub fn new(pos: Point, radius: f64, velocity: Point, color: Color) -> Self {
        Self {
            pos,
            radius,
            velocity,
            color,
            active: false
        }
    }

    pub fn random_ball(width: usize, height: usize, ball_type: BallType) -> Self {
        Self {
            pos: Point::random_point(width, height),
            radius: ball_type.radius,
            color: ball_type.color,

            velocity: Point::random_velocity(ball_type.velocity * 0.8, ball_type.velocity * 1.2),

            active: false
        }
    }

    pub fn tick(&mut self) {
        // walls
        let next_x = self.pos.x + self.velocity.x;
        let next_y = self.pos.y + self.velocity.y;

        if next_x + self.radius > constants::WIDTH as f64 {
            self.velocity.x = -self.velocity.x;
        }
        if next_x - self.radius < 0.0 {
            self.velocity.x = -self.velocity.x;
        }
        if next_y + self.radius > constants::HEIGHT as f64 {
            self.velocity.y = -self.velocity.y;
        }

        if next_y - self.radius < 0.0 {
            self.velocity.y = -self.velocity.y;
        }

        self.pos = Point {
            x: self.pos.x + self.velocity.x,
            y: self.pos.y + self.velocity.y,
        }
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
            y: random_sign() * random_velocity(min, max),

        }
    }
}