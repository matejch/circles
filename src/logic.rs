use std::fmt;
use web_sys::console;

#[derive(Debug)]
pub struct GameState {
    pub width: usize,
    pub height: usize,
    pub objects: Vec<Ball>,
    pub captured: u8,
    pub shots: u8,
    pub result: GameResult,
}

impl GameState {
    pub fn new(width: usize, height: usize) -> Self {
        let objects = vec![
            Ball::new(Point { x: 150., y: 150.0 }, 10.0, Point { x: 1.0, y: 0.0 }),
            Ball::new(Point { x: 250.0, y: 250.0 }, 20.0, Point { x: -1.0, y: -1.0 }),
        ];

        Self {
            width,
            height,
            objects,
            captured: 0,
            shots: 3,
            result: GameResult::Playing,
        }
    }

    pub fn tick(&mut self) {

        for obj in &mut self.objects {
            obj.tick();
        }
    }

    pub fn show_state(&self) {
        for obj in &self.objects
        {
            console::log_1(&format!("{} {}", obj.pos.x, obj.pos.y).into());
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GameResult {
    Playing,
    Lost,
    Won,
}

// pub type Point = (f64, f64);


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} <{}>", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ball {
    pub pos: Point,
    pub radius: f64,
    pub velocity: Point, // velocity 2d vector
}

impl Ball {
    pub fn new(pos: Point, radius: f64, velocity: Point) -> Self {
        Self {
            pos,
            radius,
            velocity,
        }
    }

    pub fn tick(&mut self) {
        self.pos = Point {
            x: self.pos.x + self.velocity.x,
            y: self.pos.y + self.velocity.y,
        }
    }
    pub fn show_state(&self) {
        console::log_1(&format!("bala na poziciji: {} {}", self.pos.x, self.pos.y).into());
    }
}