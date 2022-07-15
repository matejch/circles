use std::fmt;
use web_sys::console;
use crate::quadtree::QuadTreeNode;
use crate::spheres::{Ball, BLACK_BALL, BLUE_BALL, BROWN_BALL, CYAN_BALL, FORREST_BALL, GRAY_BALL, GREEN_BALL, MAGENTA_BALL, MAROON_BALL, NAVY_BALL, OLIVE_BALL, ORANGE_BALL, PURPLE_BALL, RED_BALL, SILVER_BALL, TEAL_BALL, WHITE_BALL, YELLOW_BALL};

#[derive(Debug)]
pub struct GameState {
    pub width: usize,
    pub height: usize,
    pub objects: Vec<Ball>,
    pub captured: u8,
    pub shots: u8,
    pub result: GameResult,
    pub quad: Option<Box<QuadTreeNode>>,
    pub is_paused: bool,
    pub is_render_debug: bool,
}

impl GameState {
    pub fn new(width: usize, height: usize) -> Self {
        let objects = vec![

            Ball::random_ball(width, height, BLACK_BALL),
            Ball::random_ball(width, height, WHITE_BALL),
            Ball::random_ball(width, height, GRAY_BALL),
            Ball::random_ball(width, height, RED_BALL),
            Ball::random_ball(width, height, BLUE_BALL),
            Ball::random_ball(width, height, GREEN_BALL),
            Ball::random_ball(width, height, YELLOW_BALL),
            Ball::random_ball(width, height, CYAN_BALL),
            Ball::random_ball(width, height, MAGENTA_BALL),
            Ball::random_ball(width, height, TEAL_BALL),
            Ball::random_ball(width, height, OLIVE_BALL),
            Ball::random_ball(width, height, PURPLE_BALL),
            Ball::random_ball(width, height, NAVY_BALL),
            Ball::random_ball(width, height, MAROON_BALL),
            Ball::random_ball(width, height, FORREST_BALL),
            Ball::random_ball(width, height, SILVER_BALL),
            Ball::random_ball(width, height, ORANGE_BALL),
            Ball::random_ball(width, height, BROWN_BALL),



        ];

        let qt = QuadTreeNode::new(0.0, 0.0, width as f64, height as f64,objects.clone(),0);
        console::log_1(&format!("{:#?}", qt).into());

        Self {
            width,
            height,
            objects,
            captured: 0,
            shots: 3,
            result: GameResult::Playing,
            quad: qt,
            is_paused: false,
            is_render_debug: false
        }
    }

    pub fn pause_play(&mut self) {
        self.is_paused = !self.is_paused;

        if self.is_paused {
            self.is_render_debug = true;
        }
    }

    pub fn tick(&mut self) {
        for obj in &mut self.objects {
            obj.tick();
        }
        self.update_quadtree();

    }

    fn update_quadtree(&mut self) {
        self.quad = QuadTreeNode::new(0.0, 0.0, self.width as f64, self.height as f64,self.objects.clone(), 0);
    }

    pub fn show_state(&self) {
        for obj in &self.objects
        {
            console::log_1(&format!("{} {}", obj.pos.x, obj.pos.y).into());
        }
    }

    pub fn create_capture_ball(&mut self, x: f64, y: f64) {
        console::log_1(&format!("creating obj at: {} {}", x, y).into());
    }
}



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



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    Playing,
    Lost,
    Won,
}

