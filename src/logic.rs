use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::fmt;
use std::ops::Div;
use web_sys::console;
use crate::quadtree::{QuadTreeNode, Rect};
use crate::spheres::{ACTIVE_BALL, Ball, BLACK_BALL, BLUE_BALL, BROWN_BALL, calc_moment_of_collision, CYAN_BALL, FORREST_BALL, GRAY_BALL, GREEN_BALL, MAGENTA_BALL, MAROON_BALL, NAVY_BALL, OLIVE_BALL, ORANGE, ORANGE_BALL, PURPLE_BALL, RED_BALL, resolve_collision_active, SILVER_BALL, TEAL_BALL, WHITE_BALL, YELLOW_BALL};
use crate::spheres::BallState::{Expanding, Normal, Shrinking, Vanish};

#[derive(Debug)]
pub struct GameState {
    pub width: usize,
    pub height: usize,
    //pub objects: Vec<Ball>,
    pub objects: HashMap<usize, Ball>,
    pub next_id: usize,
    pub captured: u8,
    pub shots: u8,
    pub result: GameResult,
    pub quad: Option<Box<QuadTreeNode>>,
    pub is_paused: bool,
    pub is_render_debug: bool,
}

impl GameState {
    pub fn gen_next_id(&mut self) -> usize {
        self.next_id += 1;
        return self.next_id;
    }

    pub fn insert_object(&mut self, ball: &mut Ball) {
        let new_id = self.gen_next_id();
        ball.id = new_id;
        self.objects.insert(new_id, *ball);
    }

    pub fn new(width: usize, height: usize) -> Self {
        let mut new_state = Self {
            width,
            height,
            objects: HashMap::new(),
            next_id: 1,
            captured: 0,
            shots: 3,
            result: GameResult::Playing,
            quad: None,
            is_paused: false,
            is_render_debug: false,
        };

        new_state.insert_object(Ball::random_ball(0, width, height, BLACK_BALL).borrow_mut());
        new_state.insert_object(Ball::random_ball(0, width, height, WHITE_BALL).borrow_mut());
        new_state.insert_object(Ball::random_ball(0, width, height, GRAY_BALL).borrow_mut());
        new_state.insert_object(Ball::random_ball(0, width, height, RED_BALL).borrow_mut());
        new_state.insert_object(Ball::random_ball(0, width, height, BLUE_BALL).borrow_mut());
        new_state.insert_object(Ball::random_ball(0, width, height, GREEN_BALL).borrow_mut());
        new_state.insert_object(Ball::random_ball(0, width, height, YELLOW_BALL).borrow_mut());
        new_state.insert_object(Ball::random_ball(0, width, height, CYAN_BALL).borrow_mut());
        new_state.insert_object(Ball::random_ball(0, width, height, MAGENTA_BALL).borrow_mut());
        new_state.insert_object(Ball::random_ball(0, width, height, TEAL_BALL).borrow_mut());
        new_state.insert_object(Ball::random_ball(0, width, height, OLIVE_BALL).borrow_mut());
        new_state.insert_object(Ball::random_ball(0, width, height, PURPLE_BALL).borrow_mut());
        new_state.insert_object(Ball::random_ball(0, width, height, NAVY_BALL).borrow_mut());
        new_state.insert_object(Ball::random_ball(0, width, height, MAROON_BALL).borrow_mut());
        new_state.insert_object(Ball::random_ball(0, width, height, FORREST_BALL).borrow_mut());
        new_state.insert_object(Ball::random_ball(0, width, height, SILVER_BALL).borrow_mut());
        new_state.insert_object(Ball::random_ball(0, width, height, ORANGE_BALL).borrow_mut());
        new_state.insert_object(Ball::random_ball(0, width, height, BROWN_BALL).borrow_mut());


        let qt = QuadTreeNode::new(0.0, 0.0, width as f64, height as f64, new_state.objects.clone(), 0, 0);
        //console::log_1(&format!(" quad tree: {:#?}", qt).into());
        new_state.quad = qt;

        return new_state;
    }

    pub fn check_win_lose(&self) {

    }

    pub fn get_rectangles(&self) -> Vec<Rect> {
        if self.quad.is_none() {
            return vec![];
        }

        return self.quad.as_ref().unwrap().get_rectangles();
    }

    pub fn pause_play(&mut self) {
        self.is_paused = !self.is_paused;

        if self.is_paused {
            self.is_render_debug = true;
        }
    }

    fn handle_collisions(&mut self)
    {
        if self.quad.is_none() {
            return;
        }

        // find collisions candidates
        let candidates = self.quad.as_ref().unwrap().find_collision_candidates();

        // check collisions
        for cand in candidates {
            if cand.is_collision() {

                let first = self.objects.get(&cand.first.id).expect("should be in there").ball_state;
                let second = self.objects.get(&cand.second.id).expect("should be in there").ball_state;

                let ball1 = self.objects.get(&cand.first.id).expect("should be in there");
                let ball2 = self.objects.get(&cand.second.id).expect("should be in there");

                let t = calc_moment_of_collision(ball1, ball2);


                if (first == Shrinking || first == Expanding) && second == Normal {
                    let second_ball = self.objects.get_mut(&cand.second.id).expect("should be in there");
                    second_ball.next_ball_state = Expanding;
                }

                if (second == Shrinking || second == Expanding) && first == Normal {
                    let first_ball = self.objects.get_mut(&cand.first.id).expect("should be in there");
                    first_ball.next_ball_state = Expanding;
                }
            }
        }
    }


    pub fn tick(&mut self) {
        self.objects.retain(|key, obj| obj.ball_state != Vanish);

        for (_, obj) in &mut self.objects {
            obj.tick();
        }

        self.update_quadtree();
        self.handle_collisions();

        if self.objects.len() == 0 {
            return;
        }

        for (_, obj) in &mut self.objects {
            obj.apply_tick_changes();
        }
    }

    fn update_quadtree(&mut self) {
        self.quad = QuadTreeNode::new(0.0, 0.0, self.width as f64, self.height as f64, self.objects.clone(), 0, 0);
    }

    pub fn show_state(&self) {
        for (_, obj) in &self.objects
        {
            console::log_1(&format!("{} {}", obj.pos.x, obj.pos.y).into());
        }
    }

    pub fn create_capture_ball(&mut self, x: f64, y: f64) {
        if self.shots == 0 {
            return;
        }

        let mut active_ball = Ball::new(
            self.gen_next_id(),
            Point { x, y },
            Point { x: 0.0, y: 0.0 },
            ACTIVE_BALL,
            Expanding);
        self.insert_object(active_ball.borrow_mut());
        self.shots -= 1;

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

